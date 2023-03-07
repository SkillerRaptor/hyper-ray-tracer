/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod application;
mod hit_record;
mod hittable;
mod ray;

use application::Application;

fn main() {
    let mut application = Application::new();
    application.run();
}
