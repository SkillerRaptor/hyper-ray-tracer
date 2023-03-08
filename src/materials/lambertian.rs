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
};

#[derive(Clone)]
pub(crate) struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub(crate) fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + math::random_unit_vector();
        if math::near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        Some((
            self.albedo,
            Ray::new(hit_record.point, scatter_direction, ray.time()),
        ))
    }
}
