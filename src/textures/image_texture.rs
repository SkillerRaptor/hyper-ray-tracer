/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{math::Vec3, textures::Texture};

#[derive(Clone)]
pub(crate) struct ImageTexture {
    data: Vec<u8>,
    components: u8,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    pub(crate) fn new(path: &str) -> Self {
        let image = image::open(path).unwrap();

        let channels = image.color().channel_count();
        let pixels = image.as_bytes();

        Self {
            data: pixels.to_vec(),
            components: channels,
            width: image.width(),
            height: image.height(),
            bytes_per_scanline: channels as u32 * image.width(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: Vec3) -> Vec3 {
        if self.data.is_empty() {
            return Vec3::new(1.0, 0.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.width as f32) as u32;
        let mut j = (v * self.height as f32) as u32;

        if i >= self.width {
            i = self.width - 1;
        }

        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let offset = (j * self.bytes_per_scanline + i * self.components as u32) as usize;

        Vec3::new(
            color_scale * self.data[offset + 0] as f32,
            color_scale * self.data[offset + 1] as f32,
            color_scale * self.data[offset + 2] as f32,
        )
    }
}
