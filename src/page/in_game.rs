use crate::actor::spawn_actor;
use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::audio::NextBGM;
use crate::camera::setup_camera;
use crate::config::GameConfig;
use crate::constant::*;
use crate::controller::player::Player;
use crate::controller::player::PlayerControlled;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::hud::overlay::OverlayEvent;
use crate::inventory::Inventory;
use crate::inventory::InventoryItem;
use crate::ldtk::loader::LDTK;
use crate::level::appearance::spawn_world_tile;
use crate::level::appearance::spawn_world_tilemap;
use crate::level::appearance::TileSprite;
use crate::level::chunk::index_to_position;
use crate::level::chunk::LevelChunk;
use crate::level::collision::spawn_wall_collisions;
use crate::level::collision::WallCollider;
use crate::level::entities::spawn_entity;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::level::spawn::spawn_dropped_items;
use crate::level::spawn::spawn_navigation_mesh;
use crate::level::spawn::spawn_random_enemies;
use crate::level::tile::Tile;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::set::FixedUpdateAfterAll;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::Spell;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::asset::*;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde::Deserialize;
use std::collections::HashMap;
use vleue_navigator::prelude::NavMeshSettings;
use vleue_navigator::prelude::NavMeshUpdateMode;
use vleue_navigator::Triangulation;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct GameLevel(pub String);

impl GameLevel {
    pub fn new<T: Into<String>>(level: T) -> Self {
        GameLevel(level.into())
    }
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
    pub next_state: Option<PlayerState>,

    /// 次に生成するショップアイテムのキュー
    /// これが空になったときは改めてキューを生成します
    pub shop_items: Vec<InventoryItem>,
}

impl Default for LevelSetup {
    fn default() -> Self {
        LevelSetup {
            level: None,
            chunk: None,
            next_level: GameLevel::new(HOME_LEVEL),
            next_state: None,
            shop_items: Vec::new(),
        }
    }
}

pub fn new_shop_item_queue(
    registry: &Registry,
    discovered_spells: Vec<Spell>,
) -> Vec<InventoryItem> {
    let mut rng = rand::thread_rng();

    let mut shop_items: Vec<InventoryItem> = registry
        .spells()
        .iter()
        .filter(|s| discovered_spells.contains(&s) || registry.get_spell_props(*s).rank <= 1)
        .map(|s| InventoryItem {
            spell: s.clone(),
            price: registry.get_spell_props(s).price,
        })
        .collect();

    shop_items.shuffle(&mut rng);

    shop_items
}

/// レベルとプレイヤーキャラクターを生成します
pub fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Registry,
    ldtk_assets: Res<Assets<LDTK>>,
    mut current: ResMut<LevelSetup>,
    config: Res<GameConfig>,
    mut next_bgm: ResMut<NextBGM>,
    mut spawn: EventWriter<SpawnEvent>,
    mut overlay: EventWriter<OverlayEvent>,
) {
    let ldtk = ldtk_assets.get(registry.assets.ldtk_level.id()).unwrap();

    // 各種変数の初期化

    let game_registry = registry.game();

    let mut rng = StdRng::from_entropy();

    // プレイヤーの状態の復元 //////////////////////////////////////////////////////////////
    let mut player_state = current
        .next_state
        .clone()
        .unwrap_or(if cfg!(feature = "item") {
            PlayerState {
                inventory: Inventory::from_vec(game_registry.debug_items.clone()),
                wands: Wand::from_vec(&game_registry.debug_wands),
                ..default()
            }
        } else {
            PlayerState::default()
        });

    player_state.name = config.player_name.clone();
    player_state.update_discovered_spell();

    // 次のレベルの選定 /////////////////////////////////////////////////////////////////////

    let level = if player_state.discovered_spells.is_empty() {
        GameLevel::new("Warehouse")
    } else {
        current.next_level.clone()
    };

    current.level = Some(level.clone());
    current.next_level = level.clone();

    // bgm ////////////////////////////////////////////////////////////////////////////////

    let props = registry.get_level(&level);
    *next_bgm = NextBGM(Some(asset_server.load(props.bgm.clone())));

    // レベルの生成 ///////////////////////////////////////////////////////////////////////////

    // 拠点に戻ってきたときは全回復します
    if level == GameLevel::new(HOME_LEVEL) {
        player_state.life = player_state.max_life;
    }

    // 拠点のみ最初にアニメーションが入るので PlayerInActive に設定します
    let getting_up_animation =
        level == GameLevel::new("Warehouse") && cfg!(not(feature = "ingame"));

    let chunk = spawn_level(&mut commands, &registry, &ldtk, &level);

    // for neighbor in ldtk.get_neighbors(&level).iter() {
    //     spawn_level(&mut commands, &registry, &ldtk, &neighbor);
    // }

    // エンティティ生成 /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // 宝箱や灯篭などのエンティティを生成します
    for ((x, y), props) in chunk.entities.iter() {
        spawn.send(SpawnEvent {
            position: index_to_position((*x, *y)) + Vec2::new(props.spawn_offset_x, 0.0),
            spawn: props.entity.clone(),
        });
    }

    // 落ちている呪文を生成
    spawn_dropped_items(&mut commands, &registry, &props.items);

    // そのほかのエンティティを生成
    for ((x, y), entity) in props.spawn.iter() {
        let position = index_to_position((*x, *y));
        spawn.send(SpawnEvent {
            position,
            spawn: entity.clone(),
        });
    }

    // 地雷原テスト実装
    if level == GameLevel::new("minefield") {
        for y in 0..chunk.max_y {
            for x in 0..chunk.max_x {
                let tile = chunk.get_tile(x, y);
                if *tile == Tile::new("Soil") || *tile == Tile::new("Grassland") {
                    if rand::random::<u32>() % 20 == 0 {
                        spawn.send(SpawnEvent {
                            position: index_to_position((x, y)),
                            spawn: Spawn::Actor(ActorType::new("ExplosiveMashroom")),
                        });
                    }
                }
            }
        }
    }

    // テスト用モンスター
    // spawn.send(SpawnEvent {
    //     position: index_to_position((29, 52)),
    //     spawn: Spawn::Actor {
    //         actor_type: ActorType::new("Salamander"),
    //         actor_group: ActorGroup::Enemy,
    //     },
    // });

    // エントリーポイントを選択
    // プレイヤーはここに配置し、この周囲はセーフゾーンとなって敵モブやアイテムは生成しません
    let entry_points = chunk.entry_points();
    let entry_point = entry_points.choose(&mut rng).expect("No entrypoint found");

    info!("entry_points {:?}", entry_points);

    let player_x = TILE_SIZE * entry_point.0 as f32 + TILE_HALF;
    let player_y = -TILE_SIZE * entry_point.1 as f32 - TILE_HALF;

    // 空間
    // ここに敵モブや落ちているアイテムを生成します
    let empties = chunk.get_spawn_tiles(&registry);

    // 空いた空間に敵モブキャラクターをランダムに生成します
    let props = registry.get_level(&level);
    let spaw_enemy_count = props.enemies;
    let spaw_enemy_types = props.enemy_types.clone();
    spawn_random_enemies(
        &empties,
        &mut rng,
        entry_point.clone(),
        spaw_enemy_count,
        &spaw_enemy_types,
        &mut spawn,
    );

    // 拠点のみ、数羽のニワトリを生成します
    if level == GameLevel::new(HOME_LEVEL) {
        for _ in 0..5 {
            if let Some((x, y)) = empties.choose(&mut rng) {
                spawn.send(SpawnEvent {
                    position: index_to_position((*x, *y)),
                    spawn: Spawn::Actor(ActorType::new("Chicken")),
                });
            }
        }
    }
    // プレイヤーを生成します
    // まずはエントリーポイントをランダムに選択します

    setup_camera(&mut commands, Vec2::new(player_x, player_y));

    // プレイヤーキャラクターの魔法使いを生成
    // プレイヤーキャラクターのみ Player コンポーネントの追加が必要なため、
    // イベントではなく直接生成します
    let entity = spawn_actor(
        &mut commands,
        &asset_server,
        &registry,
        Vec2::new(player_x, player_y),
        Actor {
            actor_type: ActorType::new("Witch"),
            life: player_state.max_life, // 新しいレベルに入るたびに全回復している
            max_life: player_state.max_life,
            amplitude: 0.0,
            fire_damage_wait: 0,
            actor_group: ActorGroup::Friend,
            wands: player_state.wands,
            inventory: player_state.inventory,
            current_wand: player_state.current_wand,
            golds: player_state.golds,
            getting_up: if getting_up_animation { 240 } else { 0 },
            ..default()
        },
    );
    commands.entity(entity).insert((
        Witch,
        PlayerControlled,
        Player::new(config.player_name.clone(), &player_state.discovered_spells),
    ));

    current.chunk = Some(chunk);

    overlay.send(OverlayEvent::SetOpen(true));
}

fn spawn_level(
    mut commands: &mut Commands,
    registry: &Registry,
    ldtk: &LDTK,
    level: &GameLevel,
) -> LevelChunk {
    // 画像データから現在位置のレベルの情報を選択して読み取ります
    let chunk = LevelChunk::new(&registry, &ldtk, &level);

    // レベルの外観を生成します
    spawn_world_tilemap(&mut commands, &registry, &chunk);

    // レベルのコリジョンを生成します
    spawn_wall_collisions(&mut commands, &chunk);

    // ナビゲーションメッシュを作成します
    // ナビメッシュ生成は重いためチャンクごとに生成します
    // このため、敵キャラクターがレベル境界を越えて接近することはありません
    spawn_navigation_mesh(&mut commands, &chunk);

    chunk
}

fn update_tile_sprites(
    mut current: ResMut<LevelSetup>,
    registry: Registry,
    mut commands: Commands,
    tiles_query: Query<(Entity, &TileSprite)>,
    collider_query: Query<Entity, With<WallCollider>>,
) {
    if let Some(ref mut chunk) = current.chunk {
        // 範囲内を更新
        if let Some((left, top, right, bottom)) = chunk.dirty {
            let props = registry.get_level(&chunk.level);

            // 縦２タイルのみ孤立して残っているものがあれば削除
            chunk.remove_isolated_tiles(&registry, &props.default_tile);

            let min_x = (left - 1).max(chunk.min_x);
            let max_x = (right + 1).min(chunk.max_x);
            let min_y = (top - 1).max(chunk.min_y);
            let max_y = (bottom + 3).min(chunk.max_y); // 縦方向のみ広めに更新することに注意

            // dirty の範囲のスプライトをいったんすべて削除
            for (entity, TileSprite((tx, ty))) in tiles_query.iter() {
                if min_x <= *tx && *tx <= max_x && min_y <= *ty && *ty <= max_y {
                    commands.entity(entity).despawn_recursive();
                }
            }

            // スプライトを再生成
            for y in min_y..(max_y + 1) {
                for x in min_x..(max_x + 1) {
                    spawn_world_tile(&mut commands, &registry, chunk, x, y);
                }
            }

            // コリジョンはすべて再生成
            for entity in collider_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_wall_collisions(&mut commands, chunk);

            // ダーティーフラグをクリア
            chunk.dirty = None;
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>();
        app.add_systems(FixedUpdate, spawn_entity.in_set(FixedUpdateGameActiveSet));
        app.add_systems(OnEnter(GameState::InGame), setup_level);
        app.add_systems(FixedUpdate, update_tile_sprites.in_set(FixedUpdateAfterAll));
        app.init_resource::<LevelSetup>();
    }
}
