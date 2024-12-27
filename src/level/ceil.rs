use crate::asset::GameAssets;
use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::constant::WALL_HEIGHT;
use crate::entity::get_entity_z;
use crate::level::map::LevelChunk;
use crate::level::tile::WorldTile;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

pub const WALL_HEIGHT_IN_TILES: u32 = 2;

pub fn spawn_roof_tiles(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk: &LevelChunk,
    x: i32,
    y: i32,
) {
    // デバッグ用
    // commands.spawn((
    //     AseSpriteSlice {
    //         aseprite: assets.atlas.clone(),
    //         name: "roof_test".to_string(),
    //     },
    //     Transform::from_xyz(
    //         TILE_SIZE * x as f32,
    //         (TILE_SIZE * (-y + WALL_HEIGHT_IN_TILES as i32) as f32),
    //         999.0,
    //     ),
    // ));

    let left_top = match (
        chunk.is_visible_ceil(x - 1, y - 1),
        chunk.is_visible_ceil(x + 0, y - 1),
        chunk.is_visible_ceil(x - 1, y + 0),
    ) {
        (false, false, false) => 0,
        (false, false, true) => 1, // 2
        (false, true, false) => 4, // 8
        (false, true, true) => 10,
        (true, false, false) => 0,
        (true, false, true) => 1, // 2
        (true, true, false) => 4, // 8
        (true, true, true) => 16,
    };
    spawn_roof_tile(commands, assets, x, y, 0, 0, left_top);

    let right_top = match (
        chunk.is_visible_ceil(x + 0, y - 1),
        chunk.is_visible_ceil(x + 1, y - 1),
        chunk.is_visible_ceil(x + 1, y + 0),
    ) {
        (false, false, false) => 3,
        (false, false, true) => 1, // 2
        (false, true, false) => 3,
        (false, true, true) => 1,  // 2
        (true, false, false) => 7, // 11
        (true, false, true) => 9,
        (true, true, false) => 7, // 11
        (true, true, true) => 16,
    };
    spawn_roof_tile(commands, assets, x, y, 1, 0, right_top);

    let left_bottom = match (
        chunk.is_visible_ceil(x - 1, y + 0),
        chunk.is_visible_ceil(x - 1, y + 1),
        chunk.is_visible_ceil(x + 0, y + 1),
    ) {
        (false, false, false) => 12,
        (false, false, true) => 4, // 8
        (false, true, false) => 12,
        (false, true, true) => 4,   // 8
        (true, false, false) => 13, // 14
        (true, false, true) => 6,
        (true, true, false) => 13, // 14
        (true, true, true) => 16,
    };
    spawn_roof_tile(commands, assets, x, y, 0, 1, left_bottom);

    let right_bottom = match (
        chunk.is_visible_ceil(x + 1, y + 0),
        chunk.is_visible_ceil(x + 0, y + 1),
        chunk.is_visible_ceil(x + 1, y + 1),
    ) {
        (false, false, false) => 15,
        (false, false, true) => 15,
        (false, true, false) => 7,  // 11
        (false, true, true) => 7,   // 11
        (true, false, false) => 13, // 14
        (true, false, true) => 13,  // 14
        (true, true, false) => 5,
        (true, true, true) => 16,
    };
    spawn_roof_tile(commands, assets, x, y, 1, 1, right_bottom);
}

fn spawn_roof_tile(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    roof_index: i32,
) {
    let x = TILE_SIZE * x as f32 + TILE_HALF * dx as f32;
    let y = (TILE_SIZE * -y as f32) + TILE_HALF * -dy as f32 + WALL_HEIGHT;
    let z = get_entity_z(y - WALL_HEIGHT);
    commands.spawn((
        Name::new("ceil"),
        WorldTile,
        StateScoped(GameState::InGame),
        Transform::from_xyz(x, y, z),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: format!("roof{:?}", roof_index).to_string(),
        },
    ));
}
