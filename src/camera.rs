/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use crate::ray::Ray;

pub(crate) struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    look_from: Vector3<f32>,
    look_at: Vector3<f32>,
    fov: f32,
    focus_dist: f32,
    w: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    lens_radius: f32,
}

impl Camera {
    pub(crate) fn new(
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        fov: f32,
        aperture: f32,
        focus_dist: f32,
        width: i32,
        height: i32,
    ) -> Self {
        let mut camera = Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(0.0, 0.0, 0.0),
            horizontal: Vector3::new(0.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 0.0, 0.0),
            look_from,
            look_at,
            fov,
            focus_dist,
            w: Vector3::new(0.0, 0.0, 0.0),
            u: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 0.0),
            lens_radius: aperture / 2.0,
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
        self.u = Vector3::new(0.0, 1.0, 0.0).cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        self.origin = self.look_from;
        self.horizontal = self.focus_dist * viewport_width * self.u;
        self.vertical = self.focus_dist * viewport_height * self.v;
        self.lower_left_corner =
            self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.focus_dist * self.w;
    }

    pub(crate) fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        );
    }

    fn random_in_unit_disk() -> Vector3<f32> {
        let mut rand = rand::thread_rng();
        loop {
            let point = Vector3::new(rand.gen_range(-1.0..1.0), rand.gen_range(-1.0..1.0), 0.0);
            if point.dot(point) < 1.0 {
                return point;
            }
        }
    }
}
