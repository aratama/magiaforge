pub mod ceil;
pub mod floor;
pub mod map;
pub mod tile;
pub mod wall;

use super::asset::GameAssets;
use super::constant::*;
use super::entity::book_shelf::spawn_book_shelf;
use super::entity::chest::spawn_chest;
use super::entity::enemy::Enemy;
use super::entity::player::Player;
use super::entity::GameEntity;
use super::hud::overlay::OverlayNextState;
use super::states::GameState;
use super::world::ceil::spawn_roof_tiles;
use super::world::map::image_to_tilemap;
use super::world::map::TileMapChunk;
use super::world::tile::*;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use wall::respawn_wall_collisions;
use wall::WallCollider;

fn setup_world(
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
    collider_query: Query<Entity, With<WallCollider>>,
    world_tile: Query<Entity, With<WorldTile>>,
) {
    let level_aseprite = level_aseprites.get(assets.level.id()).unwrap();
    let level_image = images.get(level_aseprite.atlas_image.id()).unwrap();
    let chunk = image_to_tilemap(&level_image);
    respawn_world(
        &mut commands,
        level_aseprites,
        &assets,
        collider_query,
        &chunk,
        &world_tile,
    );
    spawn_entities(&mut commands, &assets, &chunk);
}

fn respawn_world(
    mut commands: &mut Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    assets: &Res<GameAssets>,
    collider_query: Query<Entity, With<WallCollider>>,
    chunk: &TileMapChunk,
    world_tile: &Query<Entity, With<WorldTile>>,
) {
    let asset_aseprite = level_aseprites.get(assets.asset.id()).unwrap();
    respawn_world_tilemap(&mut commands, &assets, asset_aseprite, &chunk, &world_tile);
    respawn_wall_collisions(&mut commands, &collider_query, &chunk);
}

fn respawn_world_tilemap(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    asset_aseprite: &Aseprite,
    chunk: &TileMapChunk,
    world_tile: &Query<Entity, With<WorldTile>>,
) {
    for entity in world_tile.iter() {
        commands.entity(entity).despawn();
    }

    let floor_map_size = TilemapSize {
        x: chunk.width as u32,
        y: chunk.height as u32,
    };

    let ceil_map_size = TilemapSize {
        x: chunk.width as u32 * 2,
        y: chunk.height as u32 * 2,
    };

    let floor_layer_entity = TilemapId(commands.spawn_empty().id());
    let roof_layer_entity = TilemapId(commands.spawn_empty().id());

    let floor_layer_storage = TileStorage::empty(floor_map_size);
    let ceil_layer_storage = TileStorage::empty(ceil_map_size);

    // 床と壁の生成
    for y in 0..chunk.height as i32 {
        for x in 0..chunk.width as i32 {
            match chunk.get_tile(x, y) {
                Tile::StoneTile => {
                    commands.spawn((
                        WorldTile,
                        Name::new("stone_tile"),
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.asset.clone(),
                            slice: "stone tile".into(),
                            transform: Transform::from_translation(Vec3::new(
                                x as f32 * TILE_SIZE,
                                y as f32 * -TILE_SIZE,
                                FLOOR_LAYER_Z,
                            )),
                            ..default()
                        },
                    ));
                }
                Tile::Wall => {
                    let tx = x as f32 * TILE_SIZE;
                    let ty = y as f32 * -TILE_SIZE;
                    let tz = ENTITY_LAYER_Z + (-ty * Z_ORDER_SCALE);

                    // 壁
                    if chunk.get_tile(x as i32, y as i32 + 1) != Tile::Wall {
                        commands.spawn((
                            WorldTile,
                            Name::new("wall"),
                            StateScoped(GameState::InGame),
                            AsepriteSliceBundle {
                                aseprite: assets.asset.clone(),
                                slice: "stone wall".into(),
                                transform: Transform::from_translation(Vec3::new(tx, ty - 4.0, tz)),
                                ..default()
                            },
                        ));
                    }

                    // 天井
                    if false
                        || chunk.is_empty(x - 1, y - 1)
                        || chunk.is_empty(x + 0, y - 1)
                        || chunk.is_empty(x + 1, y - 1)
                        || chunk.is_empty(x - 1, y + 0)
                        || chunk.is_empty(x + 0, y + 0)
                        || chunk.is_empty(x + 1, y + 0)
                        || chunk.is_empty(x - 1, y + 1)
                        || chunk.is_empty(x + 0, y + 1)
                        || chunk.is_empty(x + 1, y + 1)
                    {
                        spawn_roof_tiles(commands, assets, x, y)
                    }
                }
                _ => {}
            }
        }
    }

    // タイルマップ本体の生成
    let floor_tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    commands.entity(floor_layer_entity.0).insert((
        Name::new("floor layer tilemap"),
        StateScoped(GameState::InGame),
        TilemapBundle {
            grid_size: floor_tile_size.into(),
            map_type: TilemapType::default(),
            size: floor_map_size,
            storage: floor_layer_storage,
            texture: TilemapTexture::Single(asset_aseprite.atlas_image.clone()),
            tile_size: floor_tile_size,
            // transformを計算するのにはget_tilemap_center_transform という関数もありますが、
            // それだとそこにタイルマップの中心が来てしまうことに注意します
            // Asepriteの座標系とはYが反転していることもあり、ここでは自力でTransformを計算しています
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -TILE_SIZE * (floor_map_size.y - 1) as f32,
                FLOOR_LAYER_Z,
            )),
            ..default()
        },
    ));

    let ceil_tile_size = TilemapTileSize {
        x: TILE_HALF,
        y: TILE_HALF,
    };
    commands.entity(roof_layer_entity.0).insert((
        Name::new("roof layer tilemap"),
        StateScoped(GameState::InGame),
        TilemapBundle {
            grid_size: ceil_tile_size.into(),
            map_type: TilemapType::default(),
            size: ceil_map_size,
            storage: ceil_layer_storage,
            texture: TilemapTexture::Single(asset_aseprite.atlas_image.clone()),
            tile_size: ceil_tile_size,
            transform: Transform::from_translation(Vec3::new(
                // 床とタイル半分ずれているので -4 しているが、ここ直さないといけないかも
                -4.0,
                // 天井レイヤーは8.0だけ上にずらしていることに注意
                -TILE_SIZE * (floor_map_size.y - 1) as f32 + WALL_HEIGHT - 4.0,
                ROOF_LAYER_Z,
            )),
            ..default()
        },
    ));
}

fn spawn_entities(mut commands: &mut Commands, assets: &Res<GameAssets>, chunk: &TileMapChunk) {
    // エンティティの生成
    for (entity, x, y) in &chunk.entities {
        let tx = TILE_SIZE * *x as f32;
        let ty = TILE_SIZE * -*y as f32;
        match entity {
            GameEntity::BookShelf => {
                spawn_book_shelf(&mut commands, assets.asset.clone(), tx, ty);
            }
            GameEntity::Chest => {
                spawn_chest(&mut commands, assets.asset.clone(), tx, ty);
            }
        }
    }
}

fn update_world(
    player_query: Query<&Player>,
    enemy_query: Query<&Enemy>,
    mut overlay_next_state: ResMut<OverlayNextState>,
) {
    let player = player_query.get_single();
    if enemy_query.is_empty() || player.is_ok_and(|p| p.life == 0) {
        *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);
        app.add_systems(
            FixedUpdate,
            update_world.run_if(in_state(GameState::InGame)),
        );
    }
}

/// スライス名からタイルマップのインデックスを計算します
#[allow(dead_code)]
pub fn slice_to_tile_texture_index(
    asset_aseprite: &Aseprite,
    asset_atlas: &Image,
    slice: &str,
) -> TileTextureIndex {
    let asset_tile_size = asset_atlas.width() / TILE_SIZE as u32;
    let stone_tile_slice = asset_aseprite.slices.get(slice).unwrap();
    let stone_tile_slice_index = TileTextureIndex(
        asset_tile_size * (stone_tile_slice.rect.min.y / TILE_SIZE) as u32
            + (stone_tile_slice.rect.min.x / TILE_SIZE) as u32,
    );
    return stone_tile_slice_index;
}
