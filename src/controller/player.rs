use super::remote::RemoteMessage;
use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::entity::actor::Actor;
use crate::entity::bullet::{spawn_bullet, BULLET_RADIUS, BULLET_SPAWNING_MARGIN};
use crate::entity::gold::{spawn_gold, Gold};
use crate::entity::witch::WITCH_COLLIDER_RADIUS;
use crate::hud::overlay::OverlayNextState;
use crate::input::{get_direction, get_fire_trigger, MyGamepad};
use crate::states::{GameMenuState, GameState};
use crate::world::CurrentLevel;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::ClientMessage;
use rand::random;

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
    pub golds: i32,
    pub last_idle_frame_count: FrameCount,
    pub last_ilde_x: f32,
    pub last_ilde_y: f32,
    pub last_idle_vx: f32,
    pub last_idle_vy: f32,
    pub last_idle_life: i32,
    pub last_idle_max_life: i32,
}

/// プレイヤーの移動
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut ExternalForce, (With<Player>, Without<Camera2d>)>,
    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    menu: Res<State<GameMenuState>>,
) {
    let force = 50000.0;
    let direction = get_direction(keys, axes, &my_gamepad);
    if let Ok(mut player_force) = player_query.get_single_mut() {
        if *menu == GameMenuState::Close {
            player_force.force = direction * force;
        } else {
            player_force.force = Vec2::ZERO;
        }
    }
}

/// 魔法の発射
fn fire_bullet(
    mut player_query: Query<(&mut Actor, &mut Transform), (With<Player>, Without<Camera2d>)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    buttons: Res<ButtonInput<MouseButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    mut writer: EventWriter<ClientMessage>,
    menu: Res<State<GameMenuState>>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
    current: Res<CurrentLevel>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        if player.life <= 0 {
            return;
        }

        if *menu == GameMenuState::Close {
            // 魔法の発射

            if get_fire_trigger(buttons, gamepad_buttons, &my_gamepad) && player.cooltime == 0 {
                let normalized = player.pointer.normalize();
                let angle = player.pointer.to_angle();
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

                    if let Some(level) = current.0 {
                        let serialized = bincode::serialize(&RemoteMessage::Fire {
                            uuid: player.uuid,
                            level,
                            x: bullet_position.x,
                            y: bullet_position.y,
                            vx: direction.x * BULLET_SPEED,
                            vy: direction.y * BULLET_SPEED,
                        })
                        .unwrap();
                        writer.send(ClientMessage::Binary(serialized));
                    }
                }

                player.cooltime = BULLET_COOLTIME;
            } else {
                player.cooltime = (player.cooltime - 1).max(0);
            }
        } else {
            player.cooltime = (player.cooltime - 1).max(0);
        }
    }
}

fn pick_gold(
    mut commands: Commands,
    gold_query: Query<Entity, With<Gold>>,
    mut player_query: Query<&mut Player>,
    mut collision_events: EventReader<CollisionEvent>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                let _ = process_pick_event(
                    &mut commands,
                    &gold_query,
                    &mut player_query,
                    &assets,
                    &audio,
                    &a,
                    &b,
                ) || process_pick_event(
                    &mut commands,
                    &gold_query,
                    &mut player_query,
                    &assets,
                    &audio,
                    &b,
                    &a,
                );
            }
            _ => {}
        }
    }
}

fn process_pick_event(
    commands: &mut Commands,
    gold_query: &Query<Entity, With<Gold>>,
    player_query: &mut Query<&mut Player>,
    assets: &Res<GameAssets>,
    audio: &Res<Audio>,
    gold: &Entity,
    player: &Entity,
) -> bool {
    if let Ok(gold) = gold_query.get(*gold) {
        if let Ok(mut player) = player_query.get_mut(*player) {
            player.golds += 1;
            audio.play(assets.cancel.clone());
            commands.entity(gold).despawn_recursive();
            return true;
        }
    }
    false
}

fn die_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_query: Query<(Entity, &Player, &Actor, &Transform)>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    audio: Res<Audio>,
    mut writer: EventWriter<ClientMessage>,
) {
    if let Ok((entity, player, actor, transform)) = player_query.get_single() {
        if actor.life <= 0 {
            commands.entity(entity).despawn_recursive();

            audio.play(assets.hiyoko.clone());

            *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));

            for _ in 0..player.golds {
                spawn_gold(
                    &mut commands,
                    &assets,
                    transform.translation.x,
                    transform.translation.y,
                );
            }

            writer.send(ClientMessage::Binary(
                bincode::serialize(&RemoteMessage::Die { uuid: actor.uuid }).unwrap(),
            ));
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
            (move_player, fire_bullet, pick_gold, die_player)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
