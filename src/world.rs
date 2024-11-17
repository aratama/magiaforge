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
use crate::entity::chest::ChestType;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::magic_circle::MagicCircleDestination;
use crate::entity::stone_lantern::spawn_stone_lantern;
use crate::entity::witch::spawn_witch;
use crate::entity::GameEntity;
use crate::footsteps::Footsteps;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory_item::InventoryItem;
use crate::player_state::PlayerState;
use crate::random::random_select;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::WandType;
use crate::world::ceil::spawn_roof_tiles;
use crate::world::map::image_to_tilemap;
use crate::world::map::LevelTileMap;
use crate::world::tile::*;
use bevy::asset::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use map::image_to_spawn_tiles;
use uuid::Uuid;
use wall::spawn_wall_collisions;
use wall::WallCollider;

#[derive(Resource, Debug, Clone, Default)]
pub enum NextLevel {
    #[default]
    None,
    Level(i32, PlayerState),
    MultiPlayArena(PlayerState),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameLevel {
    Level(i32),
    MultiPlayArena,
}

/// レベルとプレイヤーキャラクターを生成します
fn setup_world(
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
    life_bar_res: Res<LifeBarResource>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    config: Res<GameConfig>,
    next: Res<NextLevel>,
    audio: Res<Audio>,
) {
    let level = match next.clone() {
        NextLevel::None => GameLevel::Level(0),
        NextLevel::Level(level, _) => GameLevel::Level(level % LEVELS),
        NextLevel::MultiPlayArena(_) => GameLevel::MultiPlayArena,
    };

    let player = match next.clone() {
        NextLevel::None => PlayerState::from_config(&config),
        NextLevel::Level(_, player) => player,
        NextLevel::MultiPlayArena(player) => player,
    };

    let mut chunk = spawn_level(
        &mut commands,
        &level_aseprites,
        &images,
        &assets,
        &life_bar_res,
        level,
    );

    let entry_point = random_select(&mut chunk.entry_points);

    let player_x = TILE_SIZE * entry_point.x as f32 + TILE_HALF;
    let player_y = -TILE_SIZE * entry_point.y as f32 - TILE_HALF;

    if let Ok(mut camera) = camera.get_single_mut() {
        camera.translation.x = player_x;
        camera.translation.y = player_y;
    }

    // プレイヤーキャラクターの魔法使いを生成
    let witch_entity = spawn_witch(
        &mut commands,
        &assets,
        Vec2::new(player_x, player_y),
        0.0,
        Uuid::new_v4(),
        None,
        player.life,
        player.max_life,
        &life_bar_res,
        false,
        3.0,
        true,
        player.wands,
        Player {
            name: player.name,
            golds: player.golds,
            last_idle_frame_count: FrameCount(0),
            last_ilde_x: player_x,
            last_ilde_y: player_y,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: player.life,
            last_idle_max_life: player.max_life,
            inventory: player.inventory,
            equipments: player.equipments,
        },
    );

    // 足音が必要なのはプレイヤーキャラクターだけなので、足音コンポーネントを追加
    let audio_instance = audio
        .play(assets.taiikukan.clone())
        .looped()
        .with_volume(0.0)
        // .with_volume(Volume::Amplitude((config.se_volume * volume) as f64))
        // .with_panning(panning as f64)
        .handle();
    commands
        .entity(witch_entity)
        .insert(Footsteps(audio_instance));
}

fn select_bgm(next: Res<NextLevel>, mut writer: EventWriter<GameCommand>) {
    if next.is_changed() {
        writer.send(match *next {
            NextLevel::None => GameCommand::BGMDokutsu,
            NextLevel::Level(0, _) => GameCommand::BGMDokutsu,
            _ => GameCommand::BGMArechi,
        });
    }
}

/// 現状は StateScopedですべてのエンティティが削除されるので以下のコードは不要ですが、
/// 今後レベルのシームレスなスポーンを実装する場合は、以下のようなコードが必要になるかも
#[allow(dead_code)]
fn despawn_level(
    commands: &mut Commands,
    collider_query: &Query<Entity, With<WallCollider>>,
    world_tile: &Query<Entity, With<WorldTile>>,
) {
    for entity in world_tile {
        commands.entity(entity).despawn_recursive();
    }
    for entity in collider_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_level(
    mut commands: &mut Commands,
    level_aseprites: &Res<Assets<Aseprite>>,
    images: &Res<Assets<Image>>,
    assets: &Res<GameAssets>,
    life_bar_res: &Res<LifeBarResource>,
    level: GameLevel,
) -> LevelTileMap {
    let level_slice = match level {
        GameLevel::Level(level) => &format!("level{}", level % LEVELS),
        GameLevel::MultiPlayArena => "multiplay_arena",
    };

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

    spawn_world_tilemap(&mut commands, &assets, &chunk);

    spawn_wall_collisions(&mut commands, &chunk);

    spawn_entities(&mut commands, &assets, &chunk);

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

    return chunk;
}

fn spawn_world_tilemap(commands: &mut Commands, assets: &Res<GameAssets>, chunk: &LevelTileMap) {
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
                            aseprite: assets.atlas.clone(),
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
                                aseprite: assets.atlas.clone(),
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
                    assets.atlas.clone(),
                    tx + TILE_SIZE,
                    ty - TILE_HALF,
                );
            }
            GameEntity::Chest => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    ChestType::Chest,
                );
            }
            GameEntity::Crate => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    ChestType::Crate,
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
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                );
            }
            GameEntity::StoneLantern => {
                spawn_stone_lantern(&mut commands, &assets, tx + TILE_HALF, ty - TILE_HALF);
            }
            GameEntity::Usage => {
                commands.spawn(AsepriteSliceBundle {
                    aseprite: assets.atlas.clone(),
                    slice: "usage".into(),
                    transform: Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    sprite: Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    ..default()
                });
            }
            GameEntity::Routes => {
                commands.spawn(AsepriteSliceBundle {
                    aseprite: assets.atlas.clone(),
                    slice: "routes".into(),
                    transform: Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    sprite: Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    ..default()
                });
            }
            GameEntity::Spell => {
                spawn_dropped_item(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    InventoryItem::Spell(SpellType::MagicBolt),
                );
            }
            GameEntity::Wand => {
                spawn_dropped_item(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    InventoryItem::Wand(WandType::CypressWand),
                );
            }
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);
        app.init_resource::<NextLevel>();
        app.add_systems(Update, select_bgm.run_if(in_state(GameState::InGame)));
    }
}
