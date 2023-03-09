/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{aabb::Aabb, hit_record::HitRecord, hittable::Hittable, math::Vec3, ray::Ray};

pub(crate) struct Translation {
    hittable: Box<dyn Hittable>,
    displacement: Vec3,
}

impl Translation {
    pub(crate) fn new(hittable: Box<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            hittable,
            displacement,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(
            ray.origin() - self.displacement,
            ray.direction(),
            ray.time(),
        );

        let Some( mut hit_record) = self.hittable.hit(&moved_ray, time_min, time_max) else {
            return None;
        };

        hit_record.point += self.displacement;
        hit_record.set_face_normal(&moved_ray, hit_record.normal);

        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f32, time_end: f32) -> Option<Aabb> {
        let Some(mut output_box) = self.hittable.bounding_box(time_start, time_end) else {
            return None;
        };

        output_box = Aabb::new(
            output_box.minimum() + self.displacement,
            output_box.maximum() + self.displacement,
        );

        Some(output_box)
    }

    fn count(&self) -> u32 {
        self.hittable.count()
    }
}
