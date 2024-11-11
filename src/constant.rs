use crate::states::GameState;
use bevy_rapier2d::prelude::*;

#[allow(dead_code)]
pub const CRATE_NAME: &str = "magiaboost";

// Setupステートでの初期化が完了した直後に遷移する先のステート
// 本来は MainMenu にするが、開発時はここで起動時の画面を切り替えています
pub const INITIAL_STATE: GameState = GameState::MainMenu;

pub const MAX_WANDS: usize = 4;

/// level.aseprite のスライスの最大値 - 1
pub const LEVELS: i32 = 4;

/// 1タイルのサイズのピクセル数
/// タイルサイズは意味合いとしてゃ u32 ですが、f32 で扱うことが多いので f32 にしています
pub const TILE_SIZE: f32 = 16.0;

pub const TILE_HALF: f32 = TILE_SIZE / 2.0;

/// 壁の高さのピクセル数
/// 天井のタイルはこの大きさだけ上方向にずれます
/// 本当はもう少し大きいほうが見栄えはいいのですが、
/// そうすると1タイルの通路の床が隠れてしまい見づらくなるので小さめにしてます
pub const WALL_HEIGHT: f32 = 8.0;

// レイヤー

/// キャラクターやチェストなどのレイヤー
pub const ENTITY_LAYER_Z: f32 = 20.0;

/// 魔法陣などのレイヤー
/// 床タイルよりは常に上だが、キャラクターなどのエンティティよりは下
pub const PAINT_LAYER_Z: f32 = 10.0;

/// 床タイルのレイヤー
/// すべてのスプライトの最下部
pub const FLOOR_LAYER_Z: f32 = 0.0;

//

pub const Z_ORDER_SCALE: f32 = 0.001;

pub const CAMERA_SPEED: f32 = 0.1;

// 衝突グループ
// 敵キャラクターが同士討ちしないように、敵キャラクターはグループを分けています
// WITCH_GROUP は PVP があるため、他のすべてのグループと衝突します
// それ以外の敵キャラクターグループは、自分のグループの攻撃には衝突しません

pub const ENTITY_GROUP: Group = Group::GROUP_1;

pub const WALL_GROUP: Group = Group::GROUP_3;

/// プレイヤーキャラクターのグループ
/// 自分の生成した弾丸に衝突します
pub const WITCH_GROUP: Group = Group::GROUP_5;

pub const WITCH_BULLET_GROUP: Group = Group::GROUP_6;

pub const ENEMY_GROUP: Group = Group::GROUP_6;

pub const ENEMY_BULLET_GROUP: Group = Group::GROUP_7;

pub const MAGIC_CIRCLE_GROUP: Group = Group::GROUP_8;

/// rapier の pixels_per_meter に設定する値
/// イメージしやすくするため、1タイル = 16ピクセル = 1メートルとしています
pub const PIXELS_PER_METER: f32 = 16.0;

// UI階層
pub const OVERLAY_Z_INDEX: i32 = 10000;

pub const HUD_Z_INDEX: i32 = 1000;

pub const GAME_MENU_Z_INDEX: i32 = 500;
