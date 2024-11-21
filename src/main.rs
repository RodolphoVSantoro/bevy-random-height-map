#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::single_match)]
#![allow(clippy::single_match_else)]
#![allow(clippy::uninlined_format_args)]

use bevy::prelude::{App, DefaultPlugins, Startup};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod config;
mod setup;
mod types;
use setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}
