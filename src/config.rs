use bevy::prelude::Color;

pub const MAX_DEPTH: usize = 8; // how many cells deep the board is

pub const TICK_TIME: f32 = 0.0; // how many seconds until next tick is processed

pub const ROTATE_TIME: f32 = 0.16; // how many seconds until the camera rotates
pub const ROTATE_SPEED: f32 = 13.2; // how fast the camera rotates

pub fn height_color(z: i32) -> Color {
    let actual_z = z - 16;
    let normalized_z = actual_z as f32 / MAX_DEPTH as f32;
    let g = normalized_z;
    // println!("g: {}, z: {}", g, z);
    // let b = z as f32 / MAX_DEPTH as f32;
    return Color::rgba(0.3, g, 0.2, 1.0);
}
