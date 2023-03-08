/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{math::Vec3, perlin_noise::PerlinNoise, textures::Texture};

#[derive(Clone)]
pub(crate) struct NoiseTexture {
    noise: PerlinNoise,
    scale: f32,
}

impl NoiseTexture {
    pub(crate) fn new(scale: f32) -> Self {
        Self {
            noise: PerlinNoise::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, point: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + ((self.scale * point.z) + (10.0 * self.noise.turbulence(self.scale * point, 7)))
                    .sin())
    }
}
