/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::math::Vec3;

use cgmath::InnerSpace;
use rand::Rng;

#[derive(Clone, Debug)]
pub(crate) struct PerlinNoise {
    random_vectors: [Vec3; Self::POINT_COUNT],
    permutation_x: [u32; Self::POINT_COUNT],
    permutation_y: [u32; Self::POINT_COUNT],
    permutation_z: [u32; Self::POINT_COUNT],
}

impl PerlinNoise {
    const POINT_COUNT: usize = 256;

    pub(crate) fn new() -> Self {
        let mut rand = rand::thread_rng();

        let mut random_vectors = [Vec3::new(0.0, 0.0, 0.0); Self::POINT_COUNT];
        random_vectors.iter_mut().for_each(|vector| {
            *vector = Vec3::new(
                rand.gen_range(-1.0..1.0),
                rand.gen_range(-1.0..1.0),
                rand.gen_range(-1.0..1.0),
            )
            .normalize();
        });

        Self {
            random_vectors,
            permutation_x: Self::generate_permuation(),
            permutation_y: Self::generate_permuation(),
            permutation_z: Self::generate_permuation(),
        }
    }

    fn generate_permuation() -> [u32; Self::POINT_COUNT] {
        let mut permutation = [0; Self::POINT_COUNT];
        permutation
            .iter_mut()
            .enumerate()
            .for_each(|(i, permutation)| {
                *permutation = i as u32;
            });

        Self::permute(&mut permutation);

        permutation
    }

    fn permute(permutation: &mut [u32; Self::POINT_COUNT]) {
        let mut rand = rand::thread_rng();
        for i in (1..Self::POINT_COUNT).rev() {
            let target = rand.gen_range(0..i);
            permutation.swap(i, target);
        }
    }

    pub(crate) fn turbulence(&self, point: Vec3, depth: u32) -> f32 {
        let mut accumulator = 0.0;
        let mut point = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulator += weight * self.noise(point);
            weight *= 0.5;
            point *= 2.0;
        }

        accumulator.abs()
    }

    pub(crate) fn noise(&self, point: Vec3) -> f32 {
        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for index in 0..(2 * 2 * 2) {
            let i_x = index / (2 * 2);
            let i_y = (index / 2) % 2;
            let i_z = index % 2;

            let x = self.permutation_x[((i + i_x) & (Self::POINT_COUNT - 1) as i32) as usize];
            let y = self.permutation_y[((j + i_y) & (Self::POINT_COUNT - 1) as i32) as usize];
            let z = self.permutation_z[((k + i_z) & (Self::POINT_COUNT - 1) as i32) as usize];
            c[i_x as usize][i_y as usize][i_z as usize] = self.random_vectors[(x ^ y ^ z) as usize];
        }

        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();
        Self::trilinear_interpolation(c, u, v, w)
    }

    fn trilinear_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let mut accumulator = 0.0;
        for i in 0..(2 * 2 * 2) {
            let x = i / (2 * 2);
            let y = (i / 2) % 2;
            let z = i % 2;

            let weight = Vec3::new(u - x as f32, v - y as f32, w - z as f32);
            accumulator += (x as f32 * u + (1 - x) as f32 * (1.0 - u))
                * (y as f32 * v + (1 - y) as f32 * (1.0 - v))
                * (z as f32 * w + (1 - z) as f32 * (1.0 - w))
                * c[x][y][z].dot(weight)
        }

        accumulator
    }
}
