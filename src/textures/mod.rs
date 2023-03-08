/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::math::Vec3;

pub(crate) mod checker_texture;
pub(crate) mod solid_color;

pub(crate) trait Texture: Sync {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}
