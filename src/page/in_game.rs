use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::camera::setup_camera;
use crate::config::GameConfig;
use crate::constant::*;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::hud::overlay::OverlayEvent;
use crate::inventory::Inventory;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::level::appearance::read_level_chunk_data;
use crate::level::appearance::spawn_world_tile;
use crate::level::appearance::spawn_world_tilemap;
use crate::level::appearance::TileSprite;
use crate::level::collision::spawn_wall_collisions;
use crate::level::collision::WallCollider;
use crate::level::entities::spawn_entity;
use crate::level::entities::SpawnEntity;
use crate::level::entities::SpawnEntityEvent;
use crate::level::entities::SpawnWitchType;
use crate::level::map::image_to_spawn_tiles;
use crate::level::map::index_to_position;
use crate::level::map::LevelChunk;
use crate::level::tile::Tile;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::set::FixedUpdateAfterAll;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use strum::IntoEnumIterator;
use vleue_navigator::prelude::NavMeshSettings;
use vleue_navigator::prelude::NavMeshUpdateMode;
use vleue_navigator::Triangulation;

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
            next_level: GameLevel::Level(INITIAL_LEVEL),
            next_state: None,
            shop_items: Vec::new(),
        }
    }
}

pub fn new_shop_item_queue(
    registry: &Registry,
    discovered_spells: Vec<SpellType>,
) -> Vec<InventoryItem> {
    let mut rng = rand::thread_rng();

    let mut shop_items: Vec<InventoryItem> = SpellType::iter()
        .filter(|s| discovered_spells.contains(&s) || registry.get_spell_props(*s).rank <= 1)
        .map(|s| InventoryItem {
            item_type: InventoryItemType::Spell(s),
            price: registry.get_spell_props(s).price,
        })
        .collect();

    shop_items.shuffle(&mut rng);

    shop_items
}

/// レベルとプレイヤーキャラクターを生成します
pub fn setup_level(
    mut commands: Commands,
    registry: Registry,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    mut current: ResMut<LevelSetup>,
    config: Res<GameConfig>,
    mut spawn: EventWriter<SpawnEntityEvent>,
    mut next: ResMut<NextState<GameMenuState>>,
    mut overlay: EventWriter<OverlayEvent>,
) {
    overlay.send(OverlayEvent::SetOpen(true));

    let mut rng = StdRng::from_entropy();

    let level = current.next_level;
    current.level = Some(current.next_level);

    // 拠点のみ最初にアニメーションが入るので PlayerInActive に設定します
    let getting_up_animation = level == GameLevel::Level(0) && cfg!(not(feature = "ingame"));
    if getting_up_animation {
        next.set(GameMenuState::PlayerInActive);
    }

    let biome_tile = registry.get_level_props(level).biome;

    // 画像データからレベルの情報を選択して読み取ります
    let chunk = read_level_chunk_data(
        &registry,
        &level_aseprites,
        &images,
        level,
        &mut rng,
        biome_tile,
    );

    // ナビゲーションメッシュを作成します
    // Spawn a new navmesh that will be automatically updated.
    commands.spawn((
        StateScoped(GameState::InGame),
        NavMeshSettings {
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                // ここ半タイルぶんズレてる？
                index_to_position((chunk.min_x, chunk.min_y)),
                index_to_position((chunk.max_x, chunk.min_y)),
                index_to_position((chunk.max_x, chunk.max_y)),
                index_to_position((chunk.min_x, chunk.max_y)),
            ]),

            // 小さすぎると、わずかな隙間を通り抜けようとしたり、
            // 曲がり角で壁に近すぎて減速してしまいます
            // 大きすぎると、対象が壁際にいるときに移動不可能な目的地になってしまうので、
            // パスが見つけられなくなってしまいます
            // Actorのデフォルトが5なので、それに合わせています
            agent_radius: 5.0,
            ..default()
        },
        // Mark it for update as soon as obstacles are changed.
        // Other modes can be debounced or manually triggered.
        NavMeshUpdateMode::Direct,
        Transform::from_translation(Vec3::ZERO),
    ));

    // レベルの外観を生成します
    spawn_world_tilemap(&mut commands, &registry, &chunk);

    // エントリーポイントを選択
    // プレイヤーはここに配置し、この周囲はセーフゾーンとなって敵モブやアイテムは生成しません
    let entry_points = chunk.entry_points();
    let entry_point = entry_points.choose(&mut rng).expect("No entrypoint found");

    let mut player_state = current
        .next_state
        .clone()
        .unwrap_or(if cfg!(feature = "item") {
            PlayerState {
                inventory: Inventory::from_vec(registry.spell().debug_items.clone()),
                wands: Wand::from_vec(registry.spell().debug_wands.clone()),
                ..default()
            }
        } else {
            PlayerState::default()
        });

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
    for y in chunk.min_y..chunk.max_y {
        for x in chunk.min_x..chunk.max_x {
            let tile = chunk.get_level_tile(x, y);
            if let Some(ref entity) = tile.entity {
                spawn.send(SpawnEntityEvent {
                    position: index_to_position((x, y)),
                    entity: entity.clone(),
                });
            }
        }
    }

    // 空間
    // ここに敵モブや落ちているアイテムを生成します
    let empties = image_to_spawn_tiles(&chunk);

    // 空いた空間に敵モブキャラクターをランダムに生成します
    let props = registry.get_level_props(level);
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

    spawn_dropped_items(
        &mut commands,
        &registry,
        &empties,
        &mut rng,
        entry_point.clone(),
        props.items,
        props.item_ranks.clone(),
    );

    // 拠点のみ、数羽のニワトリを生成します
    if level == GameLevel::Level(0) {
        for _ in 0..5 {
            if let Some((x, y)) = empties.choose(&mut rng) {
                spawn.send(SpawnEntityEvent {
                    position: index_to_position((*x, *y)),
                    entity: SpawnEntity::Actor {
                        actor_type: ActorType::Chicken,
                        actor_group: ActorGroup::Neutral,
                    },
                });
            }
        }
    }

    // テスト用モンスター
    // spawn.send(SpawnEntity::DefaultActor {
    //     actor_type: ActorType::Slime,
    //     actor_group: ActorGroup::Enemy,
    //     position: index_to_position((53, 68)),
    // });

    // プレイヤーを生成します
    // まずはエントリーポイントをランダムに選択します

    setup_camera(&mut commands, Vec2::new(player_x, player_y));

    // プレイヤーキャラクターの魔法使いを生成
    spawn.send(SpawnEntityEvent {
        position: Vec2::new(player_x, player_y),
        entity: SpawnEntity::Respawn {
            actor: Actor {
                life: player_state.life,
                max_life: player_state.max_life,
                amplitude: 0.0,
                fire_damage_wait: 0,
                actor_group: ActorGroup::Friend,
                wands: player_state.wands,
                inventory: player_state.inventory,
                current_wand: player_state.current_wand,
                golds: player_state.golds,
                extra: ActorExtra::Witch {
                    witch_type: SpawnWitchType::Player,
                    getting_up: getting_up_animation,
                    name: player_state.name,
                    discovered_spells: player_state.discovered_spells,
                },
                ..default()
            },
        },
    });

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
    spaw_enemy_count: u8,
    enemy_types: &Vec<ActorType>,
    spawn: &mut EventWriter<SpawnEntityEvent>,
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
                spawn.send(SpawnEntityEvent {
                    position,
                    entity: SpawnEntity::Actor {
                        actor_type: *enemy_type,
                        actor_group: ActorGroup::Enemy,
                    },
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
    registry: &Registry,
    empties: &Vec<(i32, i32)>,
    mut rng: &mut StdRng,
    safe_zone_center: (i32, i32),
    spaw_item_count: u8,
    ranks: Vec<u8>,
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
                let props = registry.get_spell_props(*i);
                ranks.contains(&props.rank)
            })
            .choose(&mut rng)
        {
            spawn_dropped_item(
                &mut commands,
                &registry,
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

fn update_tile_sprites(
    mut current: ResMut<LevelSetup>,
    registry: Registry,
    mut commands: Commands,
    tiles_query: Query<(Entity, &TileSprite)>,
    collider_query: Query<Entity, With<WallCollider>>,
) {
    if let Some(ref mut chunk) = current.chunk {
        // 縦２タイルのみ孤立して残っているものがあれば削除
        for y in chunk.min_y..(chunk.max_y + 1) {
            for x in chunk.min_x..(chunk.max_x + 1) {
                if !chunk.get_tile(x, y + 0).is_wall()
                    && chunk.get_tile(x, y + 1).is_wall()
                    && !chunk.get_tile(x, y + 2).is_wall()
                {
                    warn!("filling gap at {} {}", x, y);
                    chunk.set_tile(x, y + 1, Tile::StoneTile);
                } else if !chunk.get_tile(x, y + 0).is_wall()
                    && chunk.get_tile(x, y + 1).is_wall()
                    && chunk.get_tile(x, y + 2).is_wall()
                    && !chunk.get_tile(x, y + 3).is_wall()
                {
                    warn!("filling gap at {} {}", x, y);
                    chunk.set_tile(x, y + 1, Tile::StoneTile);
                    chunk.set_tile(x, y + 2, Tile::StoneTile);
                }
            }
        }

        // 範囲内を更新
        if let Some((left, top, right, bottom)) = chunk.dirty {
            info!("updating chunk {:?}", chunk.dirty);

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
        app.add_event::<SpawnEntityEvent>();
        app.add_systems(FixedUpdate, spawn_entity.in_set(FixedUpdateGameActiveSet));
        app.add_systems(OnEnter(GameState::InGame), setup_level);
        app.add_systems(OnEnter(GameState::InGame), select_level_bgm);
        app.add_systems(FixedUpdate, update_tile_sprites.in_set(FixedUpdateAfterAll));
        app.init_resource::<LevelSetup>();
    }
}
