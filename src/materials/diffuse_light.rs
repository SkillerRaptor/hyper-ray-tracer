/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{hit_record::HitRecord, materials::Material, math::Vec3, ray::Ray, textures::Texture};

#[derive(Clone)]
pub(crate) struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub(crate) fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}
