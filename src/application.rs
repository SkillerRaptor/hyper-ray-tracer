/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    arguments::{Arguments, Scene},
    camera::Camera,
    hittable::{
        bvh_node::BvhNode,
        constant_medium::ConstantMedium,
        cuboid::Cuboid,
        moving_sphere::MovingSphere,
        rect::{Plane, Rect},
        rotation::{Axis, Rotation},
        sphere::Sphere,
        translation::Translation,
        Hittable,
    },
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    math::Vec3,
    ray::Ray,
    textures::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        solid_color::SolidColor,
    },
};

use cgmath::{ElementWise, InnerSpace, Vector2, Vector4};
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

    background: Vec3,
    samples: u32,
    depth: u32,
}

impl Application {
    pub(crate) fn new(arguments: Arguments) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                arguments.width,
                arguments.height,
                "Hyper-Ray-Tracer",
                glfw::WindowMode::Windowed,
            )
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

        let look_from;
        let look_at;
        let fov;
        let aperture;
        let background;

        log::info!("Generating world...");
        let world = match arguments.scene {
            Scene::Random => {
                look_from = Vec3::new(13.0, 2.0, 3.0);
                look_at = Vec3::new(0.0, 0.0, 0.0);
                fov = 20.0;
                aperture = 0.1;
                background = Vec3::new(0.7, 0.8, 1.0);
                Self::generate_random_scene()
            }
            Scene::TwoSpheres => {
                look_from = Vec3::new(13.0, 2.0, 3.0);
                look_at = Vec3::new(0.0, 0.0, 0.0);
                fov = 20.0;
                aperture = 0.0;
                background = Vec3::new(0.7, 0.8, 1.0);
                Self::generate_two_spheres()
            }
            Scene::TwoPerlinSpheres => {
                look_from = Vec3::new(13.0, 2.0, 3.0);
                look_at = Vec3::new(0.0, 0.0, 0.0);
                fov = 20.0;
                aperture = 0.0;
                background = Vec3::new(0.7, 0.8, 1.0);
                Self::generate_two_perlin_spheres()
            }
            Scene::Earth => {
                look_from = Vec3::new(13.0, 2.0, 3.0);
                look_at = Vec3::new(0.0, 0.0, 0.0);
                fov = 20.0;
                aperture = 0.0;
                background = Vec3::new(0.7, 0.8, 1.0);
                Self::generate_earth()
            }
            Scene::SimpleLight => {
                look_from = Vec3::new(26.0, 3.0, 6.0);
                look_at = Vec3::new(0.0, 2.0, 0.0);
                fov = 20.0;
                aperture = 0.0;
                background = Vec3::new(0.0, 0.0, 0.0);
                Self::generate_simple_light()
            }
            Scene::Cornell => {
                look_from = Vec3::new(278.0, 278.0, -800.0);
                look_at = Vec3::new(278.0, 278.0, 0.0);
                fov = 40.0;
                aperture = 0.0;
                background = Vec3::new(0.0, 0.0, 0.0);
                Self::generate_cornell_box()
            }
            Scene::CornellSmoke => {
                look_from = Vec3::new(278.0, 278.0, -800.0);
                look_at = Vec3::new(278.0, 278.0, 0.0);
                fov = 40.0;
                aperture = 0.0;
                background = Vec3::new(0.0, 0.0, 0.0);
                Self::generate_cornell_smoke_box()
            }
            Scene::Final => {
                look_from = Vec3::new(478.0, 278.0, -600.0);
                look_at = Vec3::new(278.0, 278.0, 0.0);
                fov = 40.0;
                aperture = 0.0;
                background = Vec3::new(0.0, 0.0, 0.0);
                Self::generate_final_scene()
            }
        };

        log::info!("Generated world");

        let camera = Camera::new(
            look_from,
            look_at,
            fov,
            aperture,
            10.0,
            0.0,
            1.0,
            current_window_size.0,
            current_window_size.1,
        );

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
            background,
            samples: arguments.samples,
            depth: arguments.depth,
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

        log::info!("Rendering image...");

        let start = Instant::now();
        self.render();
        let duration = start.elapsed();

        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;

        log::info!(
            "Rendered image in {:02}:{:02}m! ({:?})",
            minutes,
            seconds,
            duration
        );
        log::info!("Image info:");
        log::info!("  Width: {}", self.texture_size.x);
        log::info!("  Height: {}", self.texture_size.y);
        log::info!("  Samples: {}", self.samples);
        log::info!("  Depth: {}", self.depth);
        log::info!("  Objects: {}", self.world.count());
    }

    fn render(&mut self) {
        let scale = 1.0 / self.samples as f32;

        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let mut rand = rand::thread_rng();
                let x = index % self.texture_size.x as usize;
                let y = index / self.texture_size.x as usize;
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples {
                    let u = (x as f32 + rand.gen::<f32>()) / (self.texture_size.x as f32 - 1.0);
                    let v = (y as f32 + rand.gen::<f32>()) / (self.texture_size.y as f32 - 1.0);

                    let ray = self.camera.get_ray(u, v);
                    pixel_color += Self::ray_color(&ray, self.background, &self.world, self.depth);
                }

                pixel_color.x = (pixel_color.x * scale).sqrt();
                pixel_color.y = (pixel_color.y * scale).sqrt();
                pixel_color.z = (pixel_color.z * scale).sqrt();

                *pixel = Vector4::new(pixel_color.x, pixel_color.y, pixel_color.z, 1.0);
            });
    }

    fn ray_color(ray: &Ray, background: Vec3, world: &Box<dyn Hittable>, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) else {
            return background
        };

        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, hit_record.point);
        let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) else {
            return emitted
        };

        let ray_color = Self::ray_color(&scattered, background, world, depth - 1);
        attenuation.mul_element_wise(ray_color) + emitted
    }

    fn generate_random_scene() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(CheckerTexture::new(
                SolidColor::new(Vec3::new(0.2, 0.3, 0.1)),
                SolidColor::new(Vec3::new(0.9, 0.9, 0.9)),
            )),
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
                            Lambertian::new(SolidColor::new(albedo)),
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
            Lambertian::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        objects.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        )));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_two_spheres() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let checker = Lambertian::new(CheckerTexture::new(
            SolidColor::new(Vec3::new(0.2, 0.3, 0.1)),
            SolidColor::new(Vec3::new(0.9, 0.9, 0.9)),
        ));

        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            checker.clone(),
        )));
        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            checker,
        )));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_two_perlin_spheres() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let noise = Lambertian::new(NoiseTexture::new(4.0));

        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            noise.clone(),
        )));
        objects.push(Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, noise)));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_earth() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let earth = Lambertian::new(ImageTexture::new("./assets/earthmap.jpg"));

        objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth)));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_simple_light() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let noise = Lambertian::new(NoiseTexture::new(4.0));
        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            noise.clone(),
        )));
        objects.push(Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, noise)));

        let diffuse_light = DiffuseLight::new(SolidColor::new(Vec3::new(4.0, 4.0, 4.0)));
        objects.push(Box::new(Rect::new(
            Plane::XY,
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            diffuse_light,
        )));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_cornell_box() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let red = Lambertian::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lambertian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Lambertian::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(SolidColor::new(Vec3::new(15.0, 15.0, 15.0)));

        objects.push(Box::new(Rect::new(
            Plane::YZ,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            green,
        )));
        objects.push(Box::new(Rect::new(
            Plane::YZ,
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            red,
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            light,
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::XY,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));

        let mut cuboid_1: Box<dyn Hittable> = Box::new(Cuboid::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            white.clone(),
        ));
        cuboid_1 = Box::new(Rotation::new(Axis::Y, cuboid_1, 15.0));
        cuboid_1 = Box::new(Translation::new(cuboid_1, Vec3::new(265.0, 0.0, 295.0)));
        objects.push(cuboid_1);

        let mut cuboid_2: Box<dyn Hittable> = Box::new(Cuboid::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            white.clone(),
        ));
        cuboid_2 = Box::new(Rotation::new(Axis::Y, cuboid_2, -18.0));
        cuboid_2 = Box::new(Translation::new(cuboid_2, Vec3::new(130.0, 0.0, 65.0)));
        objects.push(cuboid_2);

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_cornell_smoke_box() -> Box<dyn Hittable> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        let red = Lambertian::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lambertian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Lambertian::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(SolidColor::new(Vec3::new(15.0, 15.0, 15.0)));

        objects.push(Box::new(Rect::new(
            Plane::YZ,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            green,
        )));
        objects.push(Box::new(Rect::new(
            Plane::YZ,
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            red,
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            light,
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::XY,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));

        let mut cuboid_1: Box<dyn Hittable> = Box::new(Cuboid::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            white.clone(),
        ));
        cuboid_1 = Box::new(Rotation::new(Axis::Y, cuboid_1, 15.0));
        cuboid_1 = Box::new(Translation::new(cuboid_1, Vec3::new(265.0, 0.0, 295.0)));
        cuboid_1 = Box::new(ConstantMedium::new(
            cuboid_1,
            0.01,
            SolidColor::new(Vec3::new(0.0, 0.0, 0.0)),
        ));
        objects.push(cuboid_1);

        let mut cuboid_2: Box<dyn Hittable> = Box::new(Cuboid::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            white.clone(),
        ));
        cuboid_2 = Box::new(Rotation::new(Axis::Y, cuboid_2, -18.0));
        cuboid_2 = Box::new(Translation::new(cuboid_2, Vec3::new(130.0, 0.0, 65.0)));
        cuboid_2 = Box::new(ConstantMedium::new(
            cuboid_2,
            0.01,
            SolidColor::new(Vec3::new(1.0, 1.0, 1.0)),
        ));
        objects.push(cuboid_2);

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }

    fn generate_final_scene() -> Box<dyn Hittable> {
        const BOXES_PER_SIDE: usize = 20;

        let mut rand = rand::thread_rng();

        let ground_material = Lambertian::new(SolidColor::new(Vec3::new(0.48, 0.83, 0.53)));
        let mut ground_boxes: Vec<Box<dyn Hittable>> = Vec::new();
        for i in 0..BOXES_PER_SIDE {
            for j in 0..BOXES_PER_SIDE {
                let w = 100.0;
                let x0 = -1000.0 + i as f32 * w;
                let z0 = -1000.0 + j as f32 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = rand.gen_range(1.0..101.0);
                let z1 = z0 + w;

                ground_boxes.push(Box::new(Cuboid::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    ground_material.clone(),
                )));
            }
        }

        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        objects.push(Box::new(BvhNode::new(ground_boxes, 0.0, 1.0)));

        let diffuse_light = DiffuseLight::new(SolidColor::new(Vec3::new(7.0, 7.0, 7.0)));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            123.0,
            423.0,
            147.0,
            412.0,
            554.0,
            diffuse_light,
        )));

        let center_1 = Vec3::new(400.0, 400.0, 200.0);
        let center_2 = center_1 + Vec3::new(30.0, 0.0, 0.0);

        let moving_sphere_material = Lambertian::new(SolidColor::new(Vec3::new(0.7, 0.3, 0.1)));
        objects.push(Box::new(MovingSphere::new(
            center_1,
            center_2,
            0.0,
            1.0,
            50.0,
            moving_sphere_material,
        )));

        objects.push(Box::new(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            Dielectric::new(1.5),
        )));

        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0),
        )));

        let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
        objects.push(Box::new(boundary.clone()));
        objects.push(Box::new(ConstantMedium::new(
            Box::new(boundary),
            0.2,
            SolidColor::new(Vec3::new(0.2, 0.4, 0.9)),
        )));

        let boundary = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
        objects.push(Box::new(ConstantMedium::new(
            Box::new(boundary),
            0.0001,
            SolidColor::new(Vec3::new(1.0, 1.0, 1.0)),
        )));

        let earth_map = Lambertian::new(ImageTexture::new("./assets/earthmap.jpg"));
        objects.push(Box::new(Sphere::new(
            Vec3::new(400.0, 200.0, 400.0),
            100.0,
            earth_map,
        )));

        let noise = Lambertian::new(NoiseTexture::new(0.1));
        objects.push(Box::new(Sphere::new(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            noise,
        )));

        let white = Lambertian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        let mut sphere_box: Vec<Box<dyn Hittable>> = Vec::new();
        for _ in 0..1000 {
            sphere_box.push(Box::new(Sphere::new(
                Vec3::new(
                    rand.gen_range(0.0..165.0),
                    rand.gen_range(0.0..165.0),
                    rand.gen_range(0.0..165.0),
                ),
                10.0,
                white.clone(),
            )));
        }

        objects.push(Box::new(Translation::new(
            Box::new(Rotation::new(
                Axis::Y,
                Box::new(BvhNode::new(sphere_box, 0.0, 1.0)),
                15.0,
            )),
            Vec3::new(-100.0, 270.0, 395.0),
        )));

        Box::new(BvhNode::new(objects, 0.0, 1.0))
    }
}
