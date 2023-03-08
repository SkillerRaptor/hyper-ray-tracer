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
use rand::Rng;

#[derive(Clone)]
pub(crate) struct Dielectric {
    index_of_referaction: f32,
}

impl Dielectric {
    pub(crate) fn new(index_of_referaction: f32) -> Self {
        Self {
            index_of_referaction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_referaction
        } else {
            self.index_of_referaction
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let mut rand = rand::thread_rng();
        let direction =
            if cannot_refract || math::reflectance(cos_theta, refraction_ratio) > rand.gen() {
                math::reflect(unit_direction, hit_record.normal)
            } else {
                math::refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        Some((
            Vec3::new(1.0, 1.0, 1.0),
            Ray::new(hit_record.point, direction, ray.time()),
        ))
    }

    fn emitted(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
