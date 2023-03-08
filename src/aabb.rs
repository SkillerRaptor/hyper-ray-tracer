/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{math::Vec3, ray::Ray};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Aabb {
    minimum: Vec3,
    maximum: Vec3,
}

impl Aabb {
    pub(crate) fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub(crate) fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> bool {
        for a in 0..3 {
            let inverse_direction = 1.0 / ray.direction()[a];
            let mut time_start = (self.minimum[a] - ray.origin()[a]) * inverse_direction;
            let mut time_end = (self.maximum[a] - ray.origin()[a]) * inverse_direction;
            if inverse_direction < 0.0 {
                std::mem::swap(&mut time_start, &mut time_end);
            }

            let t_min = if time_start > time_min {
                time_start
            } else {
                time_min
            };

            let t_max = if time_end < time_max {
                time_end
            } else {
                time_max
            };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub(crate) fn surrounding_box(box_0: Self, box_1: Self) -> Self {
        let small = Vec3::new(
            box_0.minimum().x.min(box_1.minimum().x),
            box_0.minimum().y.min(box_1.minimum().y),
            box_0.minimum().z.min(box_1.minimum().z),
        );

        let big = Vec3::new(
            box_0.maximum().x.max(box_1.maximum().x),
            box_0.maximum().y.max(box_1.maximum().y),
            box_0.maximum().z.max(box_1.maximum().z),
        );

        Self::new(small, big)
    }

    pub(crate) fn minimum(&self) -> Vec3 {
        self.minimum
    }

    pub(crate) fn maximum(&self) -> Vec3 {
        self.maximum
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            minimum: Vec3::new(0.0, 0.0, 0.0),
            maximum: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
