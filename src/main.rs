/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod aabb;
mod application;
mod camera;
mod hit_record;
mod hittable;
mod materials;
mod math;
mod ray;
mod textures;

use application::Application;

fn main() {
    let mut application = Application::new();
    application.run();
}
