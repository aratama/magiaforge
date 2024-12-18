use crate::asset::GameAssets;
use crate::constant::{ENTITY_LAYER_Z, MAX_WANDS};
use crate::controller::remote::send_remote_message;
use crate::controller::remote::RemoteMessage;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::gold::Gold;
use crate::entity::life::Life;
use crate::equipment::EquipmentType;
use crate::input::{get_direction, get_fire_trigger};
use crate::se::{SEEvent, SE};
use crate::states::{GameMenuState, GameState};
use bevy::core::FrameCount;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Equipment {
    pub equipment_type: EquipmentType,
    pub price: u32,
}

/// 操作可能なプレイヤーキャラクターを表します
#[derive(Component, Debug, Clone)]
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

/// プレイヤーの移動
/// ここではまだ ExternalForce へはアクセスしません
/// Actor側で ExternalForce にアクセスして、移動を行います
fn move_player(
    mut player_query: Query<&mut Actor, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok(mut actor) = player_query.get_single_mut() {
        match *menu.get() {
            GameMenuState::Closed => {
                actor.move_direction = get_direction(keys, &gamepads);
            }
            _ => {
                actor.move_direction = Vec2::ZERO;
            }
        }
    }
}

fn apply_intensity_by_lantern(mut player_query: Query<&mut Actor, With<Player>>) {
    if let Ok(mut actor) = player_query.get_single_mut() {
        let equiped_lantern = actor.equipments.iter().any(|e| match e {
            Some(Equipment {
                equipment_type: EquipmentType::Lantern,
                ..
            }) => true,
            _ => false,
        });
        actor.intensity = if equiped_lantern { 3.0 } else { 0.0 };
    }
}

/// 魔法の発射
fn trigger_bullet(
    mut player_query: Query<&mut Actor, (With<Player>, Without<Camera2d>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    gamepads: Query<&Gamepad>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        match *menu.get() {
            GameMenuState::Closed => {
                if get_fire_trigger(&buttons, &gamepads) {
                    player.fire_state = ActorFireState::Fire;
                } else {
                    player.fire_state = ActorFireState::Idle;
                }
                if buttons.pressed(MouseButton::Right) {
                    player.fire_state_secondary = ActorFireState::Fire;
                } else {
                    player.fire_state_secondary = ActorFireState::Idle;
                }
            }
            _ => {
                player.fire_state = ActorFireState::Idle;
                player.fire_state_secondary = ActorFireState::Idle;
            }
        }
    }
}

fn switch_wand(
    mut witch_query: Query<&mut Actor, With<Player>>,
    mut wheel: EventReader<MouseWheel>,
    mut writer: EventWriter<SEEvent>,
) {
    for event in wheel.read() {
        if let Ok(mut actor) = witch_query.get_single_mut() {
            let next = (actor.current_wand as i32 - event.y.signum() as i32)
                .max(0)
                .min(MAX_WANDS as i32 - 2) as usize;
            if next != actor.current_wand {
                actor.current_wand = next;
                writer.send(SEEvent::new(SE::Switch));
            }
        }
    }
}

fn pick_gold(
    mut commands: Commands,
    mut gold_query: Query<(Entity, &Transform, &mut ExternalForce), With<Gold>>,
    mut player_query: Query<(&mut Actor, &Transform), With<Player>>,
    mut writer: EventWriter<SEEvent>,
) {
    if let Ok((mut actor, player_transform)) = player_query.get_single_mut() {
        let mut got_gold = false;

        for (gold, gold_transform, mut gold_force) in gold_query.iter_mut() {
            let diff =
                player_transform.translation.truncate() - gold_transform.translation.truncate();
            if diff.length() < 16.0 {
                actor.golds += 1;
                got_gold = true;
                commands.entity(gold).despawn_recursive();
            } else if diff.length() < 48.0 {
                gold_force.force = diff.normalize() * 1000.0;
            } else {
                gold_force.force = Vec2::ZERO;
            }
        }

        if got_gold {
            writer.send(SEEvent::pos(
                SE::PickUp,
                player_transform.translation.truncate(),
            ));
        }
    }
}

fn die_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_query: Query<(Entity, &Actor, &Life, &Transform), With<Player>>,
    mut writer: EventWriter<ClientMessage>,
    mut game: EventWriter<SEEvent>,
    websocket: Res<WebSocketState>,
) {
    if let Ok((entity, actor, player_life, transform)) = player_query.get_single() {
        if player_life.life <= 0 {
            commands.entity(entity).despawn_recursive();

            game.send(SEEvent::pos(SE::Cry, transform.translation.truncate()));

            // ダウンのアニメーションを残す
            commands.spawn((
                StateScoped(GameState::InGame),
                AseSpriteAnimation {
                    aseprite: assets.player.clone(),
                    animation: "down".into(),
                },
                Transform::from_translation(
                    transform.translation.truncate().extend(ENTITY_LAYER_Z),
                ),
                PointLight2d {
                    color: Color::hsl(240.0, 1.0, 0.8),
                    intensity: 1.0,
                    radius: 100.0,
                    falloff: 1.0,
                    ..default()
                },
            ));

            send_remote_message(
                &mut writer,
                websocket.ready_state == ReadyState::OPEN,
                &RemoteMessage::Die {
                    sender: actor.uuid,
                    uuid: actor.uuid,
                },
            );
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
            (
                move_player,
                trigger_bullet,
                pick_gold,
                die_player,
                apply_intensity_by_lantern,
                switch_wand,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
