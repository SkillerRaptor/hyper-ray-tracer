/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    camera::Camera,
    hittable::{bvh_node::BvhNode, moving_sphere::MovingSphere, sphere::Sphere, Hittable},
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    math::Vec3,
    ray::Ray,
};

use cgmath::{InnerSpace, Vector2, Vector4};
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use rand::Rng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::{sync::mpsc::Receiver, time::Instant};

pub(crate) struct Application {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    window_size: Vector2<i32>,
    texture_size: Vector2<i32>,
    screen_texture: u32,
    screen_framebuffer: u32,
    pixels: Vec<Vector4<f32>>,

    camera: Camera,
    world: Box<dyn Hittable>,
}

impl Application {
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 10;

    pub(crate) fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(1280, 720, "Hyper-Ray-Tracer", glfw::WindowMode::Windowed)
            .unwrap();

        window.make_current();
        window.set_all_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mut screen_texture = 0u32;
        unsafe {
            gl::GenTextures(1, &mut screen_texture as *mut u32);
            gl::BindTexture(gl::TEXTURE_2D, screen_texture);
        };

        let mut screen_framebuffer = 0u32;
        unsafe {
            gl::GenFramebuffers(1, &mut screen_framebuffer as *mut u32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, screen_framebuffer);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                screen_texture,
                0,
            );
        }

        let current_window_size = window.get_size();

        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);

        let camera = Camera::new(
            look_from,
            look_at,
            20.0,
            0.1,
            10.0,
            0.0,
            1.0,
            current_window_size.0,
            current_window_size.1,
        );

        let world = Self::generate_random_scene();

        let mut application = Self {
            glfw,
            window,
            events,
            window_size: Vector2::new(0, 0),
            texture_size: Vector2::new(0, 0),
            screen_texture,
            screen_framebuffer,
            pixels: Vec::new(),
            camera,
            world,
        };

        application.handle_resize(current_window_size.0, current_window_size.1);

        application
    }

    pub(crate) fn run(&mut self) {
        let mut last_frame = Instant::now();
        while !self.window.should_close() {
            let current_frame = Instant::now();
            let delta_time = current_frame - last_frame;
            last_frame = current_frame;

            self.window.set_title(&format!(
                "Hyper-Ray-Tracer ({:.0} fps / {:.2})",
                1.0 / delta_time.as_secs_f32(),
                delta_time.as_secs_f32()
            ));

            self.process_events();

            unsafe {
                let data = std::mem::transmute(self.pixels.as_ptr());

                gl::BindTexture(gl::TEXTURE_2D, self.screen_texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA32F as i32,
                    self.texture_size.x,
                    self.texture_size.y,
                    0,
                    gl::RGBA,
                    gl::FLOAT,
                    data,
                );

                gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.screen_framebuffer);
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
                gl::BlitFramebuffer(
                    0,
                    0,
                    self.texture_size.x,
                    self.texture_size.y,
                    0,
                    0,
                    self.window_size.x,
                    self.window_size.y,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                )
            }

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn process_events(&mut self) {
        let mut new_size = None;
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }

                    new_size = Some(Vector2::new(width, height));
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }

        if let Some(new_size) = new_size {
            self.handle_resize(new_size.x, new_size.y);
        }
    }

    fn handle_resize(&mut self, width: i32, height: i32) {
        self.window_size = Vector2::new(width, height);
        self.texture_size = Vector2::new(width, height);

        self.camera.resize(width, height);

        self.pixels.resize(
            (self.texture_size.x * self.texture_size.y) as usize,
            Vector4::new(0.0, 0.0, 0.0, 0.0),
        );

        let start = Instant::now();

        self.render();

        let duration = start.elapsed();

        println!(
            "Rendered image in {:?} with a size of {}x{} and {} samples per pixel with {} as maximum depth",
            duration, self.texture_size.x, self.texture_size.y, Self::SAMPLES_PER_PIXEL, Self::MAX_DEPTH
        );
    }

    fn render(&mut self) {
        let scale = 1.0 / Self::SAMPLES_PER_PIXEL as f32;

        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let mut rand = rand::thread_rng();
                let x = index % self.texture_size.x as usize;
                let y = index / self.texture_size.x as usize;
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..Self::SAMPLES_PER_PIXEL {
                    let u = (x as f32 + rand.gen::<f32>()) / (self.texture_size.x as f32 - 1.0);
                    let v = (y as f32 + rand.gen::<f32>()) / (self.texture_size.y as f32 - 1.0);

                    let ray = self.camera.get_ray(u, v);
                    pixel_color += Self::ray_color(&ray, &self.world, Self::MAX_DEPTH);
                }

                pixel_color.x = (pixel_color.x * scale).sqrt();
                pixel_color.y = (pixel_color.y * scale).sqrt();
                pixel_color.z = (pixel_color.z * scale).sqrt();

                *pixel = Vector4::new(pixel_color.x, pixel_color.y, pixel_color.z, 1.0);
            });
    }

    fn ray_color(ray: &Ray, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
                let ray_color = Self::ray_color(&scattered, world, depth - 1);
                return Vec3::new(
                    ray_color.x * attenuation.x,
                    ray_color.y * attenuation.y,
                    ray_color.z * attenuation.z,
                );
            }

            return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }

    fn generate_random_scene() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        let mut rand = rand::thread_rng();

        for a in -11..11 {
            for b in -11..11 {
                let choose_material = rand.gen::<f32>();

                let center = Vec3::new(
                    a as f32 + 0.9 * rand.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rand.gen::<f32>(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    if choose_material < 0.8 {
                        let albedo = Vec3::new(rand.gen(), rand.gen(), rand.gen());
                        let center_2 = center + Vec3::new(0.0, rand.gen_range(0.0..0.5), 0.0);
                        objects.push(Box::new(MovingSphere::new(
                            center,
                            center_2,
                            0.0,
                            1.0,
                            0.2,
                            Lambertian::new(albedo),
                        )));
                    } else if choose_material < 0.95 {
                        let albedo = Vec3::new(
                            rand.gen_range(0.5..1.0),
                            rand.gen_range(0.5..1.0),
                            rand.gen_range(0.5..1.0),
                        );
                        let fuzz = rand.gen_range(0.0..0.5);
                        objects.push(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                    } else {
                        objects.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                    };
                }
            }
        }

        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(1.5),
        )));
        objects.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
        )));
        objects.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        )));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }
}
