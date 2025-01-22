use std::collections::HashMap;

use bevy::color::Color;

use crate::{actor::ActorType, interpreter::Cmd, language::Dict, spell::SpellProps};

pub const CRATE_NAME: &str = "magiaforge";

pub const WEBSOCKET_URL: &str = "wss://magia-server-38847751193.asia-northeast1.run.app";

pub const INITIAL_LEVEL: i32 = 0;

pub const DEFAULT_BGM_VOLUME: f32 = 0.3;

pub const DEFAULT_SE_VOLUME: f32 = 0.8;

pub const MAX_WANDS: usize = 4;

pub const MAX_SPELLS_IN_WAND: usize = 8;

pub const MAX_ITEMS_IN_INVENTORY_ROW: usize = 8;

pub const MAX_ITEMS_IN_INVENTORY_COLUMN: usize = 8;

pub const MAX_ITEMS_IN_INVENTORY: usize =
    MAX_ITEMS_IN_INVENTORY_ROW * MAX_ITEMS_IN_INVENTORY_COLUMN;

/// 拠点を含むシングルプレイ用ステージの数
/// level.aseprite のスライスの最大値 - 1
pub const LEVELS: i32 = 8;

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
pub const SCORCH_MARK_LAYER_Z: f32 = 10.2;

pub const BLOOD_LAYER_Z: f32 = 10.1;

/// 魔法陣などのレイヤー
/// 床タイルよりは常に上だが、キャラクターなどのエンティティよりは下
pub const PAINT_LAYER_Z: f32 = 10.0;

/// 床タイルのレイヤー
/// すべてのスプライトの最下部
pub const FLOOR_LAYER_Z: f32 = 0.0;

pub const WATER_FOAM_LAYER_Z: f32 = -1.0;

pub const WATER_MESH_LIGHTER_LAYER_Z: f32 = -1.4;

pub const WATER_MESH_DARKER_LAYER_Z: f32 = -1.5;

pub const SHORE_LAYER_Z: f32 = -2.0;

/// y座標に対してのz座標の増加量
pub const Z_ORDER_SCALE: f32 = -0.001;

pub const CAMERA_SPEED: f32 = 0.1;

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

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct GameConstants {
    pub spells: HashMap<String, SpellProps>,
    pub levels: Vec<LevelProps>,
}

#[derive(serde::Deserialize)]
pub struct LevelProps {
    pub name: Dict<String>,
    pub enemies: u8,
    pub enemy_types: Vec<ActorType>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct GameActors {
    pub dumping_on_ice: f32,
    pub dumping_on_air: f32,
    pub acceleration_on_ice: f32,
    pub acceleration_on_air: f32,
    pub acceleration_on_drowning: f32,
    pub acceleration_on_firing: f32,
    pub actors: HashMap<String, ActorProps>,
}

#[derive(serde::Deserialize)]
pub struct ActorProps {
    pub move_force: f32,
    pub jump: f32,
    pub linear_damping: f32,
}

#[derive(Debug)]
pub enum SenarioType {
    HelloRabbit,
    SingleplayRabbit,
    MultiplayerRabbit,
    ReserchRabbit,
    TrainingRabbit,
    SpellListRabbit,
    HugeSlime,
}

impl SenarioType {
    pub fn to_acts<'a>(&self, senarios: &'a GameSenarios) -> &'a Vec<Cmd> {
        senarios.senarios.get(&format!("{:?}", self)).unwrap()
    }
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct GameSenarios {
    pub senarios: HashMap<String, Vec<Cmd>>,
}
