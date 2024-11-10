pub mod ceil;
pub mod map;
pub mod tile;
pub mod wall;

use crate::asset::GameAssets;
use crate::command::GameCommand;
use crate::config::GameConfig;
use crate::constant::*;
use crate::controller::player::Player;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::slime::spawn_slime;
use crate::entity::book_shelf::spawn_book_shelf;
use crate::entity::broken_magic_circle::spawn_broken_magic_circle;
use crate::entity::chest::spawn_chest;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::magic_circle::MagicCircleDestination;
use crate::entity::stone_lantern::spawn_stone_lantern;
use crate::entity::witch::spawn_witch;
use crate::entity::GameEntity;
use crate::hud::life_bar::LifeBarResource;
use crate::states::GameState;
use crate::world::ceil::spawn_roof_tiles;
use crate::world::map::image_to_tilemap;
use crate::world::map::LevelTileMap;
use crate::world::tile::*;
use bevy::asset::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::Audio;
use map::image_to_spawn_tiles;
use uuid::Uuid;
use wall::respawn_wall_collisions;
use wall::WallCollider;

#[derive(Resource, Debug, Clone, Default)]
pub struct CurrentLevel(pub Option<i32>);

#[derive(Resource, Debug, Clone, Copy, Default)]

pub enum NextLevel {
    #[default]
    None,
    Level(i32),
    MultiPlayArena,
}

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
    mut current: ResMut<CurrentLevel>,
    next: Res<NextLevel>,
    mut writer: EventWriter<GameCommand>,
    audio: Res<Audio>,
) {
    info!("setup_world {:?}", next);

    writer.send(GameCommand::BGMDokutsu);

    let level_slice = match *next {
        NextLevel::None => "level0",
        NextLevel::Level(level) => &format!("level{}", level % LEVELS),
        NextLevel::MultiPlayArena => "multiplay_arena",
    };

    current.0 = match *next {
        NextLevel::None => None,
        NextLevel::Level(level) => Some(level % LEVELS),
        NextLevel::MultiPlayArena => None,
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
        &audio,
        level_slice,
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
    audio: &Res<Audio>,
    level_slice: &str,
) {
    let level_aseprite = level_aseprites.get(assets.level.id()).unwrap();
    let level_image = images.get(level_aseprite.atlas_image.id()).unwrap();
    let slice = level_aseprite.slices.get(level_slice).unwrap();

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

    let mut empties = image_to_spawn_tiles(&chunk);

    respawn_world(&mut commands, &assets, collider_query, &chunk, &world_tile);
    spawn_entities(&mut commands, &assets, &chunk);

    let player_x = TILE_SIZE * chunk.entry_point.x as f32 + TILE_HALF;
    let player_y = -TILE_SIZE * chunk.entry_point.y as f32 - TILE_HALF;

    if let Ok(mut camera) = camera.get_single_mut() {
        camera.translation.x = player_x;
        camera.translation.y = player_y;
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
            golds: 10,
            last_idle_frame_count: *frame_count,
            last_ilde_x: player_x,
            last_ilde_y: player_y,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: life,
            last_idle_max_life: max_life,
        },
        false,
        3.0,
        &audio,
    );

    if 20 < empties.len() {
        for _ in 0..10 {
            let (x, y) = random_select(&mut empties);
            spawn_slime(
                &mut commands,
                &assets,
                Vec2::new(
                    TILE_SIZE * x as f32 + TILE_HALF,
                    TILE_SIZE * -y as f32 - TILE_HALF,
                ),
                &life_bar_res,
            );
        }

        for _ in 0..10 {
            let (x, y) = random_select(&mut empties);
            spawn_eyeball(
                &mut commands,
                &assets,
                Vec2::new(
                    TILE_SIZE * x as f32 + TILE_HALF,
                    TILE_SIZE * -y as f32 - TILE_HALF,
                ),
                &life_bar_res,
            );
        }
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
                    if !chunk.equals(x as i32, y as i32 + 1, Tile::Wall) {
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
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    MagicCircleDestination::NextLevel,
                );
            }
            GameEntity::MultiPlayArenaMagicCircle => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    MagicCircleDestination::MultiplayArena,
                );
            }
            GameEntity::BrokenMagicCircle => {
                spawn_broken_magic_circle(
                    &mut commands,
                    assets.asset.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                );
            }
            GameEntity::StoneLantern => {
                spawn_stone_lantern(&mut commands, &assets, tx + TILE_HALF, ty - TILE_HALF);
            }
            GameEntity::Usage => {
                commands.spawn(AsepriteSliceBundle {
                    aseprite: assets.asset.clone(),
                    slice: "usage".into(),
                    transform: Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    sprite: Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    ..default()
                });
            }
            GameEntity::SinglePlay => {
                commands.spawn(AsepriteSliceBundle {
                    aseprite: assets.asset.clone(),
                    slice: "single_play".into(),
                    transform: Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    sprite: Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    ..default()
                });
            }
            GameEntity::MultiPlayArena => {
                commands.spawn(AsepriteSliceBundle {
                    aseprite: assets.asset.clone(),
                    slice: "multiplay_arena".into(),
                    transform: Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    sprite: Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);
        app.init_resource::<CurrentLevel>();
        app.init_resource::<NextLevel>();
    }
}
