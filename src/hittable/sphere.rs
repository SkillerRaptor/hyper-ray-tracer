/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::InnerSpace;

use crate::{
    aabb::Aabb, hit_record::HitRecord, hittable::Hittable, materials::Material, math::Vec3,
    ray::Ray,
};

#[derive(Clone)]
pub(crate) struct Sphere<M>
where
    M: Material,
{
    center: Vec3,
    radius: f32,
    material: M,
}

impl<M> Sphere<M>
where
    M: Material,
{
    pub(crate) fn new(center: Vec3, radius: f32, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M> Hittable for Sphere<M>
where
    M: Material,
{
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        let origin_center = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = origin_center.dot(ray.direction());
        let c = origin_center.dot(origin_center) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < time_min || time_max < root {
            root = (-half_b + sqrtd) / a;
            if root < time_min || time_max < root {
                return None;
            }
        }

        let mut hit_record = HitRecord {
            point: ray.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (ray.at(root) - self.center) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _time_start: f32, _time_end: f32) -> Option<Aabb> {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        Some(Aabb::new(
            self.center - radius_vector,
            self.center + radius_vector,
        ))
    }
}
