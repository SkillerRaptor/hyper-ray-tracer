/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::{InnerSpace, Vector3};

use crate::{hit_record::HitRecord, hittable::Hittable, ray::Ray};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub(crate) fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let origin_center = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = origin_center.dot(ray.direction());
        let c = origin_center.dot(origin_center) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        hit_record.normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);

        true
    }
}
