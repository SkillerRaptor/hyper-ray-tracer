/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, material::Material, ray::Ray};

use cgmath::{InnerSpace, Vector3};

#[derive(Clone, Debug)]
pub(crate) enum Hittable {
    Sphere {
        center: Vector3<f32>,
        radius: f32,
        material: Material,
    },
    MovingSphere {
        center_0: Vector3<f32>,
        center_1: Vector3<f32>,
        time_0: f32,
        time_1: f32,
        radius: f32,
        material: Material,
    },
    List {
        objects: Vec<Hittable>,
    },
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
            Hittable::Sphere {
                center,
                radius,
                material,
            } => {
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
                hit_record.material = material;
                hit_record.set_face_normal(&ray, outward_normal);

                true
            }
            Hittable::MovingSphere {
                center_0,
                center_1,
                time_0,
                time_1,
                radius,
                material,
            } => {
                let center = |time| -> Vector3<f32> {
                    center_0 + ((time - time_0) / (time_1 - time_0)) * (center_1 - center_0)
                };

                let origin_center = ray.origin() - center(ray.time());
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

                let outward_normal = (ray.at(root) - center(ray.time())) / radius;

                hit_record.t = root;
                hit_record.point = ray.at(hit_record.t);
                hit_record.material = material;
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
