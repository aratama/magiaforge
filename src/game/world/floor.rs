use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn spawn_floor_tile(
    commands: &mut Commands,
    tile_pos: TilePos,
    tilemap_id: TilemapId,
    texture_index: TileTextureIndex,
) -> Entity {
    commands
        .spawn((
            Name::new("stone_tile"),
            TileBundle {
                position: tile_pos,
                tilemap_id,
                texture_index,
                ..default()
            },
        ))
        .id()
}
