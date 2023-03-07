/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{material::Material, ray::Ray};

use cgmath::{InnerSpace, Vector3};

#[derive(Clone, Copy, Debug)]
pub(crate) struct HitRecord {
    pub(crate) point: Vector3<f32>,
    pub(crate) normal: Vector3<f32>,
    pub(crate) t: f32,
    pub(crate) front_face: bool,
    pub(crate) material: Material,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: Default::default(),
            front_face: Default::default(),
            material: Material::Lambertian {
                albedo: Vector3::new(0.0, 0.0, 0.0),
            },
        }
    }
}
