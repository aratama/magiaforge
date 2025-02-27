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
use crate::hud::overlay::OverlayEvent;
use crate::inventory::Inventory;
use crate::level::appearance::spawn_world_tile;
use crate::level::appearance::TileSprite;
use crate::level::chunk::index_to_position;
use crate::level::chunk::Bounds;
use crate::level::chunk::LevelChunk;
use crate::level::collision::spawn_wall_collisions;
use crate::level::collision::WallCollider;
use crate::level::entities::spawn_entity;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::level::spawn::spawn_random_enemies;
use crate::level::tile::Tile;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::level::world::LevelScoped;
use crate::player_state::PlayerState;
use crate::registry::path_to_string;
use crate::registry::Registry;
use crate::set::FixedUpdateAfterAll;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::asset::*;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::HashMap;

/// レベルとプレイヤーキャラクターを生成します
pub fn setup_game_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Registry,
    mut world: ResMut<GameWorld>,
    config: Res<GameConfig>,
    mut spawn: EventWriter<SpawnEvent>,
    mut overlay: EventWriter<OverlayEvent>,
) {
    // 各種変数の初期化 /////////////////////////////////////////////////////////////////////

    let game_registry = registry.game();
    let mut rng = StdRng::from_entropy();

    // プレイヤーの状態の復元 //////////////////////////////////////////////////////////////
    let mut player_state = world
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
        GameLevel::new("Inlet")
    } else {
        world.next_level.clone()
    };

    // 拠点に戻ってきたときは全回復します
    if level == GameLevel::new(HOME_LEVEL) {
        player_state.life = player_state.max_life;
    }

    // レベルの生成 ///////////////////////////////////////////////////////////////////////////

    // 画像データから現在位置のレベルの情報を選択して読み取ります
    // 各レベルのスプライト生成には隣接するレベルのタイル情報も必要なため、
    // 隣接するレベルも含めて先に読み取ります
    // 地形のスプライトやコリジョンなどはupdate_tile_spritesで改めて生成されます
    let center_chunk = LevelChunk::new(&registry, &level, false);
    world.chunks.push(center_chunk.clone());

    // 各レベルのエンティティを生成します
    spawn_level_entities_and_navmesh(&registry, &mut spawn, &mut rng, &center_chunk);

    // プレイヤーキャラクターの生成 ///////////////////////////////////////////////////////////////////////////////////////////
    // エントリーポイントをランダムに選択
    // プレイヤーはここに配置し、この周囲はセーフゾーンとなって敵モブやアイテムは生成しません
    // プレイヤーキャラクターのみ Player コンポーネントの追加が必要なため、
    // イベントではなく直接生成します
    // 拠点のみ最初にアニメーションが入るので PlayerInActive に設定します
    let getting_up_animation = level == GameLevel::new("Inlet") && cfg!(not(feature = "ingame"));
    let entry_points = center_chunk.entry_points();
    let player_position = entry_points.choose(&mut rng).expect("No entrypoint found");
    let entity = spawn_actor(
        &mut commands,
        &asset_server,
        &registry,
        *player_position,
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

    // リソースの更新やカメラ設定などの後処理 ////////////////////////////////////////////////////////////////////

    world.next_level = level.clone();

    setup_camera(&mut commands, *player_position);

    overlay.send(OverlayEvent::SetOpen(true));
}

fn spawn_level_entities_and_navmesh(
    registry: &Registry,
    mut spawn: &mut EventWriter<SpawnEvent>,
    mut rng: &mut StdRng,
    chunk: &LevelChunk,
) {
    let level = chunk.level.clone();

    let props = registry.get_level(&level);

    // エンティティ生成 /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // 宝箱や灯篭などのエンティティを生成します
    for props in chunk.entities.iter() {
        spawn.send(SpawnEvent {
            position: props.position,
            spawn: props.spawn.clone(),
        });
    }

    // エンティティ生成 /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // 地雷原テスト実装
    if level == GameLevel::new("minefield") {
        for (x, y) in chunk.bounds.iter() {
            let tile = chunk.get_tile(x, y);
            if *tile == Tile::new("Soil") || *tile == Tile::new("Grassland") {
                if rand::random::<u32>() % 20 == 0 {
                    spawn.send(SpawnEvent {
                        position: index_to_position((x, y)),
                        spawn: Spawn::Actor {
                            actor_type: "ExplosiveMashroom".to_string(),
                        },
                    });
                }
            }
        }
    }

    // 空間
    // ここに敵モブや落ちているアイテムを生成します
    let empties = chunk.get_spawn_tiles(&registry);

    // 空いた空間に敵モブキャラクターをランダムに生成します
    let spaw_enemy_count = props.enemies;
    let spaw_enemy_types = props.get_enemy_types();
    spawn_random_enemies(
        &level,
        &empties,
        &mut rng,
        chunk.entry_points().clone(),
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
                    spawn: Spawn::Actor {
                        actor_type: "Chicken".to_string(),
                    },
                });
            }
        }
    }
}

fn spawn_neighbor_chunks(
    registry: Registry,
    mut world: ResMut<GameWorld>,
    mut spawn: EventWriter<SpawnEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut rng = StdRng::from_entropy();
    let ldtk = registry.ldtk();
    // 現在のチャンクを取得
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let position = player_transform.translation.truncate();
    let Some(chunk) = world.find_chunk_by_position(position) else {
        return;
    };
    for neighbor in ldtk.get_neighbors(&chunk.level).iter() {
        if world.get_chunk(neighbor).is_none() {
            let chunk = LevelChunk::new(&registry, neighbor, true);
            world.chunks.push(chunk.clone());
            spawn_level_entities_and_navmesh(&registry, &mut spawn, &mut rng, &chunk);

            // チャンクの生成は重い処理になるため、1フレームで生成するチャンクは最大ひとつ
            return;
        }
    }
}

/// 現在プレイヤーがいるチャンクと隣接していないチャンクを削除します
/// チャンクの削除は時間がかかることがあるため、削除するのは1フレームに1個までとします
/// todo 血痕や爆発痕の削除
fn despawn_chunks(
    mut commands: Commands,
    registry: Registry,
    mut world: ResMut<GameWorld>,
    player_query: Query<&Transform, With<Player>>,
    actors_query: Query<(Entity, &Transform), (With<Actor>, Without<Player>)>,
    scoped_query: Query<(Entity, &LevelScoped)>,
) {
    let ldtk = registry.ldtk();
    // 現在のチャンクを取得
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let position = player_transform.translation.truncate();
    let Some(current_level) = world.get_level_by_position(position) else {
        return;
    };

    let cs = ldtk.get_neighbors(&current_level);
    let neighbors: Vec<&GameLevel> = cs.iter().collect();

    let Some(chunk_to_remove): Option<GameLevel> = world
        .chunks
        .iter()
        .find(|chunk| current_level != chunk.level && !neighbors.contains(&&chunk.level))
        .map(|chunk| chunk.level.clone())
    else {
        return;
    };

    // LevelScopedを削除
    for (entity, level_scoped) in scoped_query.iter() {
        if level_scoped.0 == chunk_to_remove {
            commands.entity(entity).despawn_recursive();
        }
    }

    // アクターを削除
    // 大半のエンティティは LevelScoped で削除しますが、
    // アクターのみはレベルをまたがって移動することが考えられるため、
    // 現在の位置に応じて削除します
    let chunk = world.get_chunk(&chunk_to_remove).unwrap();
    for (entity, transform) in actors_query.iter() {
        if chunk.contains(transform.translation.truncate()) {
            commands.entity(entity).despawn_recursive();
        }
    }

    world.chunks.retain(|c| chunk_to_remove != c.level);
}

fn remove_isolated_tiles(mut world: ResMut<GameWorld>, registry: Registry) {
    let mut isolates: Vec<(i32, i32)> = Vec::new();
    // 縦２タイルのみ孤立して残っているものがあれば削除
    for chunk in &world.chunks {
        if chunk.dirty_scriptes.is_none() {
            continue;
        }
        for y in chunk.bounds.min_y..(chunk.bounds.max_y + 1) {
            for x in chunk.bounds.min_x..(chunk.bounds.max_x + 1) {
                if !world.is_wall(&registry, x, y + 0)
                    && world.is_wall(&registry, x, y + 1)
                    && !world.is_wall(&registry, x, y + 2)
                {
                    warn!("Remove isolated wall at {} {}", x, y);
                    isolates.push((x, y + 1));
                } else if !world.is_wall(&registry, x, y + 0)
                    && world.is_wall(&registry, x, y + 1)
                    && world.is_wall(&registry, x, y + 2)
                    && !world.is_wall(&registry, x, y + 3)
                {
                    warn!("Remove isolated wall at {} {}", x, y);
                    isolates.push((x, y + 1));
                    isolates.push((x, y + 2));
                }
            }
        }
    }
    for (x, y) in isolates.iter() {
        world.set_tile(*x, *y, Tile::new("Soil"));
    }
}

fn update_tile_collisions(
    mut world: ResMut<GameWorld>,
    registry: Registry,
    mut commands: Commands,
    collider_query: Query<(Entity, &LevelScoped), With<WallCollider>>,
) {
    // コリジョンの再生成
    // mutableでchunkを取得
    for chunk in &mut world.chunks {
        // コリジョンの再生成
        // dirty フラグが設定されている場合は常に再生成します
        if chunk.dirty_scriptes.is_some() || chunk.loading_index == 0 {
            info!("spawning chunk collisions: {:?}", chunk.level);

            // 既存のコリジョンを削除
            for (entity, scope) in collider_query.iter() {
                if scope.0 == chunk.level {
                    commands.entity(entity).despawn_recursive();
                }
            }

            // コリジョンを再生成
            spawn_wall_collisions(&mut commands, &registry, chunk);
        }
    }
}

fn update_dirty_tile_sprites(
    mut world: ResMut<GameWorld>,
    registry: Registry,
    mut commands: Commands,
    tiles_query: Query<(Entity, &TileSprite)>,
) {
    for chunk in &world.chunks {
        if let Some(dirty) = &chunk.dirty_scriptes.clone() {
            // 爆弾での破壊時など、通常の更新
            info!("updating dirty tiles: {:?}", dirty);

            clear_tiles_by_bounds(&mut commands, &tiles_query, *dirty);

            for (x, y) in dirty.iter() {
                spawn_world_tile(&mut commands, &registry, &world, &chunk, x, y);
            }
        }
    }

    for chunk in &mut world.chunks {
        chunk.dirty_scriptes = None;
    }
}

/// チャンクの読み込み・更新には次の二種類があります
/// ・loading_index による順次読み込みの場合。これはレベル全体にわたる広範囲であり、処理に時間がかかるので、各フレームで分割して行う
/// ・爆弾の爆発の範囲などを dirty フラグで更新する場合。これは通常狭い範囲であり、画面内に収まっていることも多いので、瞬時にすべて更新する
/// loading_index による順次読み込みの最中に dirty フラグが設定された場合、二重生成を避けるため、チャンクすべての再生成を瞬時に行います。
/// これが発生するケースは希です
fn update_lazy_tile_sprites(
    mut world: ResMut<GameWorld>,
    registry: Registry,
    mut commands: Commands,
    tiles_query: Query<(Entity, &TileSprite)>,
) {
    // チャンクごとに更新すべきインデックス列を保持
    let mut tile_spawning_indices: HashMap<String, Vec<i32>> = HashMap::new();

    // ハッシュマップにチャンクごとVecを生成
    for chunk in &world.chunks {
        tile_spawning_indices.insert(chunk.level.0.clone(), Vec::new());
    }

    // コリジョンの再生成
    // mutableでchunkを取得
    for chunk in &mut world.chunks {
        let indiceis_to_spawn = tile_spawning_indices.get_mut(&chunk.level.0).unwrap();

        // loading_index の続きを一部更新
        if chunk.height() <= chunk.loading_index {
            continue;
        }

        indiceis_to_spawn.push(chunk.bounds.min_y + chunk.loading_index);

        clear_tiles_by_bounds(
            &mut commands,
            &tiles_query,
            Bounds {
                min_x: chunk.bounds.min_x,
                max_x: chunk.bounds.max_x,
                min_y: chunk.bounds.min_y + chunk.loading_index,
                max_y: chunk.bounds.min_y + chunk.loading_index + 1,
            },
        );

        chunk.loading_index += 1;
    }

    // 予約されたインデックスに応じてスプライトを生成
    // こちらは immutableでchunkを取得
    for (chunk_identifier, indices) in tile_spawning_indices.iter() {
        let chunk = world
            .chunks
            .iter()
            .find(|c| c.level.0 == *chunk_identifier)
            .unwrap();

        // スプライトを再生成
        for y in indices.iter() {
            for x in chunk.bounds.min_x..chunk.bounds.max_x {
                spawn_world_tile(&mut commands, &registry, &world, &chunk, x, *y);
            }
        }
    }
}

fn clear_tiles_by_bounds(
    commands: &mut Commands,
    tiles_query: &Query<(Entity, &TileSprite)>,
    bounds: Bounds,
) {
    for (entity, TileSprite((tx, ty))) in tiles_query.iter() {
        if bounds.contains(*tx, *ty) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn select_bgm(
    registry: Registry,
    asset_server: Res<AssetServer>,
    mut next_bgm: ResMut<NextBGM>,
    setup: Res<GameWorld>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let position = player_transform.translation.truncate();

    let Some(chunk) = setup.find_chunk_by_position(position) else {
        return;
    };

    let level_props = registry.get_level(&chunk.level);

    if let Some(source) = &next_bgm.0 {
        let Some(path) = source.path() else {
            return;
        };
        if path_to_string(path) == level_props.bgm {
            return;
        }
        *next_bgm = NextBGM(Some(asset_server.load(level_props.bgm.clone())));
        info!("bgm changed to {:?}", level_props.bgm);
    } else {
        *next_bgm = NextBGM(Some(asset_server.load(level_props.bgm.clone())));
        info!("bgm changed to {:?}", level_props.bgm);
    };
}

fn clear_world(mut world: ResMut<GameWorld>) {
    world.chunks.clear();
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>();
        app.add_systems(
            FixedUpdate,
            (
                select_bgm,
                (spawn_neighbor_chunks, despawn_chunks, spawn_entity).chain(),
            )
                .in_set(FixedUpdateGameActiveSet),
        );
        app.add_systems(OnEnter(GameState::InGame), setup_game_world);
        app.add_systems(OnExit(GameState::InGame), clear_world);
        app.add_systems(
            FixedUpdate,
            (
                remove_isolated_tiles,
                update_tile_collisions,
                update_dirty_tile_sprites,
                update_lazy_tile_sprites,
            )
                .chain()
                .in_set(FixedUpdateAfterAll),
        );
        app.init_resource::<GameWorld>();
    }
}
