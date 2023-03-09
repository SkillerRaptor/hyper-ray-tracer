/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    hit_record::HitRecord,
    materials::Material,
    math::{self, Vec3},
    ray::Ray,
    textures::Texture,
};

#[derive(Clone)]
pub(crate) struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub(crate) fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        Some((
            self.albedo
                .value(hit_record.u, hit_record.v, hit_record.point),
            Ray::new(hit_record.point, math::random_in_unit_sphere(), ray.time()),
        ))
    }

    fn emitted(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
