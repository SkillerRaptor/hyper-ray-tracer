/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    aabb::Aabb, hit_record::HitRecord, hittable::Hittable, materials::Material, math::Vec3,
    ray::Ray,
};

#[derive(Clone, Copy, Debug)]
pub enum Plane {
    XY,
    YZ,
    ZX,
}

#[derive(Clone)]
pub(crate) struct Rect<M: Material> {
    plane: Plane,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: M,
}

impl<M: Material> Rect<M> {
    pub(crate) fn new(
        plane: Plane,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: M,
    ) -> Self {
        Self {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for Rect<M> {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::XY => (2, 0, 1),
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
        };

        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < time_min || t > time_max {
            return None;
        }

        let a = ray.origin()[a_axis] + t * ray.direction()[a_axis];
        let b = ray.origin()[b_axis] + t * ray.direction()[b_axis];
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let mut hit_record = HitRecord {
            point: ray.at(t),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            u: (a - self.a0) / (self.a1 - self.a0),
            v: (b - self.b0) / (self.b1 - self.b0),
            front_face: false,
            material: &self.material,
        };

        let mut outward_normal = Vec3::new(0.0, 0.0, 0.0);
        outward_normal[k_axis] = 1.0;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _time_start: f32, _time_end: f32) -> Option<Aabb> {
        Some(match self.plane {
            Plane::XY => Aabb::new(
                Vec3::new(self.a0, self.b0, self.k - 0.0001),
                Vec3::new(self.a1, self.b1, self.k + 0.0001),
            ),
            Plane::YZ => Aabb::new(
                Vec3::new(self.k - 0.0001, self.a0, self.b0),
                Vec3::new(self.k + 0.0001, self.a1, self.b1),
            ),
            Plane::ZX => Aabb::new(
                Vec3::new(self.a0, self.k - 0.0001, self.b0),
                Vec3::new(self.a1, self.k + 0.0001, self.b1),
            ),
        })
    }

    fn count(&self) -> u32 {
        1
    }
}
