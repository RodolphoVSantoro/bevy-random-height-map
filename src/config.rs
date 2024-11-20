use bevy::prelude::Color;

pub const INITIAL_HEIGHT: i32 = 60; // the initial height of the board for rendering
pub const MAX_HEIGHT_MOD: usize = 4; // how high the board can go up or down
pub const MAX_DEPTH: usize = 2 * MAX_HEIGHT_MOD; // how many cells deep the board is

pub const TICK_TIME: f32 = 0.0; // how many seconds until next tick is processed

pub const ROTATE_TIME: f32 = 0.16; // how many seconds until the camera rotates
pub const ROTATE_SPEED: f32 = 3.2; // how fast the camera rotates

pub fn height_color(z: i32) -> Color {
    let actual_z = z - (INITIAL_HEIGHT - MAX_HEIGHT_MOD as i32);
    let normalized_z = actual_z as f32 / MAX_DEPTH as f32;
    let g = normalized_z;
    // println!("g: {}, z: {}", g, z);
    // let b = z as f32 / MAX_DEPTH as f32;
    return Color::rgba(0.3, g, 0.2, 1.0);
}
