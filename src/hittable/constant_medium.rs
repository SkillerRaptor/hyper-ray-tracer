/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::f32::consts::E;

use cgmath::InnerSpace;
use rand::Rng;

use crate::{
    aabb::Aabb, hit_record::HitRecord, hittable::Hittable, materials::isotropic::Isotropic,
    math::Vec3, ray::Ray, textures::Texture,
};

pub(crate) struct ConstantMedium<T: Texture> {
    boundary: Box<dyn Hittable>,
    negative_inverse_density: f32,
    phase_function: Isotropic<T>,
}

impl<T: Texture> ConstantMedium<T> {
    pub(crate) fn new(boundary: Box<dyn Hittable>, density: f32, texture: T) -> Self {
        Self {
            boundary,
            negative_inverse_density: -1.0 / density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<T: Texture> Hittable for ConstantMedium<T> {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        // TODO: Implement for shapes with holes

        let mut record_1 = self.boundary.hit(ray, -f32::INFINITY, f32::INFINITY)?;
        let mut record_2 = self.boundary.hit(ray, record_1.t + 0.0001, f32::INFINITY)?;

        if record_1.t < time_min {
            record_1.t = time_min;
        }

        if record_2.t > time_max {
            record_2.t = time_max;
        }

        if record_1.t >= record_2.t {
            return None;
        }

        if record_1.t < 0.0 {
            record_1.t = 0.0;
        }

        let mut rand = rand::thread_rng();
        let ray_length = ray.direction().magnitude();
        let distance_inside_boundary = (record_2.t - record_1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * rand.gen::<f32>().log(E);

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = record_1.t + hit_distance / ray_length;

        Some(HitRecord {
            point: ray.at(t),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: &self.phase_function,
        })
    }

    fn bounding_box(&self, time_start: f32, time_end: f32) -> Option<Aabb> {
        self.boundary.bounding_box(time_start, time_end)
    }

    fn count(&self) -> u32 {
        self.boundary.count()
    }
}
