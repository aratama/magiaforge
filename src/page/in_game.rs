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
use crate::entity::get_entity_z;
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
use crate::level::biome::Biome;
use crate::level::ceil::spawn_roof_tiles;
use crate::level::map::image_to_spawn_tiles;
use crate::level::map::image_to_tilemap;
use crate::level::map::LevelChunk;
use crate::level::tile::*;
use crate::level::wall::spawn_wall_collisions;
use crate::message::HELLO;
use crate::message::HELLO_RABBITS;
use crate::message::HUGE_SLIME;
use crate::message::HUGE_SLIME2;
use crate::message::HUGE_SLIME3;
use crate::message::HUGE_SLIME4;
use crate::message::HUGE_SLIME5;
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
use crate::spell::SpellType;
use crate::states::GameState;
use crate::theater::Act;
use bevy::asset::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameLevel {
    Level(i32),
    MultiPlayArena,
}

/// 現在のレベル、次のレベル、次のレベルでのプレイヤーキャラクターの状態など、
/// レベル間を移動するときの情報を保持します
#[derive(Resource, Debug, Clone)]
pub struct Interlevel {
    /// 現在プレイ中のレベル
    pub level: Option<GameLevel>,

    /// 現在プレイ中のレベルのマップ構造情報
    pub chunk: Option<LevelChunk>,

    /// 次のレベル
    /// 魔法陣から転移するとこのレベルに移動します
    pub next_level: GameLevel,

    /// 次のプレイヤー状態
    /// 魔法陣から転移したとき、この状態でプレイヤーを初期化します
    pub next_state: PlayerState,
}

impl Default for Interlevel {
    fn default() -> Self {
        Interlevel {
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
    mut current: ResMut<Interlevel>,
    config: Res<GameConfig>,
) {
    let mut rng = StdRng::from_entropy();

    let level = current.next_level;
    current.level = Some(current.next_level);

    // レベルの外観を生成します
    let chunk = spawn_level_appearance(
        &mut commands,
        &level_aseprites,
        &images,
        &assets,
        level,
        &mut rng,
    );

    // レベルのコリジョンを生成します
    spawn_wall_collisions(&mut commands, &chunk);

    // 宝箱や灯篭などのエンティティを生成します
    spawn_entities(&mut commands, &assets, &life_bar_res, &chunk);

    // エントリーポイントを選択
    // プレイヤーはここに配置し、この周囲はセーフゾーンとなって敵モブやアイテムは生成しません
    let entry_point = chunk
        .entry_points
        .choose(&mut rng)
        .expect("No entrypoint found");

    // 空間
    // ここに敵モブや落ちているアイテムを生成します
    let empties = image_to_spawn_tiles(&chunk);

    // 空いた空間に敵モブキャラクターをランダムに生成します
    let spaw_enemy_count = match level {
        GameLevel::Level(0) => 0,
        GameLevel::Level(1) => 10,
        GameLevel::Level(4) => 0, // ボス部屋
        GameLevel::MultiPlayArena => 0,
        _ => 30,
    };
    let spaw_enemy_types = match level {
        GameLevel::Level(0) => vec![],
        GameLevel::Level(1) => vec![SpawnEnemyType::Slime],
        GameLevel::Level(4) => vec![], // ボス部屋
        GameLevel::MultiPlayArena => vec![],
        _ => vec![SpawnEnemyType::Slime, SpawnEnemyType::Eyeball],
    };
    spawn_random_enemies(
        &mut commands,
        &assets,
        &life_bar_res,
        &empties,
        &mut rng,
        entry_point.clone(),
        spaw_enemy_count,
        &spaw_enemy_types,
    );

    let spaw_item_count = match level {
        GameLevel::Level(0) => 0,
        GameLevel::Level(4) => 0, // ボス部屋
        GameLevel::MultiPlayArena => 0,
        _ => 3,
    };
    spawn_dropped_items(
        &mut commands,
        &assets,
        &empties,
        &mut rng,
        entry_point.clone(),
        spaw_item_count,
    );

    // プレイヤーを生成します
    // まずはエントリーポイントをランダムに選択します

    let mut player = current.next_state.clone();
    player.name = config.player_name.clone();
    let player_x = TILE_SIZE * entry_point.0 as f32 + TILE_HALF;
    let player_y = -TILE_SIZE * entry_point.1 as f32 - TILE_HALF;

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
            getting_up: if level == GameLevel::Level(0) { 240 } else { 0 },
        },
        ActorGroup::Player,
        player.current_wand,
    );

    current.chunk = Some(chunk);
}

fn select_level_bgm(
    next_level: Res<Interlevel>,
    mut next_bgm: ResMut<NextBGM>,
    assets: Res<GameAssets>,
) {
    if next_level.is_changed() {
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

/// 床や壁の外観(スプライト)を生成します
fn spawn_level_appearance(
    mut commands: &mut Commands,
    level_aseprites: &Res<Assets<Aseprite>>,
    images: &Res<Assets<Image>>,
    assets: &Res<GameAssets>,
    level: GameLevel,
    mut rng: &mut StdRng,
) -> LevelChunk {
    let level_aseprite = level_aseprites.get(assets.level.id()).unwrap();
    let level_image = images.get(level_aseprite.atlas_image.id()).unwrap();

    let level_slice = match level {
        GameLevel::Level(level) => {
            let keys = level_aseprite
                .slices
                .keys()
                .filter(|s| s.starts_with(&format!("level_{}_", level)));
            keys.choose(&mut rng).unwrap()
        }
        GameLevel::MultiPlayArena => "multiplay_arena",
    };

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

    spawn_world_tilemap(
        &mut commands,
        &assets,
        &chunk,
        // バイオームをハードコーディングしているけどこれでいい？
        match level {
            GameLevel::Level(2) => Biome::Grassland,
            _ => Biome::StoneTile,
        },
    );

    return chunk;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnEnemyType {
    Slime,
    Eyeball,
}

fn spawn_random_enemies(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_res: &Res<LifeBarResource>,
    empties: &Vec<(i32, i32)>,
    mut rng: &mut StdRng,
    safe_zone_center: (i32, i32),
    spaw_enemy_count: u32,
    enemy_types: &Vec<SpawnEnemyType>,
) {
    let mut empties = empties.clone();
    empties.shuffle(&mut rng);

    let mut enemies = 0;

    for (x, y) in empties {
        if spaw_enemy_count <= enemies {
            break;
        }

        if Vec2::new(safe_zone_center.0 as f32, safe_zone_center.1 as f32)
            .distance(Vec2::new(x as f32, y as f32))
            < 8.0
        {
            continue;
        }

        let position = Vec2::new(
            TILE_SIZE * x as f32 + TILE_HALF,
            TILE_SIZE * -y as f32 - TILE_HALF,
        );

        match enemy_types.choose(&mut rng) {
            Some(SpawnEnemyType::Slime) => {
                spawn_slime(
                    &mut commands,
                    &assets,
                    position,
                    &life_bar_res,
                    0,
                    5,
                    ActorGroup::Enemy,
                    None,
                );
            }
            Some(SpawnEnemyType::Eyeball) => {
                spawn_eyeball(
                    &mut commands,
                    &assets,
                    position,
                    &life_bar_res,
                    ActorGroup::Enemy,
                    8,
                );
            }
            None => {
                warn!("No enemy type found");
            }
        }

        enemies += 1;
    }
}

fn spawn_dropped_items(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    empties: &Vec<(i32, i32)>,
    mut rng: &mut StdRng,
    safe_zone_center: (i32, i32),
    spaw_item_count: u32,
) {
    let mut empties = empties.clone();
    empties.shuffle(&mut rng);

    let mut items = 0;

    for (x, y) in empties {
        if spaw_item_count <= items {
            break;
        }

        if Vec2::new(safe_zone_center.0 as f32, safe_zone_center.1 as f32)
            .distance(Vec2::new(x as f32, y as f32))
            < 16.0
        {
            continue;
        }

        let position = Vec2::new(
            TILE_SIZE * x as f32 + TILE_HALF,
            TILE_SIZE * -y as f32 - TILE_HALF,
        );

        let spell = SpellType::iter().choose(&mut rng).unwrap();
        spawn_dropped_item(
            &mut commands,
            &assets,
            position,
            InventoryItem {
                item_type: InventoryItemType::Spell(spell),
                price: 0,
            },
        );

        items += 1;
    }
}

fn spawn_world_tilemap(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk: &LevelChunk,
    biome: Biome,
) {
    // 床と壁の生成
    for y in chunk.min_y..chunk.max_y as i32 {
        for x in chunk.min_x..chunk.max_x as i32 {
            match chunk.get_tile(x, y) {
                Tile::Biome => match biome {
                    Biome::StoneTile => {
                        spawn_stone_tile(commands, assets, x, y);
                    }
                    Biome::Grassland => {
                        spawn_grassland(commands, &assets, x, y);
                    }
                },
                Tile::StoneTile => {
                    spawn_stone_tile(commands, assets, x, y);
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
                Tile::Grassland => {
                    spawn_grassland(commands, &assets, x, y);
                }
                Tile::Blank => {}
            }
        }
    }
}

fn spawn_stone_tile(commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
    let r = rand::random::<u32>() % 3;
    let slice = format!("stone_tile{}", r);
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

fn spawn_grassland(commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
    commands.spawn((
        WorldTile,
        Name::new("grassland"),
        StateScoped(GameState::InGame),
        Transform::from_translation(Vec3::new(
            x as f32 * TILE_SIZE,
            y as f32 * -TILE_SIZE,
            FLOOR_LAYER_Z,
        )),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "grassland".into(),
        },
    ));

    for i in 0..3 {
        let x = x as f32 * TILE_SIZE;
        let y = y as f32 * -TILE_SIZE + 5.0 * i as f32;
        commands.spawn((
            WorldTile,
            Name::new("grass"),
            StateScoped(GameState::InGame),
            Transform::from_translation(Vec3::new(x, y, get_entity_z(y) + 0.01)),
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: format!("grass_{}", rand::random::<u32>() % 3).into(),
            },
        ));
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
                if rand::random::<u32>() % 4 != 0 {
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
                if 0.2 < rand::random::<f32>() {
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
                        messages: vec![Act::Speech(TRAINING_RABBIT.to_string())],
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
                        messages: vec![Act::Speech(SINGLEPLAY.to_string())],
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
                        messages: vec![
                            Act::BGM(Some(assets.saihate.clone())),
                            Act::Speech(HELLO.to_string()),
                            Act::Speech(HELLO_RABBITS.to_string()),
                        ],
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
                        messages: vec![Act::Speech(MULTIPLAY.to_string())],
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
                        messages: vec![
                            Act::Speech(WITCHES_ARE.to_string()),
                            Act::Speech(HUGE_SLIME.to_string()),
                            Act::Speech(HUGE_SLIME2.to_string()),
                            Act::Speech(HUGE_SLIME3.to_string()),
                            Act::Speech(HUGE_SLIME4.to_string()),
                            Act::Speech(HUGE_SLIME5.to_string()),
                        ],
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
        app.init_resource::<Interlevel>();
    }
}
