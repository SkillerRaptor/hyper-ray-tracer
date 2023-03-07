/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::Vector3;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
    time: f32,
}

impl Ray {
    pub(crate) fn new(origin: Vector3<f32>, direction: Vector3<f32>, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
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

    pub(crate) fn time(&self) -> f32 {
        self.time
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,
        }
    }
}
