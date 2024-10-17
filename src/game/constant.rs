use super::states::GameState;
use bevy_rapier2d::prelude::*;

pub const TILE_SIZE: f32 = 16.0;

pub const Z_ORDER_SCALE: f32 = 0.001;

#[allow(dead_code)]
pub const CRATE_NAME: &str = "my_bevy_game";

// Setupステートでの初期化が完了した直後に遷移する先のステート
// 本来は MainMenu にするが、開発時はここで起動時の画面を切り替えています
pub const INITIAL_STATE: GameState = GameState::InGame;

pub const WALL_GROUP: Group = Group::GROUP_1;

pub const BULLET_GROUP: Group = Group::GROUP_2;
