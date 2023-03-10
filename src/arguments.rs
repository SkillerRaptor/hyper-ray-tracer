/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum Scene {
    Random,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth,
    SimpleLight,
    Cornell,
    CornellSmoke,
    Final,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct Arguments {
    /// Width of the window
    #[arg(long, default_value_t = 1280)]
    pub(crate) width: u32,

    /// Height of the window
    #[arg(long, default_value_t = 720)]
    pub(crate) height: u32,

    /// Samples per pixel
    #[arg(long, default_value_t = 100)]
    pub(crate) samples: u32,

    /// Max depth
    #[arg(long, default_value_t = 10)]
    pub(crate) depth: u32,

    /// Tile size
    #[arg(long, default_value_t = 80)]
    pub(crate) tile_size: u32,

    /// Scene
    #[arg(long, value_enum, default_value_t = Scene::Random)]
    pub(crate) scene: Scene,
}
