/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::cmp::Ordering;

use crate::{aabb::Aabb, hit_record::HitRecord, hittable::Hittable, ray::Ray};

enum Node {
    Branch {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
    },
    Leaf {
        leaf: Box<dyn Hittable>,
    },
}

pub(crate) struct BvhNode {
    tree: Node,
    bounding_box: Aabb,
}

impl BvhNode {
    pub(crate) fn new(mut objects: Vec<Box<dyn Hittable>>, time_start: f32, time_end: f32) -> Self {
        let mut axis_ranges: Vec<(usize, f32)> = (0..3)
            .map(|axis| (axis, Self::axis_range(&objects, time_start, time_end, axis)))
            .collect();
        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;
        objects.sort_unstable_by(Self::box_compare(time_start, time_end, axis));

        let len = objects.len();
        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = objects.pop().unwrap();
                let Some(aabb) = leaf.bounding_box(time_start, time_end) else {
                    panic!()
                };

                Self {
                    tree: Node::Leaf { leaf },
                    bounding_box: aabb,
                }
            }
            _ => {
                let right = Self::new(objects.drain(len / 2..).collect(), time_start, time_end);
                let left = Self::new(objects, time_start, time_end);
                let aabb = Aabb::surrounding_box(left.bounding_box, right.bounding_box);
                Self {
                    tree: Node::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bounding_box: aabb,
                }
            }
        }
    }

    fn box_compare(
        time_start: f32,
        time_end: f32,
        axis: usize,
    ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
        move |a, b| {
            let a_bbox = a.bounding_box(time_start, time_end);
            let b_bbox = b.bounding_box(time_start, time_end);
            if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                let ac = a.minimum()[axis] + a.maximum()[axis];
                let bc = b.minimum()[axis] + b.maximum()[axis];
                ac.partial_cmp(&bc).unwrap()
            } else {
                panic!()
            }
        }
    }

    fn axis_range(
        objects: &Vec<Box<dyn Hittable>>,
        time_start: f32,
        time_end: f32,
        axis: usize,
    ) -> f32 {
        let (min, max) = objects
            .iter()
            .fold((f32::MAX, f32::MIN), |(min, max), hit| {
                let Some(aabb) = hit.bounding_box(time_start, time_end) else {
                    return (min, max)
                };

                (min.min(aabb.minimum()[axis]), max.max(aabb.maximum()[axis]))
            });

        max - min
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, time_min, time_max) {
            return None;
        }

        match &self.tree {
            Node::Branch { left, right } => {
                let left = left.hit(ray, time_min, time_max);

                let mut time_max = time_max;
                if let Some(hit) = left {
                    time_max = hit.t;
                }

                let right = right.hit(ray, time_min, time_max);
                if right.is_some() {
                    right
                } else {
                    left
                }
            }
            Node::Leaf { leaf } => leaf.hit(ray, time_min, time_max),
        }
    }

    fn bounding_box(&self, _time_start: f32, _time_end: f32) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
