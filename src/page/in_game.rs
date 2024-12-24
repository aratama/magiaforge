use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::camera::setup_camera;
use crate::config::GameConfig;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::player::Player;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::sandbug::spawn_sandbag;
use crate::enemy::slime::spawn_slime;
use crate::entity::actor::ActorGroup;
use crate::entity::bgm::spawn_bgm_switch;
use crate::entity::book_shelf::spawn_book_shelf;
use crate::entity::broken_magic_circle::spawn_broken_magic_circle;
use crate::entity::chest::spawn_chest;
use crate::entity::chest::ChestType;
use crate::entity::chest::CHEST_OR_BARREL;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::magic_circle::MagicCircleDestination;
use crate::entity::rabbit::spawn_rabbit;
use crate::entity::shop::spawn_shop_door;
use crate::entity::stone_lantern::spawn_stone_lantern;
use crate::entity::witch::spawn_witch;
use crate::entity::GameEntity;
use crate::equipment::EquipmentType;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::Dict;
use crate::level::ceil::spawn_roof_tiles;
use crate::level::map::image_to_spawn_tiles;
use crate::level::map::image_to_tilemap;
use crate::level::map::LevelChunk;
use crate::level::tile::*;
use crate::level::wall::spawn_wall_collisions;
use crate::level::wall::WallCollider;
use crate::message::HELLO;
use crate::message::LEVEL0;
use crate::message::LEVEL1;
use crate::message::LEVEL2;
use crate::message::LEVEL3;
use crate::message::MULTIPLAY;
use crate::message::MULTIPLAY_ARENA;
use crate::message::SINGLEPLAY;
use crate::message::TRAINING_RABBIT;
use crate::message::UNKNOWN_LEVEL;
use crate::message::WITCHES_ARE;
use crate::player_state::PlayerState;
use crate::random::random_select_mut;
use crate::spell::SpellType;
use crate::states::GameState;
use bevy::asset::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameLevel {
    Level(i32),
    MultiPlayArena,
}

#[derive(Resource, Debug, Clone)]
pub struct CurrentLevel {
    pub level: Option<GameLevel>,
    pub chunk: Option<LevelChunk>,
    pub next_level: GameLevel,
    pub next_state: PlayerState,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        CurrentLevel {
            level: None,
            chunk: None,
            next_level: GameLevel::Level(INITIAL_LEVEL),
            next_state: PlayerState::from_config(&GameConfig::default()),
        }
    }
}

/// レベルとプレイヤーキャラクターを生成します
pub fn setup_level(
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
    life_bar_res: Res<LifeBarResource>,
    mut current: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
) {
    let level = match current.next_level {
        GameLevel::Level(level) => GameLevel::Level(level % LEVELS),
        GameLevel::MultiPlayArena => GameLevel::MultiPlayArena,
    };

    let mut player = current.next_state.clone();
    player.name = config.player_name.clone();

    let mut chunk = spawn_level(
        &mut commands,
        &level_aseprites,
        &images,
        &assets,
        &life_bar_res,
        level,
    );

    let entry_point = random_select_mut(&mut chunk.entry_points);

    let player_x = TILE_SIZE * entry_point.x as f32 + TILE_HALF;
    let player_y = -TILE_SIZE * entry_point.y as f32 - TILE_HALF;

    setup_camera(&mut commands, Vec2::new(player_x, player_y));

    // プレイヤーキャラクターの魔法使いを生成
    spawn_witch(
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
        player.golds,
        player.wands,
        player.inventory,
        player.equipments,
        Player {
            name: player.name,
            last_idle_frame_count: FrameCount(0),
            last_ilde_x: player_x,
            last_ilde_y: player_y,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: player.life,
            last_idle_max_life: player.max_life,
        },
        ActorGroup::Player,
        player.current_wand,
    );

    current.level = Some(level);
    current.chunk = Some(chunk);
}

fn select_level_bgm(
    next_level: Res<CurrentLevel>,
    mut next_bgm: ResMut<NextBGM>,
    assets: Res<GameAssets>,
) {
    if next_level.is_changed() {
        info!("select_level_bgm {:?}", next_level.next_level);
        *next_bgm = NextBGM(Some(match next_level.next_level {
            GameLevel::Level(0) => assets.dokutsu.clone(),
            GameLevel::Level(LAST_BOSS_LEVEL) => {
                let mut rng = rand::thread_rng();
                let mut bgms = vec![
                    assets.deamon.clone(),
                    assets.action.clone(),
                    assets.decisive.clone(),
                    assets.enjin.clone(),
                    // assets.sacred.clone(), // ボスのプロモート後用BGM
                    assets.battle_cinematic.clone(),
                    assets.battle_fight.clone(),
                    assets.final_battle.clone(),
                    assets.human_vs_machine.clone(),
                ];
                bgms.shuffle(&mut rng);
                bgms.pop().unwrap()
            }
            _ => {
                let mut rng = rand::thread_rng();
                let mut bgms = vec![
                    assets.arechi.clone(),
                    assets.touha.clone(),
                    assets.mori.clone(),
                    assets.meikyu.clone(),
                    assets.shiden.clone(),
                    assets.midnight_forest.clone(),
                ];
                bgms.shuffle(&mut rng);
                bgms.pop().unwrap()
            }
        }));
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
) -> LevelChunk {
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

    spawn_entities(&mut commands, &assets, &life_bar_res, &chunk);

    if 30 < empties.len() {
        for _ in 0..20 {
            let (x, y) = random_select_mut(&mut empties);
            if level != GameLevel::Level(1) && rand::random::<usize>() % 2 == 0 {
                spawn_eyeball(
                    &mut commands,
                    &assets,
                    Vec2::new(
                        TILE_SIZE * x as f32 + TILE_HALF,
                        TILE_SIZE * -y as f32 - TILE_HALF,
                    ),
                    &life_bar_res,
                    ActorGroup::Enemy,
                    8,
                );
            } else {
                spawn_slime(
                    &mut commands,
                    &assets,
                    Vec2::new(
                        TILE_SIZE * x as f32 + TILE_HALF,
                        TILE_SIZE * -y as f32 - TILE_HALF,
                    ),
                    &life_bar_res,
                    0,
                    5,
                    ActorGroup::Enemy,
                    None,
                );
            }
        }

        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let (x, y) = random_select_mut(&mut empties);
            let spell = SpellType::iter().choose(&mut rng).unwrap();
            spawn_dropped_item(
                &mut commands,
                &assets,
                Vec2::new(
                    TILE_SIZE * x as f32 + TILE_HALF,
                    TILE_SIZE * -y as f32 - TILE_HALF,
                ),
                InventoryItem {
                    item_type: InventoryItemType::Spell(spell),
                    price: 0,
                },
            );
        }
    }

    return chunk;
}

fn spawn_world_tilemap(commands: &mut Commands, assets: &Res<GameAssets>, chunk: &LevelChunk) {
    // 床と壁の生成
    for y in chunk.min_y..chunk.max_y as i32 {
        for x in chunk.min_x..chunk.max_x as i32 {
            let r = rand::random::<u32>() % 3;
            let slice = format!("stone_tile{}", r);

            match chunk.get_tile(x, y) {
                Tile::StoneTile => {
                    commands.spawn((
                        WorldTile,
                        Name::new("stone_tile"),
                        StateScoped(GameState::InGame),
                        Transform::from_translation(Vec3::new(
                            x as f32 * TILE_SIZE,
                            y as f32 * -TILE_SIZE,
                            FLOOR_LAYER_Z,
                        )),
                        AseSpriteSlice {
                            aseprite: assets.atlas.clone(),
                            name: slice.into(),
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
                            Transform::from_translation(Vec3::new(tx, ty - TILE_HALF, tz)),
                            AseSpriteSlice {
                                aseprite: assets.atlas.clone(),
                                name: "stone wall".into(),
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

fn spawn_entities(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_resource: &Res<LifeBarResource>,
    chunk: &LevelChunk,
) {
    let mut rng = rand::thread_rng();

    let mut dropped_spells: Vec<SpellType> = SpellType::iter().collect();
    dropped_spells.shuffle(&mut rng);

    let mut equipments: Vec<EquipmentType> = EquipmentType::iter().collect();
    equipments.shuffle(&mut rng);

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
            GameEntity::CrateOrBarrel => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    *CHEST_OR_BARREL
                        .iter()
                        .choose(&mut rand::thread_rng())
                        .unwrap(),
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
            GameEntity::MagicCircleHome => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    MagicCircleDestination::Home,
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
                commands.spawn((
                    Name::new("usage"),
                    Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    AseSpriteSlice {
                        aseprite: assets.atlas.clone(),
                        name: "usage".into(),
                    },
                ));
            }
            GameEntity::Routes => {
                commands.spawn((
                    Name::new("routes"),
                    Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
                    Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    AseSpriteSlice {
                        aseprite: assets.atlas.clone(),
                        name: "routes".into(),
                    },
                ));
            }
            GameEntity::Spell => {
                if 0.5 < rand::random::<f32>() {
                    let spell = dropped_spells.pop().unwrap_or(SpellType::MagicBolt);
                    let props = spell.to_props();
                    spawn_dropped_item(
                        &mut commands,
                        &assets,
                        Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        InventoryItem {
                            item_type: InventoryItemType::Spell(spell),
                            price: props.price,
                        },
                    );
                } else {
                    let equipment = equipments.pop().unwrap_or(EquipmentType::Lantern);
                    let props = equipment.to_props();
                    spawn_dropped_item(
                        &mut commands,
                        &assets,
                        Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        InventoryItem {
                            item_type: InventoryItemType::Equipment(equipment),
                            price: props.price,
                        },
                    );
                }
            }
            GameEntity::HugeSlime => {
                spawn_huge_slime(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                );
            }
            GameEntity::ShopRabbit => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_yellow,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    ShopRabbit,
                    ShopRabbitSensor,
                    ShopRabbitOuterSensor,
                );
            }
            GameEntity::TrainingRabbit => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_red,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    MessageRabbit {
                        message: TRAINING_RABBIT.to_string(),
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            GameEntity::SinglePlayRabbit => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_white,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    MessageRabbit {
                        message: SINGLEPLAY.to_string(),
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            GameEntity::GuideRabbit => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_blue,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    MessageRabbit {
                        message: HELLO.to_string(),
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            GameEntity::MultiplayerRabbit => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_black,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    MessageRabbit {
                        message: MULTIPLAY.to_string(),
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            GameEntity::ReadingRabbit => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_green,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    MessageRabbit {
                        message: WITCHES_ARE.to_string(),
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            GameEntity::Sandbug => {
                spawn_sandbag(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    life_bar_resource,
                );
            }
            GameEntity::ShopDoor => {
                spawn_shop_door(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                );
            }
            GameEntity::BGM => {
                spawn_bgm_switch(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                );
            }
        }
    }
}

pub fn level_to_name(level: GameLevel) -> Dict<&'static str> {
    match level {
        GameLevel::Level(0) => LEVEL0,
        GameLevel::Level(1) => LEVEL1,
        GameLevel::Level(2) => LEVEL2,
        GameLevel::Level(3) => LEVEL3,
        GameLevel::MultiPlayArena => MULTIPLAY_ARENA,
        _ => UNKNOWN_LEVEL,
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_level);
        app.add_systems(OnEnter(GameState::InGame), select_level_bgm);
        app.init_resource::<CurrentLevel>();
    }
}
