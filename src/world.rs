pub mod ceil;
pub mod map;
pub mod tile;
pub mod wall;

use super::asset::GameAssets;
use super::constant::*;
use super::controller::player::Player;
use super::entity::actor::Actor;
use super::entity::book_shelf::spawn_book_shelf;
use super::entity::chest::spawn_chest;
use super::entity::slime::spawn_slime;
use super::entity::witch::spawn_witch;
use super::entity::GameEntity;
use super::hud::life_bar::LifeBarResource;
use super::hud::overlay::OverlayNextState;
use super::states::GameState;
use super::world::ceil::spawn_roof_tiles;
use super::world::map::image_to_tilemap;
use super::world::map::LevelTileMap;
use super::world::tile::*;
use crate::config::GameConfig;
use crate::entity::magic_circle::spawn_magic_circle;
use bevy::asset::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;
use map::image_to_empty_tiles;
use uuid::Uuid;
use wall::respawn_wall_collisions;
use wall::WallCollider;

#[derive(Resource, Debug, Clone, Default)]
pub struct NextLevel(pub Option<i32>);

fn setup_world(
    commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
    collider_query: Query<Entity, With<WallCollider>>,
    world_tile: Query<Entity, With<WorldTile>>,
    life_bar_res: Res<LifeBarResource>,
    camera: Query<&mut Transform, With<Camera2d>>,
    frame_count: Res<FrameCount>,
    config: Res<GameConfig>,
    next: Res<NextLevel>,
) {
    let level = match &next.0 {
        None => 0,
        Some(level) => level % LEVELS,
    };

    spawn_level(
        commands,
        level_aseprites,
        images,
        assets,
        collider_query,
        world_tile,
        life_bar_res,
        camera,
        frame_count,
        config,
        level,
    );
}

fn spawn_level(
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
    collider_query: Query<Entity, With<WallCollider>>,
    world_tile: Query<Entity, With<WorldTile>>,
    life_bar_res: Res<LifeBarResource>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    frame_count: Res<FrameCount>,
    config: Res<GameConfig>,
    level: i32,
) {
    info!("spawn_level {}", level);
    let level_aseprite = level_aseprites.get(assets.level.id()).unwrap();
    let level_image = images.get(level_aseprite.atlas_image.id()).unwrap();
    let slice = level_aseprite
        .slices
        .get(&format!("level{}", level))
        .unwrap();

    info!(
        "bounds min_x:{} max_x:{} min_y:{} max_y:{}",
        slice.rect.min.x, slice.rect.max.x, slice.rect.min.y, slice.rect.max.y
    );

    let chunk = image_to_tilemap(
        &level_image,
        slice.rect.min.x as i32,
        slice.rect.max.x as i32,
        slice.rect.min.y as i32,
        slice.rect.max.y as i32,
    );

    let mut empties = image_to_empty_tiles(&chunk);

    respawn_world(&mut commands, &assets, collider_query, &chunk, &world_tile);
    spawn_entities(&mut commands, &assets, &chunk);

    let player_position = random_select(&mut empties);
    let mut player_x = TILE_SIZE * player_position.0 as f32;
    let mut player_y = -TILE_SIZE * player_position.1 as f32;

    // デバッグ用
    player_x = TILE_SIZE * (chunk.min_x as f32 + 9.0);
    player_y = -TILE_SIZE * (chunk.min_y as f32 + 11.0);

    if let Ok(mut camera) = camera.get_single_mut() {
        camera.translation.x = player_x;
        camera.translation.y = player_y;
        info!(
            "camera x:{} y:{}",
            camera.translation.x, camera.translation.y
        );
    }

    let life = 150;
    let max_life = 150;
    spawn_witch(
        &mut commands,
        &assets,
        Vec2::new(player_x, player_y),
        0.0,
        Uuid::new_v4(),
        None,
        life,
        max_life,
        &life_bar_res,
        Player {
            name: config.player_name.clone(),
            last_idle_frame_count: *frame_count,
            last_ilde_x: player_x,
            last_ilde_y: player_y,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: life,
            last_idle_max_life: max_life,
        },
    );

    for _ in 0..10 {
        let (x, y) = random_select(&mut empties);
        spawn_slime(
            &mut commands,
            assets.slime.clone(),
            Vec2::new(
                TILE_SIZE * x as f32 + TILE_HALF,
                TILE_SIZE * -y as f32 - TILE_HALF,
            ),
            &life_bar_res,
        );
    }
}

fn random_select<T: Copy>(xs: &mut Vec<T>) -> T {
    xs.remove((rand::random::<usize>() % xs.len()) as usize)
}

fn respawn_world(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    collider_query: Query<Entity, With<WallCollider>>,
    chunk: &LevelTileMap,
    world_tile: &Query<Entity, With<WorldTile>>,
) {
    respawn_world_tilemap(&mut commands, &assets, &chunk, &world_tile);
    respawn_wall_collisions(&mut commands, &collider_query, &chunk);
}

fn respawn_world_tilemap(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk: &LevelTileMap,
    world_tile: &Query<Entity, With<WorldTile>>,
) {
    for entity in world_tile {
        commands.entity(entity).despawn_recursive();
    }

    // 床と壁の生成
    for y in chunk.min_y..chunk.max_y as i32 {
        for x in chunk.min_x..chunk.max_x as i32 {
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
                                transform: Transform::from_translation(Vec3::new(
                                    tx,
                                    ty - TILE_HALF,
                                    tz,
                                )),
                                ..default()
                            },
                        ));
                    }

                    // // 天井
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
                        spawn_roof_tiles(commands, assets, &chunk, x, y)
                    }
                }
                _ => {}
            }
        }
    }
}

fn spawn_entities(mut commands: &mut Commands, assets: &Res<GameAssets>, chunk: &LevelTileMap) {
    // エンティティの生成
    for (entity, x, y) in &chunk.entities {
        let tx = TILE_SIZE * *x as f32;
        let ty = TILE_SIZE * -*y as f32;
        match entity {
            GameEntity::BookShelf => {
                spawn_book_shelf(
                    &mut commands,
                    assets.asset.clone(),
                    tx + TILE_SIZE,
                    ty - TILE_HALF,
                );
            }
            GameEntity::Chest => {
                spawn_chest(
                    &mut commands,
                    assets.asset.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                );
            }
            GameEntity::MagicCircle => {
                spawn_magic_circle(
                    &mut commands,
                    assets.asset.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                );
            }
        }
    }
}

fn update_world(
    player_query: Query<&Actor, With<Player>>,
    mut overlay_next_state: ResMut<OverlayNextState>,
) {
    let player = player_query.get_single();
    if player.is_ok_and(|p| p.life == 0) {
        *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);

        app.add_systems(
            FixedUpdate,
            update_world
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );

        app.init_resource::<NextLevel>();
    }
}
