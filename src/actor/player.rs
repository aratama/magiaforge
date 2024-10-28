use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::actor::Actor;
use crate::entity::bullet::{spawn_bullet, BULLET_RADIUS, BULLET_SPAWNING_MARGIN};
use crate::entity::witch::WITCH_COLLIDER_RADIUS;
use crate::gamepad::{get_direction, get_fire_trigger};
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

// 魔法の拡散
const BULLET_SCATTERING: f32 = 0.3;

// 魔法弾の速度
// pixels_per_meter が 100.0 に設定されているので、
// 200は1フレームに2ピクセル移動する速度です
const BULLET_SPEED: f32 = 50.0;

// 次の魔法を発射するまでの待機フレーム数
const BULLET_COOLTIME: i32 = 8;

// 一度に発射する弾丸の数
const BULLETS_PER_FIRE: u32 = 1;

/// 操作可能なプレイヤーキャラクターを表します
#[derive(Component)]
pub struct Player {
    pub last_idle_frame_count: FrameCount,
    pub last_ilde_x: f32,
    pub last_ilde_y: f32,
    pub last_idle_vx: f32,
    pub last_idle_vy: f32,
    pub last_idle_life: i32,
    pub last_idle_max_life: i32,
}

fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut Actor,
            &mut Transform,
            &mut ExternalForce,
            &GlobalTransform,
            &mut Sprite,
        ),
        (With<Player>, Without<Camera2d>),
    >,
    mut commands: Commands,
    assets: Res<GameAssets>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let force = 50000.0;

    let direction = get_direction(keys);

    // 魔法の発射
    if get_fire_trigger(buttons) {
        let direction = Vec2::from_angle(0.0);
        let bullet_position = Vec2::ZERO;
        spawn_bullet(
            &mut commands,
            assets.asset.clone(),
            bullet_position,
            direction * BULLET_SPEED,
            &assets,
        );
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_player.run_if(in_state(GameState::InGame)),
        );
    }
}
