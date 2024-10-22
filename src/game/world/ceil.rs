use crate::game::{asset::GameAssets, states::GameState};

use super::{super::world::slice_to_tile_texture_index, WorldTile, TILE_HALF, TILE_SIZE};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Aseprite, AsepriteSlice, AsepriteSliceBundle};
use bevy_ecs_tilemap::prelude::*;

#[allow(dead_code)]
pub fn get_ceil_tile_indices(
    asset_aseprite: &Aseprite,
    asset_image: &Image,
) -> Vec<TileTextureIndex> {
    let mut ceil_tile_indices = Vec::with_capacity(16);
    for i in 0..16 {
        ceil_tile_indices.push(slice_to_tile_texture_index(
            asset_aseprite,
            asset_image,
            format!("roof{}", i).as_str(),
        ));
    }
    return ceil_tile_indices;
}

pub fn spawn_roof_tiles(commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
    spawn_roof_tile(commands, assets, x, y, 0, 0, 0);
    spawn_roof_tile(commands, assets, x, y, 1, 0, 3);
    spawn_roof_tile(commands, assets, x, y, 0, 1, 12);
    spawn_roof_tile(commands, assets, x, y, 1, 1, 15);
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
    commands.spawn((
        WorldTile,
        Name::new("ceil"),
        StateScoped(GameState::InGame),
        AsepriteSliceBundle {
            aseprite: assets.asset.clone(),
            slice: AsepriteSlice::new(format!("roof{:?}", roof_index).as_str()),
            transform: Transform::from_xyz(
                TILE_SIZE * x as f32 + TILE_HALF * dx as f32 - 4.0,
                (TILE_SIZE * -y as f32) + TILE_HALF * -dy as f32 + 12.0,
                5.0,
            ),
            ..default()
        },
    ));
}
