/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::{InnerSpace, Vector3};

use crate::{hit_record::HitRecord, ray::Ray};

#[derive(Clone, Debug)]
pub(crate) enum Hittable {
    Sphere { center: Vector3<f32>, radius: f32 },
    List { objects: Vec<Hittable> },
}

impl Hittable {
    pub(crate) fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        match *self {
            Hittable::Sphere { center, radius } => {
                let origin_center = ray.origin() - center;
                let a = ray.direction().dot(ray.direction());
                let half_b = origin_center.dot(ray.direction());
                let c = origin_center.dot(origin_center) - radius * radius;
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

                let outward_normal = (ray.at(root) - center) / radius;

                hit_record.t = root;
                hit_record.point = ray.at(hit_record.t);
                hit_record.normal = (hit_record.point - center) / radius;
                hit_record.set_face_normal(&ray, outward_normal);

                true
            }
            Hittable::List { ref objects } => {
                let mut temp_record = HitRecord::default();
                let mut hit_anything = false;
                let mut closest = t_max;

                for object in objects {
                    if object.hit(ray, t_min, closest, &mut temp_record) {
                        hit_anything = true;
                        closest = temp_record.t;
                        *hit_record = temp_record;
                    }
                }

                hit_anything
            }
        }
    }
}
