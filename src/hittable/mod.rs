/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{aabb::Aabb, hit_record::HitRecord, ray::Ray};

pub(crate) mod bvh_node;
pub(crate) mod list;
pub(crate) mod moving_sphere;
pub(crate) mod sphere;

pub(crate) trait Hittable: Sync {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self, time_start: f32, time_end: f32) -> Option<Aabb>;

    fn count(&self) -> u32;
}
