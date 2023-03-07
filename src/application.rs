/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::{InnerSpace, Vector2, Vector3, Vector4};
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};
use std::sync::mpsc::Receiver;

use crate::{
    hit_record::HitRecord, hittable::Hittable, hittable_list::HittableList, ray::Ray,
    sphere::Sphere,
};

pub(crate) struct Application {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    window_size: Vector2<i32>,
    texture_size: Vector2<i32>,
    screen_texture: u32,
    screen_framebuffer: u32,
    pixels: Vec<Vector4<f32>>,

    world: HittableList,
}

impl Application {
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

        let mut world = HittableList::new();
        world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(
            Vector3::new(0.0, -100.5, -1.0),
            100.0,
        )));

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
            world,
        };

        application.handle_resize(current_window_size.0, current_window_size.1);

        application
    }

    pub(crate) fn run(&mut self) {
        while !self.window.should_close() {
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

        self.pixels.resize(
            (self.texture_size.x * self.texture_size.y) as usize,
            Vector4::new(0.0, 0.0, 0.0, 0.0),
        );

        self.render();
    }

    fn write_pixel(&mut self, x: u32, y: u32, color: Vector4<f32>) {
        self.pixels[(y * self.texture_size.x as u32 + x) as usize] = color;
    }

    fn render(&mut self) {
        let aspect_ratio = self.texture_size.x as f32 / self.texture_size.y as f32;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

        for y in 0..self.texture_size.y as u32 {
            for x in 0..self.texture_size.x as u32 {
                let u = x as f32 / (self.texture_size.x as f32 - 1.0);
                let v = y as f32 / (self.texture_size.y as f32 - 1.0);

                let ray = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );
                let color = Self::ray_color(&ray, &self.world);

                self.write_pixel(x, y, Vector4::new(color.x, color.y, color.z, 1.0));
            }
        }
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vector3<f32> {
        let mut hit_record = HitRecord::default();
        if world.hit(ray, 0.0, f32::INFINITY, &mut hit_record) {
            return 0.5 * (hit_record.normal + Vector3::new(1.0, 1.0, 1.0));
        }

        let unit_direction = InnerSpace::normalize(ray.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}
