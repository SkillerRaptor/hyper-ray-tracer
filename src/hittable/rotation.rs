/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::f32::consts::PI;

use crate::{aabb::Aabb, hit_record::HitRecord, hittable::Hittable, math::Vec3, ray::Ray};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn get_axises(&self) -> (usize, usize, usize) {
        match *self {
            Axis::X => (0, 1, 2),
            Axis::Y => (1, 2, 0),
            Axis::Z => (2, 0, 1),
        }
    }
}

pub(crate) struct Rotation {
    hittable: Box<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: Option<Aabb>,
    axis: Axis,
}

impl Rotation {
    pub(crate) fn new(axis: Axis, hittable: Box<dyn Hittable>, angle: f32) -> Self {
        let (r_axis, a_axis, b_axis) = axis.get_axises();
        let radians = (PI / 180.0) * angle;
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);
        let bounding_box = hittable.bounding_box(0.0, 1.0).map(|mut b| {
            let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let r =
                            k as f32 * b.maximum()[r_axis] + (1 - k) as f32 * b.minimum()[r_axis];
                        let a =
                            i as f32 * b.maximum()[a_axis] + (1 - i) as f32 * b.minimum()[a_axis];
                        let b =
                            j as f32 * b.maximum()[b_axis] + (1 - j) as f32 * b.minimum()[b_axis];

                        let new_a = cos_theta * a - sin_theta * b;
                        let new_b = sin_theta * a + cos_theta * b;

                        if new_a < min[a_axis] {
                            min[a_axis] = new_a;
                        }

                        if new_b < min[b_axis] {
                            min[b_axis] = new_b;
                        }

                        if r < min[r_axis] {
                            min[r_axis] = r;
                        }

                        if new_a > max[a_axis] {
                            max[a_axis] = new_a;
                        }

                        if new_b > max[b_axis] {
                            max[b_axis] = new_b;
                        }

                        if r > max[r_axis] {
                            max[r_axis] = r;
                        }
                    }
                }
            }

            b.set_minimum(min);
            b.set_maximum(max);
            b
        });

        Self {
            hittable,
            sin_theta,
            cos_theta,
            bounding_box,
            axis,
        }
    }
}

impl Hittable for Rotation {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord> {
        let (_, a_axis, b_axis) = self.axis.get_axises();
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[a_axis] =
            self.cos_theta * ray.origin()[a_axis] + self.sin_theta * ray.origin()[b_axis];
        origin[b_axis] =
            -self.sin_theta * ray.origin()[a_axis] + self.cos_theta * ray.origin()[b_axis];
        direction[a_axis] =
            self.cos_theta * ray.direction()[a_axis] + self.sin_theta * ray.direction()[b_axis];
        direction[b_axis] =
            -self.sin_theta * ray.direction()[a_axis] + self.cos_theta * ray.direction()[b_axis];

        let rotated_ray = Ray::new(origin, direction, ray.time());
        self.hittable
            .hit(&rotated_ray, time_min, time_max)
            .map(|mut hit| {
                let mut point = hit.point;
                let mut normal = hit.normal;
                point[a_axis] =
                    self.cos_theta * hit.point[a_axis] - self.sin_theta * hit.point[b_axis];
                point[b_axis] =
                    self.sin_theta * hit.point[a_axis] + self.cos_theta * hit.point[b_axis];
                normal[a_axis] =
                    self.cos_theta * hit.normal[a_axis] - self.sin_theta * hit.normal[b_axis];
                normal[b_axis] =
                    self.sin_theta * hit.normal[a_axis] + self.cos_theta * hit.normal[b_axis];
                hit.point = point;
                hit.normal = normal;
                hit
            })
    }

    fn bounding_box(&self, _time_start: f32, _time_end: f32) -> Option<Aabb> {
        self.bounding_box
    }

    fn count(&self) -> u32 {
        1
    }
}
