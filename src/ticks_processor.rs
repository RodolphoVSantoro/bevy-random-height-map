use crate::config::ROTATE_SPEED;
use crate::types::{FrameTimer, RotateTimer};

use bevy::prelude::*;

pub fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut frame_timer: ResMut<FrameTimer>,
    mut rotate_timer: ResMut<RotateTimer>,
) {
    static mut ROTATED: bool = false;
    if !frame_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    if !rotate_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut transform = query.single_mut();

    if !unsafe { ROTATED } {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-0.4));
        unsafe { ROTATED = true };
    }

    transform.rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_y(time.delta_seconds() * ROTATE_SPEED),
    );
}
