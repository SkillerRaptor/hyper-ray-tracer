/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::math::Vec3;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub(crate) fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub(crate) fn origin(&self) -> Vec3 {
        self.origin
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.direction
    }

    pub(crate) fn time(&self) -> f32 {
        self.time
    }
}
