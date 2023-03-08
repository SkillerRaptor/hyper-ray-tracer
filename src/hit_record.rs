/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{materials::Material, math::Vec3, ray::Ray};

use cgmath::InnerSpace;

pub(crate) struct HitRecord<'a> {
    pub(crate) point: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) t: f32,
    pub(crate) u: f32,
    pub(crate) v: f32,
    pub(crate) front_face: bool,
    pub(crate) material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub(crate) fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
