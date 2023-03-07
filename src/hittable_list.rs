/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, hittable::Hittable, ray::Ray};

pub(crate) struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}
