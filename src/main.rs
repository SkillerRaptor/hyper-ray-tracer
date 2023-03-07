/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod application;
mod hit_record;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use application::Application;

fn main() {
    let mut application = Application::new();
    application.run();
}
