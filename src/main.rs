#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::single_match)]
#![allow(clippy::single_match_else)]
#![allow(clippy::uninlined_format_args)]

use bevy::prelude::{App, DefaultPlugins, Startup, Timer, TimerMode, Update};

mod config;
use config::{ROTATE_TIME, TICK_TIME};

mod types;
use types::{FrameTimer, RotateTimer};

mod setup;
use setup::setup;
mod ticks_processor;
use ticks_processor::rotate_camera;

fn main() {
    App::new()
        .insert_resource(FrameTimer(Timer::from_seconds(
            TICK_TIME,
            TimerMode::Repeating,
        )))
        .insert_resource(RotateTimer(Timer::from_seconds(
            ROTATE_TIME,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}
