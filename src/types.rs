use bevy::prelude::{Resource, Timer};

pub struct Cell {
    pub height: i32,
}

impl Default for Cell {
    fn default() -> Self {
        return Cell { height: 0 };
    }
}

#[derive(Resource)]
pub struct Board(pub Vec<Vec<Cell>>);

#[derive(Resource)]
pub struct FrameTimer(pub Timer);

#[derive(Resource)]
pub struct RotateTimer(pub Timer);
