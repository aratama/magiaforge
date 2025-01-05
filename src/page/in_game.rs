use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::camera::setup_camera;
use crate::config::GameConfig;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::ActorGroup;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::witch::spawn_witch;
use crate::equipment::EquipmentType;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::Dict;
use crate::level::appearance::spawn_level_appearance;
use crate::level::entities::spawn_entity;
use crate::level::entities::SpawnEnemyType;
use crate::level::entities::SpawnEntity;
use crate::level::map::image_to_spawn_tiles;
use crate::level::map::LevelChunk;
use crate::level::wall::spawn_wall_collisions;
use crate::message::LEVEL0;
use crate::message::LEVEL1;
use crate::message::LEVEL2;
use crate::message::LEVEL3;
use crate::message::LEVEL4;
use crate::message::MULTIPLAY_ARENA;
use crate::message::UNKNOWN_LEVEL;
use crate::player_state::PlayerState;
use crate::spell::SpellType;
use crate::states::GameMenuState;
use crate::states::GameState;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;
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
pub struct LevelSetup {
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

    /// 次に生成するショップアイテムのキュー
    /// これが空になったときは改めてキューを生成します
    pub shop_items: Vec<InventoryItem>,
}

impl Default for LevelSetup {
    fn default() -> Self {
        LevelSetup {
            level: None,
            chunk: None,
            next_level: GameLevel::Level(INITIAL_LEVEL),
            next_state: PlayerState::default(),
            shop_items: Vec::new(),
        }
    }
}

pub fn new_shop_item_queue(discovered_spells: Vec<SpellType>) -> Vec<InventoryItem> {
    let mut rng = rand::thread_rng();

    let mut shop_items: Vec<InventoryItem> = discovered_spells
        .iter()
        .map(|s| InventoryItem {
            item_type: InventoryItemType::Spell(*s),
            price: s.to_props().price,
        })
        .collect();
    shop_items.extend(
        EquipmentType::iter()
            .filter(|e| e.to_props().rank == 0)
            .map(|e| InventoryItem {
                item_type: InventoryItemType::Equipment(e),
                price: e.to_props().price,
            }),
    );

    shop_items.shuffle(&mut rng);

    shop_items
}

/// レベルとプレイヤーキャラクターを生成します
pub fn setup_level(
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
    life_bar_res: Res<LifeBarResource>,
    mut current: ResMut<LevelSetup>,
    config: Res<GameConfig>,
    mut spawn: EventWriter<SpawnEntity>,
    mut next: ResMut<NextState<GameMenuState>>,
) {
    let mut rng = StdRng::from_entropy();

    let level = current.next_level;
    current.level = Some(current.next_level);

    // 拠点のみ最初にアニメーションが入るので PlayerInActive に設定します
    if level == GameLevel::Level(0) {
        next.set(GameMenuState::PlayerInActive);
    }

    // レベルの外観を生成します
    let chunk = spawn_level_appearance(
        &mut commands,
        &level_aseprites,
        &images,
        &assets,
        level,
        &mut rng,
    );

    // エントリーポイントを選択
    // プレイヤーはここに配置し、この周囲はセーフゾーンとなって敵モブやアイテムは生成しません
    let entry_point = chunk
        .entry_points
        .choose(&mut rng)
        .expect("No entrypoint found");

    let mut player_state = current.next_state.clone();

    player_state.name = config.player_name.clone();
    let player_x = TILE_SIZE * entry_point.0 as f32 + TILE_HALF;
    let player_y = -TILE_SIZE * entry_point.1 as f32 - TILE_HALF;

    // 拠点に戻ってきたときは全回復します
    if level == GameLevel::Level(0) {
        player_state.life = player_state.max_life;
    }

    // レベルのコリジョンを生成します
    spawn_wall_collisions(&mut commands, &chunk);

    // 宝箱や灯篭などのエンティティを生成します
    for entity in &chunk.entities {
        spawn.send(entity.clone());
    }

    // 空間
    // ここに敵モブや落ちているアイテムを生成します
    let empties = image_to_spawn_tiles(&chunk);

    // 空いた空間に敵モブキャラクターをランダムに生成します
    let spaw_enemy_count = match level {
        GameLevel::Level(0) => 0,
        GameLevel::Level(1) => 10,
        GameLevel::Level(2) => 10,
        GameLevel::Level(3) => 10,
        GameLevel::Level(4) => 10,
        GameLevel::Level(5) => 10,
        GameLevel::Level(6) => 0, // ボス部屋
        GameLevel::MultiPlayArena => 0,
        _ => 0,
    };
    let spaw_enemy_types = match level {
        GameLevel::Level(0) => vec![],
        GameLevel::Level(1) => vec![SpawnEnemyType::Slime],
        GameLevel::Level(2) => vec![SpawnEnemyType::Slime, SpawnEnemyType::Spider],
        GameLevel::Level(3) => vec![SpawnEnemyType::Spider, SpawnEnemyType::Eyeball],
        GameLevel::Level(4) => vec![SpawnEnemyType::Eyeball, SpawnEnemyType::Shadow],
        GameLevel::Level(5) => vec![SpawnEnemyType::Shadow, SpawnEnemyType::Salamander],
        GameLevel::Level(6) => vec![], // ボス部屋
        GameLevel::MultiPlayArena => vec![],
        _ => vec![],
    };
    spawn_random_enemies(
        &empties,
        &mut rng,
        entry_point.clone(),
        spaw_enemy_count,
        &spaw_enemy_types,
        &mut spawn,
    );

    spawn_dropped_items(
        &mut commands,
        &assets,
        &empties,
        &mut rng,
        entry_point.clone(),
        match level {
            GameLevel::Level(1) => 3,
            GameLevel::Level(2) => 3,
            GameLevel::Level(3) => 3,
            GameLevel::Level(4) => 3,
            GameLevel::Level(5) => 3,
            _ => 0,
        },
        match level {
            GameLevel::Level(1) => vec![0, 1, 2],
            GameLevel::Level(2) => vec![1, 2, 3],
            GameLevel::Level(3) => vec![2, 3, 4],
            GameLevel::Level(4) => vec![3, 4, 5],
            GameLevel::Level(5) => vec![4, 5, 6],
            _ => vec![],
        },
    );

    // 拠点のみ、数羽のニワトリを生成します
    if level == GameLevel::Level(0) {
        for _ in 0..4 {
            if let Some((x, y)) = empties.choose(&mut rng) {
                spawn.send(SpawnEntity::Chiken {
                    position: Vec2::new(TILE_SIZE * *x as f32, TILE_SIZE * -*y as f32),
                });
            }
        }
    }

    // テスト用モンスター
    // spawn.send(SpawnEntity::Enemy {
    //     enemy_type: SpawnEnemyType::Salamander,
    //     position: Vec2::new(TILE_SIZE * 24 as f32, TILE_SIZE * -34 as f32),
    // });

    // プレイヤーを生成します
    // まずはエントリーポイントをランダムに選択します

    setup_camera(&mut commands, Vec2::new(player_x, player_y));

    // プレイヤーキャラクターの魔法使いを生成
    spawn_witch(
        &mut commands,
        &assets,
        Vec2::new(player_x, player_y),
        0.0,
        Uuid::new_v4(),
        None,
        player_state.life,
        player_state.max_life,
        &life_bar_res,
        false,
        3.0,
        player_state.golds,
        player_state.wands,
        player_state.inventory,
        player_state.equipments,
        Player::new(player_state.name, level == GameLevel::Level(0)),
        ActorGroup::Player,
        player_state.current_wand as usize,
    );

    current.chunk = Some(chunk);
}

fn select_level_bgm(
    next_level: Res<LevelSetup>,
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

fn spawn_random_enemies(
    empties: &Vec<(i32, i32)>,
    mut rng: &mut StdRng,
    safe_zone_center: (i32, i32),
    spaw_enemy_count: u32,
    enemy_types: &Vec<SpawnEnemyType>,
    spawn: &mut EventWriter<SpawnEntity>,
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
            Some(enemy_type) => {
                spawn.send(SpawnEntity::Enemy {
                    enemy_type: *enemy_type,
                    position,
                });
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
    ranks: Vec<u32>,
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

        if let Some(spell) = SpellType::iter()
            .filter(|i| {
                let props = i.to_props();
                ranks.contains(&props.rank)
            })
            .choose(&mut rng)
        {
            spawn_dropped_item(
                &mut commands,
                &assets,
                position,
                InventoryItem {
                    item_type: InventoryItemType::Spell(spell),
                    price: 0,
                },
            );
        } else {
            warn!("No spell found to spawn as dropped item");
        }

        items += 1;
    }
}

pub fn level_to_name(level: GameLevel) -> Dict<&'static str> {
    match level {
        GameLevel::Level(0) => LEVEL0,
        GameLevel::Level(1) => LEVEL1,
        GameLevel::Level(2) => LEVEL2,
        GameLevel::Level(3) => LEVEL3,
        GameLevel::Level(4) => LEVEL4,
        GameLevel::MultiPlayArena => MULTIPLAY_ARENA,
        _ => UNKNOWN_LEVEL,
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEntity>();
        app.add_systems(
            FixedUpdate,
            spawn_entity
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );

        app.add_systems(OnEnter(GameState::InGame), setup_level);
        app.add_systems(OnEnter(GameState::InGame), select_level_bgm);
        app.init_resource::<LevelSetup>();
    }
}
