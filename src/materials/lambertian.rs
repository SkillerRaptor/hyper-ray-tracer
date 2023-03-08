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
pub(crate) struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub(crate) fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + math::random_unit_vector();
        if math::near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        Some((
            self.albedo
                .value(hit_record.u, hit_record.v, hit_record.point),
            Ray::new(hit_record.point, scatter_direction, ray.time()),
        ))
    }

    fn emitted(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
