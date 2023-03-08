/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, math::Vec3, ray::Ray};

pub(crate) mod dielectric;
pub(crate) mod lambertian;
pub(crate) mod metal;

pub(crate) trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}
