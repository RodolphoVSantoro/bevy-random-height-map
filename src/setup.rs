use crate::{
    config::height_color,
    types::{Board, Cell},
};
use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;

const GRID_SIZE: usize = 32;

fn mark_cell_height(
    board: &mut Board,
    marked: &mut Vec<Vec<bool>>,
    height_diff_map: &mut Vec<Vec<i32>>,
    x: u32,
    y: u32,
    height: i32,
) {
    marked[x as usize][y as usize] = true;
    board.0[x as usize][y as usize].height = height + 20;
    for dx in -1..2 as i32 {
        for dy in -1..2 as i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            let mut new_height = height + height_diff_map[x as usize][y as usize];
            if new_height < -4 {
                new_height = -4;
            }
            if new_height > 4 {
                new_height = 4;
            }
            if new_x < 0 || new_x >= GRID_SIZE as i32 || new_y < 0 || new_y >= GRID_SIZE as i32 {
                continue;
            }
            if marked[new_x as usize][new_y as usize] {
                continue;
            }
            mark_cell_height(
                board,
                marked,
                height_diff_map,
                new_x as u32,
                new_y as u32,
                new_height,
            );
        }
    }
}

pub fn create_board() -> Board {
    let mut marked: Vec<Vec<bool>> = vec![];
    let mut height_diff_map: Vec<Vec<i32>> = vec![];
    let mut board: Board = Board(vec![]);
    let mut text_result = String::new();
    for _ in 0..GRID_SIZE {
        let mut row: Vec<i32> = vec![];
        let mut row_marked: Vec<bool> = vec![];
        let mut row_cell: Vec<Cell> = vec![];
        for _ in 0..GRID_SIZE {
            let height_diff = (rand::random::<u32>() % 3) as i32 - 1;
            row.push(height_diff);
            row_marked.push(false);
            row_cell.push(Cell::default());
            text_result.push_str(&format!("{},", height_diff));
        }
        text_result.push_str("\n");
        marked.push(row_marked);
        height_diff_map.push(row);
        board.0.push(row_cell);
    }
    let mut diff_file = File::create("diff_map.txt").unwrap();
    diff_file.write_all(text_result.as_bytes()).unwrap();

    let start_x = rand::random::<u32>() % GRID_SIZE as u32;
    let start_y = rand::random::<u32>() % GRID_SIZE as u32;
    let start_height = 0;
    mark_cell_height(
        &mut board,
        &mut marked,
        &mut height_diff_map,
        start_x,
        start_y,
        start_height,
    );

    text_result = String::new();
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            text_result.push_str(&format!("{},", board.0[x][y].height));
        }
        text_result.push_str("\n");
    }
    let mut height_file = File::create("height_map.txt").unwrap();
    height_file.write_all(text_result.as_bytes()).unwrap();

    return board;
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const SCALE: f32 = 10.0;
    const X_SIZE: f32 = SCALE / GRID_SIZE as f32;
    const Y_SIZE: f32 = SCALE / GRID_SIZE as f32;
    const Z_SIZE: f32 = SCALE / GRID_SIZE as f32;

    let board = create_board();

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(-Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // init board sprites
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let x_3d = x as f32 * X_SIZE - SCALE / 2.0;
            let y_3d = board.0[x as usize][y as usize].height as f32 * Z_SIZE - SCALE / 2.0;
            let z_3d = y as f32 * Y_SIZE - SCALE / 2.0;
            let cube_size = Vec3::new(X_SIZE, Y_SIZE, Z_SIZE);
            let color = height_color(board.0[x as usize][y as usize].height);
            // plane
            commands.spawn((PbrBundle {
                mesh: meshes.add(Cuboid::default().mesh().scaled_by(cube_size)),
                transform: Transform::from_translation(Vec3::new(x_3d, y_3d, z_3d)),
                material: materials.add(color),
                ..default()
            },));
        }
    }
}
