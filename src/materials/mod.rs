/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, math::Vec3, ray::Ray};

pub(crate) mod dielectric;
pub(crate) mod diffuse_light;
pub(crate) mod isotropic;
pub(crate) mod lambertian;
pub(crate) mod metal;

pub(crate) trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;

    fn emitted(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}
