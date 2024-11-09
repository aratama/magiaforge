use super::remote::RemoteMessage;
use crate::asset::GameAssets;
use crate::command::GameCommand;
use crate::constant::MAX_WANDS;
use crate::entity::actor::{Actor, ActorFireState, ActorMoveState};
use crate::entity::bullet::BulletType;
use crate::entity::gold::{spawn_gold, Gold};
use crate::entity::witch::{self, Witch};
use crate::input::{get_direction, get_fire_trigger, MyGamepad};
use crate::states::{GameMenuState, GameState};
use crate::wand::Spell;
use bevy::core::FrameCount;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};

const PLAYER_MOVE_FORCE: f32 = 50000.0;

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
    mut player_query: Query<(&mut Actor, &mut ExternalForce), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok((mut actor, mut player_force)) = player_query.get_single_mut() {
        if *menu == GameMenuState::Closed {
            let direction = get_direction(keys, axes, &gamepads);
            player_force.force = direction * PLAYER_MOVE_FORCE;
            actor.move_state = if 0.0 < player_force.force.length() {
                ActorMoveState::Run
            } else {
                ActorMoveState::Idle
            };
        } else {
            player_force.force = Vec2::ZERO;
            actor.move_state = ActorMoveState::Idle;
        }
    }
}

fn switch_intensity(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Actor, (With<Player>, Without<Camera2d>)>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        if let Ok(mut actor) = player_query.get_single_mut() {
            actor.intensity = if actor.intensity == 0.0 { 3.0 } else { 0.0 };
        }
    }
}

/// 魔法の発射
fn trigger_bullet(
    mut player_query: Query<&mut Actor, (With<Player>, Without<Camera2d>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if *menu == GameMenuState::Closed && get_fire_trigger(buttons, gamepad_buttons, &my_gamepad)
        {
            player.fire_state = ActorFireState::Fire;
        } else {
            player.fire_state = ActorFireState::Idle;
        }
    }
}

fn switch_wand(mut witch_query: Query<&mut Witch>, mut wheel: EventReader<MouseWheel>) {
    for event in wheel.read() {
        if let Ok(mut witch) = witch_query.get_single_mut() {
            witch.current_wand = (witch.current_wand as i32 - event.y.signum() as i32)
                .max(0)
                .min(MAX_WANDS as i32 - 1) as usize;
        }
    }
}

fn select_bullet_type(mut witch_query: Query<(&Witch, &mut Actor)>) {
    if let Ok((witch, mut actor)) = witch_query.get_single_mut() {
        actor.bullet_type = match &witch.wands[witch.current_wand] {
            None => BulletType::BlueBullet,
            Some(wand) => match wand.slots[0] {
                Some(Spell::MagicBolt) => BulletType::BlueBullet,
                Some(Spell::PurpleBolt) => BulletType::PurpleBullet,
                Some(Spell::SlimeCharge) => BulletType::SlimeAttackBullet,
                None => BulletType::BlueBullet,
            },
        }
    }
}

fn pick_gold(
    mut commands: Commands,
    mut gold_query: Query<(Entity, &Transform, &mut ExternalForce), With<Gold>>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut writer: EventWriter<GameCommand>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        let mut got_gold = false;

        for (gold, gold_transform, mut gold_force) in gold_query.iter_mut() {
            let diff =
                player_transform.translation.truncate() - gold_transform.translation.truncate();
            if diff.length() < 16.0 {
                player.golds += 1;
                got_gold = true;
                commands.entity(gold).despawn_recursive();
            } else if diff.length() < 48.0 {
                gold_force.force = diff.normalize() * 1000.0;
            }
        }

        if got_gold {
            writer.send(GameCommand::SECancel(Some(
                player_transform.translation.truncate(),
            )));
        }
    }
}

fn die_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_query: Query<(Entity, &Player, &Actor, &Transform)>,
    mut writer: EventWriter<ClientMessage>,
    mut game: EventWriter<GameCommand>,
    websocket: Res<WebSocketState>,
) {
    if let Ok((entity, player, actor, transform)) = player_query.get_single() {
        if actor.life <= 0 {
            commands.entity(entity).despawn_recursive();

            game.send(GameCommand::SEHiyoko(Some(
                transform.translation.truncate(),
            )));
            game.send(GameCommand::StateMainMenu);

            for _ in 0..player.golds {
                spawn_gold(
                    &mut commands,
                    &assets,
                    transform.translation.x,
                    transform.translation.y,
                );
            }

            if websocket.ready_state == ReadyState::OPEN {
                writer.send(ClientMessage::Binary(
                    bincode::serialize(&RemoteMessage::Die {
                        sender: actor.uuid,
                        uuid: actor.uuid,
                    })
                    .unwrap(),
                ));
            }
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
                switch_intensity,
                switch_wand,
                select_bullet_type,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
