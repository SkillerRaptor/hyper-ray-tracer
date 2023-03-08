/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::{InnerSpace, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

pub(crate) type Vec3 = Vector3<f32>;

pub(crate) fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub(crate) fn random_in_unit_sphere() -> Vec3 {
    let mut rand = rand::thread_rng();
    let range = Uniform::from(-1.0..1.0);
    loop {
        let point = Vec3::new(
            range.sample(&mut rand),
            range.sample(&mut rand),
            range.sample(&mut rand),
        );

        if point.dot(point) < 1.0 {
            return point;
        }
    }
}

pub(crate) fn random_in_unit_disk() -> Vec3 {
    let mut rand = rand::thread_rng();
    loop {
        let point = Vec3::new(rand.gen_range(-1.0..1.0), rand.gen_range(-1.0..1.0), 0.0);
        if point.dot(point) < 1.0 {
            return point;
        }
    }
}

pub(crate) fn near_zero(vector: Vec3) -> bool {
    const S: f32 = 1e-8;
    (vector.x.abs() < S) && (vector.y.abs() < S) && (vector.z.abs() < S)
}

pub(crate) fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub(crate) fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.dot(r_out_perp)).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub(crate) fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
