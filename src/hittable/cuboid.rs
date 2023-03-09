/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::marker::PhantomData;

use crate::{
    aabb::Aabb,
    hit_record::HitRecord,
    hittable::{
        list::List,
        rect::{Plane, Rect},
        Hittable,
    },
    materials::Material,
    math::Vec3,
    ray::Ray,
};

pub(crate) struct Cuboid<M: Clone + Material> {
    box_min: Vec3,
    box_max: Vec3,
    sides: Box<List>,
    _phantom: PhantomData<M>,
}

impl<M: Clone + Material + 'static> Cuboid<M> {
    pub(crate) fn new(box_min: Vec3, box_max: Vec3, material: M) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

        objects.push(Box::new(Rect::new(
            Plane::XY,
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_max.z,
            material.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::XY,
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_min.z,
            material.clone(),
        )));

        objects.push(Box::new(Rect::new(
            Plane::ZX,
            box_min.z,
            box_max.z,
            box_min.x,
            box_max.x,
            box_max.y,
            material.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::ZX,
            box_min.z,
            box_max.z,
            box_min.x,
            box_max.x,
            box_min.y,
            material.clone(),
        )));

        objects.push(Box::new(Rect::new(
            Plane::YZ,
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_max.x,
            material.clone(),
        )));
        objects.push(Box::new(Rect::new(
            Plane::YZ,
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_min.x,
            material,
        )));

        let sides = List::new(objects);

        Self {
            box_min,
            box_max,
            sides: Box::new(sides),
            _phantom: PhantomData,
        }
    }
}

impl<M: Clone + Material> Hittable for Cuboid<M> {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, time_min, time_max)
    }

    fn bounding_box(&self, _time_start: f32, _time_end: f32) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }

    fn count(&self) -> u32 {
        self.sides.count()
    }
}
