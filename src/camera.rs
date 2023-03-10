/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    math::{self, Vec3},
    ray::Ray,
};

use cgmath::InnerSpace;
use rand::Rng;

#[derive(Clone, Debug)]
pub(crate) struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    look_from: Vec3,
    look_at: Vec3,
    fov: f32,
    focus_dist: f32,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
}

impl Camera {
    pub(crate) fn new(
        look_from: Vec3,
        look_at: Vec3,
        fov: f32,
        aperture: f32,
        focus_dist: f32,
        time_0: f32,
        time_1: f32,
        width: i32,
        height: i32,
    ) -> Self {
        let mut camera = Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 0.0),
            look_from,
            look_at,
            fov,
            focus_dist,
            w: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            lens_radius: aperture / 2.0,
            time_0,
            time_1,
        };

        camera.resize(width, height);

        camera
    }

    pub(crate) fn resize(&mut self, width: i32, height: i32) {
        let aspect_ratio = width as f32 / height as f32;
        let theta = self.fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        self.w = (self.look_from - self.look_at).normalize();
        self.u = Vec3::new(0.0, 1.0, 0.0).cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        self.origin = self.look_from;
        self.horizontal = self.focus_dist * viewport_width * self.u;
        self.vertical = self.focus_dist * viewport_height * self.v;
        self.lower_left_corner =
            self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.focus_dist * self.w;
    }

    pub(crate) fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * math::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rand = rand::thread_rng();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rand.gen_range(self.time_0..self.time_1),
        )
    }
}
