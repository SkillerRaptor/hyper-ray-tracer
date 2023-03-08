/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{math::Vec3, textures::Texture};

#[derive(Clone)]
pub(crate) struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub(crate) fn new(odd: T, even: U) -> Self {
        Self { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
