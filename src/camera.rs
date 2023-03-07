/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::Vector3;

use crate::ray::Ray;

pub(crate) struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

impl Camera {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let mut camera = Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(0.0, 0.0, 0.0),
            horizontal: Vector3::new(0.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 0.0, 0.0),
        };

        camera.resize(width, height);

        camera
    }

    pub(crate) fn resize(&mut self, width: i32, height: i32) {
        let aspect_ratio = width as f32 / height as f32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        self.origin = Vector3::new(0.0, 0.0, 0.0);
        self.horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        self.vertical = Vector3::new(0.0, viewport_height, 0.0);
        self.lower_left_corner = self.origin
            - self.horizontal / 2.0
            - self.vertical / 2.0
            - Vector3::new(0.0, 0.0, focal_length);
    }
    pub(crate) fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
