use super::super::asset::GameAssets;
use super::super::constant::*;
use super::super::gamepad::{get_direction, get_fire_trigger, MyGamepad};
use super::super::states::GameState;
use super::remote::RemoteMessage;
use crate::game::config::{GameConfig, GameConfigPlugin};
use crate::game::entity::actor::Actor;
use crate::game::entity::bullet::add_bullet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_websocket_sync::ClientMessage;
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
pub struct Player {}

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
    mut camera_query: Query<&mut Transform, (With<Camera>, With<Camera2d>, Without<Player>)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    buttons: Res<ButtonInput<MouseButton>>,

    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
) {
    let force = 50000.0;

    let direction = get_direction(keys, axes, &my_gamepad);

    if let Ok((mut player, mut player_transform, mut player_force, _, mut player_sprite)) =
        player_query.get_single_mut()
    {
        player_transform.translation.z =
            ENTITY_LAYER_Z - player_transform.translation.y * Z_ORDER_SCALE;
        player_force.force = direction * force;

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x +=
                (player_transform.translation.x - camera_transform.translation.x) * CAMERA_SPEED;
            camera_transform.translation.y +=
                (player_transform.translation.y - camera_transform.translation.y) * CAMERA_SPEED;

            // プレイヤーの向き
            let angle = player.pointer.to_angle();

            // println!("angle: {}", angle);
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
                    add_bullet(
                        &mut commands,
                        assets.asset.clone(),
                        player_transform.translation.truncate() + normalized * 10.0,
                        direction * BULLET_SPEED,
                    );
                }

                player.cooltime = BULLET_COOLTIME;
            } else {
                player.cooltime = (player.cooltime - 1).max(0);
            }
        }
    }
}

fn sync(
    query: Query<(&Actor, &GlobalTransform)>,
    mut writer: EventWriter<ClientMessage>,
    config: Res<GameConfig>,
) {
    if config.online {
        if let Ok((actor, transform)) = query.get_single() {
            let translation = transform.translation();
            let value = RemoteMessage::Ping {
                uuid: actor.uuid,
                x: translation.x,
                y: translation.y,
            };
            let message = bincode::serialize(&value).unwrap();
            writer.send(ClientMessage::Binary(message));
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_player.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(FixedUpdate, sync.run_if(in_state(GameState::InGame)));
    }
}
