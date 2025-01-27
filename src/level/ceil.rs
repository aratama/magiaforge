use crate::component::animated_slice::AnimatedSlice;
use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::level::appearance::TileSprite;
use crate::level::map::LevelChunk;
use crate::level::tile::Tile;
use crate::registry::Registry;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

pub const WALL_HEIGHT_IN_TILES: u32 = 2;

/// 描画しようとしているタイルの、左上のタイルを4x4タイルセットのインデックスで返します
pub fn get_tile_index_left_top(
    chunk: &LevelChunk,
    xi: i32,
    yi: i32,
    depth: i32,
    targets: &Vec<&Tile>,
) -> i32 {
    match (
        chunk.is_visible_ceil(xi - 1, yi - 1, depth, targets),
        chunk.is_visible_ceil(xi + 0, yi - 1, depth, targets),
        chunk.is_visible_ceil(xi - 1, yi + 0, depth, targets),
    ) {
        (false, false, false) => 0,
        (false, false, true) => 1,
        (false, true, false) => 4,
        (false, true, true) => 10,
        (true, false, false) => 0,
        (true, false, true) => 1,
        (true, true, false) => 4,
        (true, true, true) => 16,
    }
}

pub fn get_tile_index_right_top(
    chunk: &LevelChunk,
    xi: i32,
    yi: i32,
    depth: i32,
    targets: &Vec<&Tile>,
) -> i32 {
    match (
        chunk.is_visible_ceil(xi + 0, yi - 1, depth, targets),
        chunk.is_visible_ceil(xi + 1, yi - 1, depth, targets),
        chunk.is_visible_ceil(xi + 1, yi + 0, depth, targets),
    ) {
        (false, false, false) => 3,
        (false, false, true) => 1,
        (false, true, false) => 3,
        (false, true, true) => 1,
        (true, false, false) => 7,
        (true, false, true) => 9,
        (true, true, false) => 7,
        (true, true, true) => 16,
    }
}

pub fn get_tile_index_left_bottom(
    chunk: &LevelChunk,
    xi: i32,
    yi: i32,
    depth: i32,
    targets: &Vec<&Tile>,
) -> i32 {
    match (
        chunk.is_visible_ceil(xi - 1, yi + 0, depth, targets),
        chunk.is_visible_ceil(xi - 1, yi + 1, depth, targets),
        chunk.is_visible_ceil(xi + 0, yi + 1, depth, targets),
    ) {
        (false, false, false) => 12,
        (false, false, true) => 4,
        (false, true, false) => 12,
        (false, true, true) => 4,
        (true, false, false) => 13,
        (true, false, true) => 6,
        (true, true, false) => 13,
        (true, true, true) => 16,
    }
}

pub fn get_tile_index_right_bottom(
    chunk: &LevelChunk,
    xi: i32,
    yi: i32,
    depth: i32,
    targets: &Vec<&Tile>,
) -> i32 {
    match (
        chunk.is_visible_ceil(xi + 1, yi + 0, depth, targets),
        chunk.is_visible_ceil(xi + 0, yi + 1, depth, targets),
        chunk.is_visible_ceil(xi + 1, yi + 1, depth, targets),
    ) {
        (false, false, false) => 15,
        (false, false, true) => 15,
        (false, true, false) => 7,
        (false, true, true) => 7,
        (true, false, false) => 13,
        (true, false, true) => 13,
        (true, true, false) => 5,
        (true, true, true) => 16,
    }
}

/// ひとつのタイルを四分割し、それぞれのオートタイルを選択して描画します
/// prefixesにはアニメーションのフレームごとにスライスのプリフィックスを渡します
/// オートタイルが選択されると、そのプリフィックスに _0 ～ _16 を選択して追加しスライス名とします
pub fn spawn_autotiles<T: Component + Clone>(
    prefixes: &Vec<String>,
    commands: &mut Commands,
    registry: &Registry,
    chunk: &LevelChunk,
    targets: &Vec<&Tile>,
    y_offset: f32,
    xi: i32,
    yi: i32,
    z: f32,
    depth: i32,
    marker: &T,
) {
    let lt = get_tile_index_left_top(chunk, xi, yi, depth, targets);
    spawn_autotile(
        prefixes, commands, registry, y_offset, xi, yi, z, 0, 0, lt, marker,
    );
    let rt = get_tile_index_right_top(chunk, xi, yi, depth, targets);
    spawn_autotile(
        prefixes, commands, registry, y_offset, xi, yi, z, 1, 0, rt, marker,
    );
    let lb = get_tile_index_left_bottom(chunk, xi, yi, depth, targets);
    spawn_autotile(
        prefixes, commands, registry, y_offset, xi, yi, z, 0, 1, lb, marker,
    );
    let rb = get_tile_index_right_bottom(chunk, xi, yi, depth, targets);
    spawn_autotile(
        prefixes, commands, registry, y_offset, xi, yi, z, 1, 1, rb, marker,
    );
}

fn spawn_autotile<T: Component + Clone>(
    prefix: &Vec<String>,
    commands: &mut Commands,
    registry: &Registry,
    y_offset: f32,
    xi: i32,
    yi: i32,
    z: f32,
    dx: i32,
    dy: i32,
    roof_index: i32,
    marker: &T,
) {
    let x = TILE_SIZE * xi as f32 + TILE_HALF * dx as f32;
    let y = (TILE_SIZE * -yi as f32) + TILE_HALF * -dy as f32 + y_offset;
    commands.spawn((
        TileSprite((xi, yi)),
        Name::new(prefix[0].clone()),
        StateScoped(GameState::InGame),
        Transform::from_xyz(x, y, z),
        AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
            name: format!("{}_{:?}", prefix[0], roof_index).to_string(),
        },
        AnimatedSlice {
            slices: prefix
                .clone()
                .iter()
                .map(|s| format!("{}_{}", s, roof_index))
                .collect(),
            wait: 50,
        },
        marker.clone(),
    ));
}
