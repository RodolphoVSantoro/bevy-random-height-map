use bevy::prelude::Resource;

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
