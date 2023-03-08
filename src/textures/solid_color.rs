/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{math::Vec3, textures::Texture};

#[derive(Clone)]
pub(crate) struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub(crate) fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        self.color
    }
}
