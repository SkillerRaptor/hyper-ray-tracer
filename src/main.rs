/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod application;

use application::Application;

fn main() {
    let mut application = Application::new();
    application.run();
}
