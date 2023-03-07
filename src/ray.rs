/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::Vector3;

pub(crate) struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub(crate) fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub(crate) fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }

    pub(crate) fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub(crate) fn direction(&self) -> Vector3<f32> {
        self.direction
    }
}
