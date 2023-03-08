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
mod material;
mod math;
mod ray;

use application::Application;

fn main() {
    let mut application = Application::new();
    application.run();
}
