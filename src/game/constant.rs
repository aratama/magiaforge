use super::states::GameState;
use bevy_rapier2d::prelude::*;

/// 1タイルのサイズのピクセル数
/// タイルサイズは意味合いとしてゃ u32 ですが、f32 で扱うことが多いので f32 にしています
pub const TILE_SIZE: f32 = 16.0;

pub const TILE_HALF: f32 = TILE_SIZE / 2.0;

/// 壁の高さのピクセル数
/// 天井のタイルはこの大きさだけ上方向にずれます
/// 本当はもう少し大きいほうが見栄えはいいのですが、
/// そうすると1タイルの通路の床が隠れてしまい見づらくなるので小さめにしてます
pub const WALL_HEIGHT: f32 = 8.0;

pub const FLOOR_LAYER_Z: f32 = 0.0;

pub const ENTITY_LAYER_Z: f32 = 3.0;

pub const ROOF_LAYER_Z: f32 = 6.0;

pub const Z_ORDER_SCALE: f32 = 0.001;

pub const CAMERA_SPEED: f32 = 0.1;

#[allow(dead_code)]
pub const CRATE_NAME: &str = "my_bevy_game";

// Setupステートでの初期化が完了した直後に遷移する先のステート
// 本来は MainMenu にするが、開発時はここで起動時の画面を切り替えています
pub const INITIAL_STATE: GameState = GameState::InGame;

pub const ENEMY_GROUP: Group = Group::GROUP_4;

pub const WALL_GROUP: Group = Group::GROUP_1;

pub const BULLET_GROUP: Group = Group::GROUP_2;

/// rapier の pixels_per_meter に設定する値
/// イメージしやすくするため、1タイル = 16ピクセル = 1メートルとしています
pub const PIXELS_PER_METER: f32 = 16.0;

// z_index
pub const HUD_Z_INDEX: i32 = 10000;

pub const OVERLAY_Z_INDEX: i32 = 1000;
