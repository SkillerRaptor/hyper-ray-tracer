/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{camera::Camera, hit_record::HitRecord, hittable::Hittable, ray::Ray};

use cgmath::{InnerSpace, Vector2, Vector3, Vector4};
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use std::{io::Write, sync::mpsc::Receiver, time::Instant};

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
    world: Hittable,
}

impl Application {
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

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

        let mut objects = Vec::new();
        objects.push(Hittable::Sphere {
            center: Vector3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        });
        objects.push(Hittable::Sphere {
            center: Vector3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        });

        let current_window_size = window.get_size();

        let mut application = Self {
            glfw,
            window,
            events,
            window_size: Vector2::new(0, 0),
            texture_size: Vector2::new(0, 0),
            screen_texture,
            screen_framebuffer,
            pixels: Vec::new(),
            camera: Camera::new(current_window_size.0, current_window_size.1),
            world: Hittable::List { objects },
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
        println!("Rendered frame in {:?}", duration);
    }

    fn write_pixel(&mut self, x: u32, y: u32, color: Vector4<f32>) {
        self.pixels[(y * self.texture_size.x as u32 + x) as usize] = color;
    }

    fn render(&mut self) {
        let scale = 1.0 / Self::SAMPLES_PER_PIXEL as f32;
        let mut rand = rand::thread_rng();

        for y in 0..self.texture_size.y as u32 {
            for x in 0..self.texture_size.x as u32 {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);

                for _ in 0..Self::SAMPLES_PER_PIXEL {
                    let u = (x as f32 + rand.gen::<f32>()) / (self.texture_size.x as f32 - 1.0);
                    let v = (y as f32 + rand.gen::<f32>()) / (self.texture_size.y as f32 - 1.0);

                    let ray = self.camera.get_ray(u, v);
                    pixel_color += Self::ray_color(&ray, &self.world, Self::MAX_DEPTH);
                }

                pixel_color.x = (pixel_color.x * scale).sqrt();
                pixel_color.y = (pixel_color.y * scale).sqrt();
                pixel_color.z = (pixel_color.z * scale).sqrt();

                self.write_pixel(
                    x,
                    y,
                    Vector4::new(pixel_color.x, pixel_color.y, pixel_color.z, 1.0),
                );
            }
        }
    }

    fn ray_color(ray: &Ray, world: &Hittable, depth: u32) -> Vector3<f32> {
        if depth == 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let mut hit_record = HitRecord::default();
        if world.hit(ray, 0.001, f32::INFINITY, &mut hit_record) {
            let target = hit_record.point + hit_record.normal + Self::random_unit_vector();
            return 0.5
                * Self::ray_color(
                    &Ray::new(hit_record.point, target - hit_record.point),
                    world,
                    depth - 1,
                );
        }

        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }

    fn random_unit_vector() -> Vector3<f32> {
        Self::random_in_unit_sphere().normalize()
    }

    fn random_in_unit_sphere() -> Vector3<f32> {
        let mut rand = rand::thread_rng();
        let range = Uniform::from(-1.0..1.0);
        loop {
            let point = Vector3::new(
                range.sample(&mut rand),
                range.sample(&mut rand),
                range.sample(&mut rand),
            );

            if point.dot(point) < 1.0 {
                return point;
            }
        }
    }
}
