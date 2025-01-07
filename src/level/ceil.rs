use crate::asset::GameAssets;
use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::level::map::LevelChunk;
use crate::level::tile::WorldTile;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

use super::tile::Tile;

pub const WALL_HEIGHT_IN_TILES: u32 = 2;

pub fn spawn_roof_tiles(
    prefix: &'static str,
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk: &LevelChunk,
    a: Tile,
    b: Tile,
    y_offset: f32,
    xi: i32,
    yi: i32,
    z: f32,
    depth: i32,
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
        chunk.is_visible_ceil(xi - 1, yi - 1, depth, a, b),
        chunk.is_visible_ceil(xi + 0, yi - 1, depth, a, b),
        chunk.is_visible_ceil(xi - 1, yi + 0, depth, a, b),
    ) {
        (false, false, false) => 0,
        (false, false, true) => 1,
        (false, true, false) => 4,
        (false, true, true) => 10,
        (true, false, false) => 0,
        (true, false, true) => 1,
        (true, true, false) => 4,
        (true, true, true) => 16,
    };
    spawn_roof_tile(
        prefix, commands, assets, y_offset, xi, yi, z, 0, 0, left_top,
    );

    if xi == 12 && yi == 33 {
        info!("xi: {}, yi: {}", xi, yi);
        let t0 = chunk.is_visible_ceil(xi + 0, yi - 1, depth, a, b);
        let t1 = chunk.is_visible_ceil(xi + 1, yi - 1, depth, a, b);
        let t2 = chunk.is_visible_ceil(xi + 1, yi + 0, depth, a, b);
        info!("t0: {}, t1: {}, t2: {}", t0, t1, t2);
    }

    let right_top = match (
        chunk.is_visible_ceil(xi + 0, yi - 1, depth, a, b),
        chunk.is_visible_ceil(xi + 1, yi - 1, depth, a, b),
        chunk.is_visible_ceil(xi + 1, yi + 0, depth, a, b),
    ) {
        (false, false, false) => 3,
        (false, false, true) => 1,
        (false, true, false) => 3,
        (false, true, true) => 1,
        (true, false, false) => 7,
        (true, false, true) => 9,
        (true, true, false) => 7,
        (true, true, true) => 16,
    };
    spawn_roof_tile(
        prefix, commands, assets, y_offset, xi, yi, z, 1, 0, right_top,
    );

    let left_bottom = match (
        chunk.is_visible_ceil(xi - 1, yi + 0, depth, a, b),
        chunk.is_visible_ceil(xi - 1, yi + 1, depth, a, b),
        chunk.is_visible_ceil(xi + 0, yi + 1, depth, a, b),
    ) {
        (false, false, false) => 12,
        (false, false, true) => 4,
        (false, true, false) => 12,
        (false, true, true) => 4,
        (true, false, false) => 13,
        (true, false, true) => 6,
        (true, true, false) => 13,
        (true, true, true) => 16,
    };
    spawn_roof_tile(
        prefix,
        commands,
        assets,
        y_offset,
        xi,
        yi,
        z,
        0,
        1,
        left_bottom,
    );

    let right_bottom = match (
        chunk.is_visible_ceil(xi + 1, yi + 0, depth, a, b),
        chunk.is_visible_ceil(xi + 0, yi + 1, depth, a, b),
        chunk.is_visible_ceil(xi + 1, yi + 1, depth, a, b),
    ) {
        (false, false, false) => 15,
        (false, false, true) => 15,
        (false, true, false) => 7,
        (false, true, true) => 7,
        (true, false, false) => 13,
        (true, false, true) => 13,
        (true, true, false) => 5,
        (true, true, true) => 16,
    };
    spawn_roof_tile(
        prefix,
        commands,
        assets,
        y_offset,
        xi,
        yi,
        z,
        1,
        1,
        right_bottom,
    );
}

fn spawn_roof_tile(
    prefix: &'static str,
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    y_offset: f32,
    xi: i32,
    yi: i32,
    z: f32,
    dx: i32,
    dy: i32,
    roof_index: i32,
) {
    let x = TILE_SIZE * xi as f32 + TILE_HALF * dx as f32;
    let y = (TILE_SIZE * -yi as f32) + TILE_HALF * -dy as f32 + y_offset;
    commands.spawn((
        Name::new(prefix),
        WorldTile,
        StateScoped(GameState::InGame),
        Transform::from_xyz(x, y, z),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: format!("{}_{:?}", prefix, roof_index).to_string(),
        },
    ));
}
