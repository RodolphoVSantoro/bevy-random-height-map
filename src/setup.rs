use crate::{
    config::{GRID_SIZE, INITIAL_HEIGHT},
    types::{Board, Cell},
};
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};
use bevy_panorbit_camera::PanOrbitCamera;
use std::fs::File;
use std::io::prelude::*;

const STEPS: &'static [(i32, i32); 9] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (0, 0),
];

fn mark_cell_height(
    board: &mut Board,
    marked: &mut Vec<Vec<bool>>,
    height_diff_map: &mut Vec<Vec<i32>>,
    x: u32,
    y: u32,
    height: i32,
) {
    let mut new_x;
    let mut new_y;
    let mut new_height;
    let mut stack: Vec<(u32, u32, i32)> = vec![];
    stack.push((x, y, height));
    while !stack.is_empty() {
        let (x, y, height) = stack.pop().unwrap();
        for (dy, dx) in STEPS {
            new_x = x as i32 + dx;
            new_y = y as i32 + dy;
            if new_x < 0 || new_x >= GRID_SIZE as i32 || new_y < 0 || new_y >= GRID_SIZE as i32 {
                continue;
            }
            if marked[new_x as usize][new_y as usize] {
                continue;
            }
            new_height = height + height_diff_map[new_x as usize][new_y as usize];
            board.0[new_x as usize][new_y as usize].height = new_height + INITIAL_HEIGHT;
            marked[new_x as usize][new_y as usize] = true;
            stack.push((new_x as u32, new_y as u32, new_height));
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
            let rand_cent = rand::random::<u32>() % 1000;
            let mut height_diff = 0;
            if rand_cent < 10 {
                height_diff = -1;
            } else if rand_cent > 990 {
                height_diff = 1;
            }

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

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let board = create_board();

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(-Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 200.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    let terrain_mesh = meshes.add(create_terrain_mesh(&board));

    commands.spawn((PbrBundle {
        mesh: terrain_mesh,
        ..default()
    },));
}

fn height_color_vec(height: i32, max_height: i32, min_height: i32, vertex: f32) -> Vec4 {
    let normalized = (height - min_height) as f32 / (max_height - min_height) as f32;
    return Vec4::new(normalized, 0.0, vertex / 4.0, 0.0);
}

fn create_terrain_mesh(board: &Board) -> Mesh {
    let mut max_height = 0;
    let mut min_height = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if board.0[x][y].height > max_height {
                max_height = board.0[x][y].height;
            }
            if board.0[x][y].height < min_height {
                min_height = board.0[x][y].height;
            }
        }
    }

    let mut triangles: Vec<Vec3> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut colors: Vec<Vec4> = Vec::new();
    let mut index = 0;

    for x in 0..GRID_SIZE - 1 {
        for y in 0..GRID_SIZE - 1 {
            let vertex_1 = get_coords_from_board(x, y, board);
            let vertex_2 = get_coords_from_board(x + 1, y, board);
            let vertex_3 = get_coords_from_board(x, y + 1, board);
            let vertex_4 = get_coords_from_board(x + 1, y + 1, board);

            let mut square = vec![vertex_1, vertex_2, vertex_3, vertex_4];
            triangles.append(&mut square);

            indices.append(&mut vec![index + 2, index + 1, index]);
            indices.append(&mut vec![index + 2, index + 3, index + 1]);

            colors.append(&mut vec![
                height_color_vec(board.0[x][y].height, max_height, min_height, 1.0),
                height_color_vec(board.0[x + 1][y].height, max_height, min_height, 2.0),
                height_color_vec(board.0[x][y + 1].height, max_height, min_height, 3.0),
                height_color_vec(board.0[x + 1][y + 1].height, max_height, min_height, 4.0),
            ]);

            index += 4;
        }
    }

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, triangles)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices));

    return mesh;
}

fn get_coords_from_board(x: usize, y: usize, board: &Board) -> Vec3 {
    let x_3d = x as f32;
    let y_3d = board.0[x][y].height as f32 / 3.0;
    let z_3d = y as f32;
    return Vec3::new(x_3d, y_3d, z_3d);
}
