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

use cgmath::InnerSpace;

#[derive(Clone)]
pub(crate) struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub(crate) fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = math::reflect(ray.direction().normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * math::random_in_unit_sphere(),
            ray.time(),
        );

        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
