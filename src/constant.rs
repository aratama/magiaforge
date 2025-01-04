use crate::states::GameState;
use bevy::color::Color;
use bevy_rapier2d::prelude::*;
use std::cell::LazyCell;

// Setupステートでの初期化が完了した直後に遷移する先のステート
// 本来は MainMenu にするが、開発時はここで起動時の画面を切り替えています
pub const INITIAL_STATE: GameState = GameState::MainMenu;
pub const INITIAL_LEVEL: i32 = 0;
// pub const INITIAL_STATE: GameState = GameState::InGame;
// pub const INITIAL_LEVEL: i32 = 3;

pub const CRATE_NAME: &str = "magiaforge";

pub const WEBSOCKET_URL: &str = "wss://magia-server-38847751193.asia-northeast1.run.app";

pub const DEFAULT_BGM_VOLUME: f32 = 0.3;

pub const DEFAULT_SE_VOLUME: f32 = 0.8;

pub const MAX_WANDS: usize = 4;

pub const MAX_SPELLS_IN_WAND: usize = 8;

pub const MAX_ITEMS_IN_INVENTORY_ROW: usize = 8;

pub const MAX_ITEMS_IN_INVENTORY_COLUMN: usize = 8;

pub const MAX_ITEMS_IN_INVENTORY: usize =
    MAX_ITEMS_IN_INVENTORY_ROW * MAX_ITEMS_IN_INVENTORY_COLUMN;

pub const MAX_ITEMS_IN_EQUIPMENT: usize = 8;

/// 拠点を含むシングルプレイ用ステージの数
/// level.aseprite のスライスの最大値 - 1
pub const LEVELS: i32 = 7;

/// このレベルにボスがいなくなったらエンディングへ移行
pub const LAST_BOSS_LEVEL: i32 = LEVELS - 1;

/// 1タイルのサイズのピクセル数
/// タイルサイズは意味合いとしてゃ u32 ですが、f32 で扱うことが多いので f32 にしています
pub const TILE_SIZE: f32 = 16.0;

pub const TILE_HALF: f32 = TILE_SIZE / 2.0;

/// 壁の高さのピクセル数
/// 天井のタイルはこの大きさだけ上方向にずれます
/// 本当はもう少し大きいほうが見栄えはいいのですが、
/// そうすると1タイルの通路の床が隠れてしまい見づらくなるので小さめにしてます
pub const WALL_HEIGHT: f32 = 32.0;

// プレイヤー /////////////////////////////////////////////////////////////////////

// レイヤー ///////////////////////////////////////////////////////////////////////

/// シードは空中にあるので一番上
pub const SERVANT_SEED_LAYER_Z: f32 = 23.0;

pub const DAMAGE_NUMBER_LAYER_Z: f32 = 22.0;

pub const PARTICLE_LAYER_Z: f32 = 21.6;

pub const CEIL_LAYER_Z: f32 = 21.5;

/// キャラクターやチェストなどのレイヤー
pub const ENTITY_LAYER_Z: f32 = 20.0;

pub const SHADOW_LAYER_Z: f32 = 11.0;

/// ツボの破片のレイヤー
pub const PIECE_LAYER_Z: f32 = 10.5;

/// 爆発の煤のレイヤー
pub const SCORCH_MARK_LAYER_Z: f32 = 10.1;

/// 魔法陣などのレイヤー
/// 床タイルよりは常に上だが、キャラクターなどのエンティティよりは下
pub const PAINT_LAYER_Z: f32 = 10.0;

/// 床タイルのレイヤー
/// すべてのスプライトの最下部
pub const FLOOR_LAYER_Z: f32 = 0.0;

//

pub const Z_ORDER_SCALE: f32 = 0.001;

pub const CAMERA_SPEED: f32 = 0.1;

// 衝突グループ ////////////////////////////////////////////////////////////////////////////////////////////////////

/// 木箱などの通常のエンティティのメンバーシップです
/// ハイド状態のシャドウなどを除きほとんどのアクターと衝突し、弾丸にも衝突します
const ENTITY_MEMBERSHIPS: Group = Group::GROUP_1;

/// ツボの破片などのグループ
/// 壁やチェストにはめり込まないで散らばるが、プレイヤーキャラクターなどには干渉しない
const PIECE_MEMBERSHIPS: Group = Group::GROUP_2;

const WALL_MEMBERSHIPS: Group = Group::GROUP_3;

/// ニワトリなどの中立キャラクターのグループ
const NEUTRAL_MEMBERSHIPS: Group = Group::GROUP_4;

/// プレイヤーキャラクターのグループ
const PLAYER_MEMBERSHIPS: Group = Group::GROUP_5;

const PLAYER_BULLET_MEMBERSHIPS: Group = Group::GROUP_6;

const ENEMY_MEMBERSHIPS: Group = Group::GROUP_7;

const ENEMY_BULLET_MEMBESHIPS: Group = Group::GROUP_8;

const GOLD_MEMBERSHIPS: Group = Group::GROUP_9;

/// 汎用のセンサーです
/// 魔法陣の出入りなどの判定に使われるほか、intersections_with_shape を通じて衝撃の範囲、延焼の範囲判定などに使われるため、
/// すべてのアクターとエンティティと衝突します
const SENSOR_MEMBERSHIPS: Group = Group::GROUP_10;

/// 商品がショップから押し出されないようにするための見えない壁です
const HIDDEN_WALL_MEMBERSHIPS: Group = Group::GROUP_11;

const RABBIT_MEMBERSHIPS: Group = Group::GROUP_12;

/// SHADOW_GROUP はシャドウが隠れているときのみのメンバーシップで、
/// 壁やアクターには衝突しますが、弾丸は当たりません
const SHADOW_MEMBERSHIPS: Group = Group::GROUP_13;

/// ドロップアイテムのグループ
/// ENTITY_GROUPと似ていますが、敵キャラクターと敵キャラクターの弾丸には衝突しません
/// 敵キャラクターがアイテムを押して盾にするのを避けるためです
const DROPPED_ITEM_MEMBERSHIPS: Group = Group::GROUP_14;

pub const ENTITY_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        ENTITY_MEMBERSHIPS,
        PIECE_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | PLAYER_BULLET_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | ENEMY_BULLET_MEMBESHIPS
            | WALL_MEMBERSHIPS
            | RABBIT_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | GOLD_MEMBERSHIPS
            | SENSOR_MEMBERSHIPS,
    )
});

pub const PIECE_GROUPS: LazyCell<CollisionGroups> =
    LazyCell::new(|| CollisionGroups::new(PIECE_MEMBERSHIPS, PIECE_MEMBERSHIPS | WALL_MEMBERSHIPS));

pub const WALL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        WALL_MEMBERSHIPS,
        PIECE_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | PLAYER_BULLET_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | ENEMY_BULLET_MEMBESHIPS
            | RABBIT_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | GOLD_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS,
    )
});

const ACTOR_FILTER_BASE: LazyCell<Group> = LazyCell::new(|| {
    ENTITY_MEMBERSHIPS
        | NEUTRAL_MEMBERSHIPS
        | WALL_MEMBERSHIPS
        | PLAYER_MEMBERSHIPS
        | ENEMY_MEMBERSHIPS
        | RABBIT_MEMBERSHIPS
        | SENSOR_MEMBERSHIPS
        | SHADOW_MEMBERSHIPS
});

pub const NEUTRAL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        NEUTRAL_MEMBERSHIPS,
        PLAYER_BULLET_MEMBERSHIPS | ENEMY_BULLET_MEMBESHIPS | *ACTOR_FILTER_BASE,
    )
});

pub const PLAYER_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        PLAYER_MEMBERSHIPS,
        ENEMY_BULLET_MEMBESHIPS | DROPPED_ITEM_MEMBERSHIPS | *ACTOR_FILTER_BASE,
    )
});

pub const ENEMY_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        ENEMY_MEMBERSHIPS,
        PLAYER_BULLET_MEMBERSHIPS | *ACTOR_FILTER_BASE,
    )
});

const BULLET_FILTER_BASE: LazyCell<Group> =
    LazyCell::new(|| ENTITY_MEMBERSHIPS | WALL_MEMBERSHIPS | NEUTRAL_MEMBERSHIPS);

pub const PLAYER_BULLET_GROUP: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        PLAYER_BULLET_MEMBERSHIPS,
        // アイテムを押して動かせるように、プレイヤーの弾丸は DROPPED_ITEM_MEMBERSHIPS に衝突します
        ENEMY_MEMBERSHIPS | NEUTRAL_MEMBERSHIPS | DROPPED_ITEM_MEMBERSHIPS | *BULLET_FILTER_BASE,
    )
});

pub const ENEMY_BULLET_GROUP: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        ENEMY_BULLET_MEMBESHIPS,
        // 敵がアイテムを盾に接近するのを避けるために、敵の弾丸は DROPPED_ITEM_MEMBERSHIPS に衝突しません
        PLAYER_MEMBERSHIPS | NEUTRAL_MEMBERSHIPS | *BULLET_FILTER_BASE,
    )
});

pub const GOLD_GROUPS: LazyCell<CollisionGroups> =
    LazyCell::new(|| CollisionGroups::new(GOLD_MEMBERSHIPS, ENTITY_MEMBERSHIPS | WALL_MEMBERSHIPS));

pub const SENSOR_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        SENSOR_MEMBERSHIPS,
        PLAYER_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS
            | SENSOR_MEMBERSHIPS, // 爆弾で蜘蛛の巣を破壊するため、センサーはセンサーと衝突します
    )
});

pub const HIDDEN_WALL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        HIDDEN_WALL_MEMBERSHIPS,
        ENTITY_MEMBERSHIPS | RABBIT_MEMBERSHIPS | DROPPED_ITEM_MEMBERSHIPS,
    )
});

pub const RABBIT_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        RABBIT_MEMBERSHIPS,
        ENTITY_MEMBERSHIPS
            | WALL_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS
            | HIDDEN_WALL_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS,
    )
});

pub const SHADOW_GROUPS: LazyCell<CollisionGroups> =
    LazyCell::new(|| CollisionGroups::new(SHADOW_MEMBERSHIPS, *ACTOR_FILTER_BASE));

pub const DROPPED_ITEM_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        DROPPED_ITEM_MEMBERSHIPS,
        DROPPED_ITEM_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | PLAYER_BULLET_MEMBERSHIPS
            | WALL_MEMBERSHIPS
            | HIDDEN_WALL_MEMBERSHIPS
            | RABBIT_MEMBERSHIPS,
    )
});

/// rapier の pixels_per_meter に設定する値
/// イメージしやすくするため、1タイル = 16ピクセル = 1メートルとしています
pub const PIXELS_PER_METER: f32 = 16.0;

// UI階層

pub const LOADING_Z_INDEX: i32 = 100000;

pub const OVERLAY_Z_INDEX: i32 = 10000;

pub const POINTER_Z_INDEX: i32 = 5000;

pub const GAME_MENU_Z_INDEX: i32 = 2000;

pub const WAND_EDITOR_FLOATING_Z_INDEX: i32 = 1600;

pub const WAND_EDITOR_Z_INDEX: i32 = 1500;

pub const HUD_Z_INDEX: i32 = 1000;

// UIカラーテーマ

pub const UI_PRIMARY: Color = Color::hsla(60.0, 0.05, 0.59, 1.0);

pub const UI_PRIMARY_DARKER: Color = Color::hsla(56.0, 0.06, 0.56, 1.0);

pub const UI_SECONDARY: Color = Color::hsla(57.0, 0.11, 0.37, 1.0);

// レベル
