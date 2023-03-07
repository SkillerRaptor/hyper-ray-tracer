/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, ray::Ray};

use cgmath::{InnerSpace, Vector3};
use rand::{distributions::Uniform, prelude::Distribution};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Material {
    Lambertian { albedo: Vector3<f32> },
    Metal { albedo: Vector3<f32>, fuzz: f32 },
}

impl Material {
    pub(crate) fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        match *self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + Self::random_unit_vector();
                if Self::near_zero(scatter_direction) {
                    scatter_direction = hit_record.normal;
                }

                *scattered = Ray::new(hit_record.point, scatter_direction);
                *attenuation = albedo;
                true
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = Self::reflect(ray_in.direction().normalize(), hit_record.normal);
                *scattered = Ray::new(
                    hit_record.point,
                    reflected + fuzz * Self::random_in_unit_sphere(),
                );
                *attenuation = albedo;
                scattered.direction().dot(hit_record.normal) > 0.0
            }
        }
    }

    fn random_unit_vector() -> Vector3<f32> {
        Self::random_in_unit_sphere().normalize()
    }

    fn random_in_unit_sphere() -> Vector3<f32> {
        let mut rand = rand::thread_rng();
        let range = Uniform::from(-1.0..1.0);
        loop {
            let point = Vector3::new(
                range.sample(&mut rand),
                range.sample(&mut rand),
                range.sample(&mut rand),
            );

            if point.dot(point) < 1.0 {
                return point;
            }
        }
    }

    fn near_zero(vector: Vector3<f32>) -> bool {
        const S: f32 = 1e-8;
        (vector.x.abs() < S) && (vector.y.abs() < S) && (vector.z.abs() < S)
    }

    fn reflect(vector: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
        vector - 2.0 * vector.dot(n) * n
    }
}
