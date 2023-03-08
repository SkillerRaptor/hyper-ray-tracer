/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod aabb;
mod application;
mod arguments;
mod camera;
mod hit_record;
mod hittable;
mod logger;
mod materials;
mod math;
mod ray;
mod textures;

use application::Application;
use arguments::Arguments;
use clap::Parser;

fn main() {
    let arguments = Arguments::parse();

    logger::init(3);

    let mut application = Application::new(arguments);
    application.run();
}
