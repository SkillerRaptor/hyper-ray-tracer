/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

impl Hittable {
    pub(crate) fn new_bvh_node(
        source_objects: &[Hittable],
        start: usize,
        end: usize,
        time_0: f32,
        time_1: f32,
    ) -> Self {
        let mut rand = rand::thread_rng();
        let axis = rand.gen_range(0..3);
        let comperator = |a: &Hittable, b: &Hittable| -> bool {
            let mut box_a = Aabb::default();
            let mut box_b = Aabb::default();

            if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
                unreachable!()
            }

            box_a.minimum()[axis] < box_b.minimum()[axis]
        };

        let object_span = end - start;

        let left;
        let right;

        match object_span {
            1 => {
                left = Box::new(source_objects[start].clone());
                right = Box::new(source_objects[start].clone());
            }
            2 => {
                if comperator(&source_objects[start], &source_objects[start + 1]) {
                    left = Box::new(source_objects[start].clone());
                    right = Box::new(source_objects[start + 1].clone());
                } else {
                    left = Box::new(source_objects[start + 1].clone());
                    right = Box::new(source_objects[start].clone());
                }
            }
            _ => {
                let mut objects = source_objects.to_vec();
                objects.sort_by(|a, b| {
                    if comperator(&a, &b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                let mid = start + object_span / 2;
                left = Box::new(Self::new_bvh_node(&objects, start, mid, time_0, time_1));
                right = Box::new(Self::new_bvh_node(&objects, mid, end, time_0, time_1));
            }
        }

        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();

        if !left.bounding_box(time_0, time_1, &mut box_left)
            || !right.bounding_box(time_0, time_1, &mut box_right)
        {
            unreachable!();
        }

        Self::BvhNode {
            left,
            right,
            bounding_box: Self::surrounding_box(&box_left, &box_right),
        }
    }
}
