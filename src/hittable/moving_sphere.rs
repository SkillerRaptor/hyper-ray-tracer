/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::InnerSpace;

use crate::{
    aabb::Aabb, hit_record::HitRecord, hittable::Hittable, material::Material, math::Vec3, ray::Ray,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct MovingSphere {
    center_start: Vec3,
    center_end: Vec3,
    time_start: f32,
    time_end: f32,
    radius: f32,
    material: Material,
}

impl MovingSphere {
    pub(crate) fn new(
        center_start: Vec3,
        center_end: Vec3,
        time_start: f32,
        time_end: f32,
        radius: f32,
        material: Material,
    ) -> Self {
        Self {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            material,
        }
    }
}

impl MovingSphere {
    fn center(&self, time: f32) -> Vec3 {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        let origin_center = ray.origin() - self.center(ray.time());
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
            material: self.material,
        };

        let outward_normal = (ray.at(root) - self.center(ray.time())) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f32, time_end: f32) -> Option<Aabb> {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        let box_0 = Aabb::new(
            self.center(time_start) - radius_vector,
            self.center(time_start) + radius_vector,
        );
        let box_1 = Aabb::new(
            self.center(time_end) - radius_vector,
            self.center(time_end) + radius_vector,
        );

        Some(Aabb::surrounding_box(box_0, box_1))
    }
}
