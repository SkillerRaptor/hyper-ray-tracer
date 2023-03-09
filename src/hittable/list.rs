/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{aabb::Aabb, hit_record::HitRecord, hittable::Hittable, ray::Ray};

pub(crate) struct List {
    objects: Vec<Box<dyn Hittable>>,
}

impl List {
    pub(crate) fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
}

impl Hittable for List {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        let mut closest = time_max;
        let mut hit_anything = None;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, time_min, closest) {
                closest = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time_start: f32, time_end: f32) -> Option<Aabb> {
        let object = self.objects.first()?;
        let first_aabb = object.bounding_box(time_start, time_end)?;

        self.objects
            .iter()
            .skip(1)
            .try_fold(first_aabb, |aabb, hitable| {
                let hittable_aabb = hitable.bounding_box(time_start, time_end)?;
                Some(Aabb::surrounding_box(aabb, hittable_aabb))
            })
    }

    fn count(&self) -> u32 {
        self.objects.iter().map(|object| object.count()).sum()
    }
}
