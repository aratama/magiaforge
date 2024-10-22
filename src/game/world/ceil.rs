use super::super::world::slice_to_tile_texture_index;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_ecs_tilemap::prelude::*;

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

pub fn spawn_roof_tiles(
    commands: &mut Commands,
    floor_map_size: i32,
    mut ceil_layer_storage: &mut TileStorage,
    roof_layer_entity: TilemapId,
    ceil_tile_indices: &Vec<TileTextureIndex>,
    x: i32,
    y: i32,
) {
    spawn_roof_tile(
        commands,
        floor_map_size,
        &mut ceil_layer_storage,
        roof_layer_entity,
        ceil_tile_indices[12],
        x,
        y,
        0,
        0,
    );
    spawn_roof_tile(
        commands,
        floor_map_size,
        &mut ceil_layer_storage,
        roof_layer_entity,
        ceil_tile_indices[15],
        x,
        y,
        1,
        0,
    );
    spawn_roof_tile(
        commands,
        floor_map_size,
        &mut ceil_layer_storage,
        roof_layer_entity,
        ceil_tile_indices[0],
        x,
        y,
        0,
        1,
    );
    spawn_roof_tile(
        commands,
        floor_map_size,
        &mut ceil_layer_storage,
        roof_layer_entity,
        ceil_tile_indices[3],
        x,
        y,
        1,
        1,
    );
}

fn spawn_roof_tile(
    commands: &mut Commands,
    floor_map_size: i32,
    ceil_layer_storage: &mut TileStorage,
    roof_layer_entity: TilemapId,
    roof0_slice_index: TileTextureIndex,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) {
    // とりあえず左上
    let ceil_tile_pos = TilePos {
        x: (x * 2 + dx) as u32,
        y: (((floor_map_size - y - 1) * 2) + dy) as u32,
    };

    ceil_layer_storage.set(
        &ceil_tile_pos,
        commands
            .spawn((
                Name::new("roof tile"),
                TileBundle {
                    position: ceil_tile_pos,
                    tilemap_id: roof_layer_entity,
                    texture_index: roof0_slice_index,
                    ..default()
                },
            ))
            .id(),
    );
}
