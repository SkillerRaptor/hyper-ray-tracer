/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, ray::Ray};

use cgmath::{InnerSpace, Vector3};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Material {
    Lambertian { albedo: Vector3<f32> },
    Metal { albedo: Vector3<f32>, fuzz: f32 },
    Dielectric { index_of_referaction: f32 },
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
            Material::Dielectric {
                index_of_referaction,
            } => {
                *attenuation = Vector3::new(1.0, 1.0, 1.0);

                let refraction_ratio = if hit_record.front_face {
                    1.0 / index_of_referaction
                } else {
                    index_of_referaction
                };

                let unit_direction = ray_in.direction().normalize();
                let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
                let mut rand = rand::thread_rng();
                let direction = if cannot_refract
                    || Self::reflectance(cos_theta, refraction_ratio) > rand.gen()
                {
                    Self::reflect(unit_direction, hit_record.normal)
                } else {
                    Self::refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                *scattered = Ray::new(hit_record.point, direction);

                true
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

    fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
        v - 2.0 * v.dot(n) * n
    }

    fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.dot(r_out_perp)).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
