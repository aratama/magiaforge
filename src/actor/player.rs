use super::remote::RemoteMessage;
use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::constant::*;
use crate::entity::actor::Actor;
use crate::entity::bullet::{spawn_bullet, BULLET_RADIUS, BULLET_SPAWNING_MARGIN};
use crate::entity::witch::WITCH_COLLIDER_RADIUS;
use crate::input::{get_direction, get_fire_trigger, MyGamepad};
use crate::states::{GameMenuState, GameState};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::ClientMessage;
use rand::random;
use std::f32::consts::PI;

// 魔法の拡散
const BULLET_SCATTERING: f32 = 0.3;

// 魔法弾の速度
// pixels_per_meter が 100.0 に設定されているので、
// 200は1フレームに2ピクセル移動する速度です
const BULLET_SPEED: f32 = 200.0;

// 次の魔法を発射するまでの待機フレーム数
const BULLET_COOLTIME: i32 = 8;

// 一度に発射する弾丸の数
const BULLETS_PER_FIRE: u32 = 1;

/// 操作可能なプレイヤーキャラクターを表します
#[derive(Component)]
pub struct Player {
    pub name: String,
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

    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,

    mut writer: EventWriter<ClientMessage>,

    menu: Res<State<GameMenuState>>,

    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    let force = 50000.0;

    let direction = get_direction(keys, axes, &my_gamepad);

    if let Ok((mut player, mut player_transform, mut player_force, _, mut player_sprite)) =
        player_query.get_single_mut()
    {
        player_transform.translation.z =
            ENTITY_LAYER_Z - player_transform.translation.y * Z_ORDER_SCALE;

        if *menu == GameMenuState::Close {
            player_force.force = direction * force;

            // プレイヤーの向き
            let angle = player.pointer.to_angle();

            if angle < -PI * 0.5 || PI * 0.5 < angle {
                player_sprite.flip_x = true;
            } else {
                player_sprite.flip_x = false;
            }

            // 魔法の発射
            if get_fire_trigger(buttons, gamepad_buttons, &my_gamepad) && player.cooltime == 0 {
                let normalized = player.pointer.normalize();

                for _ in 0..BULLETS_PER_FIRE {
                    let angle_with_random = angle + (random::<f32>() - 0.5) * BULLET_SCATTERING;
                    let direction = Vec2::from_angle(angle_with_random);
                    let range = WITCH_COLLIDER_RADIUS + BULLET_RADIUS + BULLET_SPAWNING_MARGIN;
                    let bullet_position =
                        player_transform.translation.truncate() + range * normalized;
                    spawn_bullet(
                        &mut commands,
                        assets.asset.clone(),
                        bullet_position,
                        direction * BULLET_SPEED,
                        Some(player.uuid),
                        &assets,
                        &audio,
                        &config,
                    );
                    let serialized = bincode::serialize(&RemoteMessage::Fire {
                        uuid: player.uuid,
                        x: bullet_position.x,
                        y: bullet_position.y,
                        vx: direction.x * BULLET_SPEED,
                        vy: direction.y * BULLET_SPEED,
                    })
                    .unwrap();
                    writer.send(ClientMessage::Binary(serialized));
                }

                player.cooltime = BULLET_COOLTIME;
            } else {
                player.cooltime = (player.cooltime - 1).max(0);
            }
        } else {
            player_force.force = Vec2::ZERO;

            player.cooltime = (player.cooltime - 1).max(0);
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // FixedUpdateでスケジュールされたシステムには、before(PhysicsSet::SyncBackend) でスケジュールをする必要があります
            // これがない場合、変更が正しく rapier に通知されず、数回に一度のような再現性の低いバグが起きることがあるようです
            // https://taintedcoders.com/bevy/physics/rapier
            FixedUpdate,
            update_player
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
