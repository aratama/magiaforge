use super::remote::RemoteMessage;
use crate::asset::GameAssets;
use crate::command::GameCommand;
use crate::entity::actor::{Actor, ActorFireState, ActorMoveState};
use crate::entity::gold::{spawn_gold, Gold};
use crate::input::{get_direction, get_fire_trigger, MyGamepad};
use crate::states::{GameMenuState, GameState};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::ClientMessage;

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
    mut player_query: Query<(&mut Actor, &mut ExternalForce), (With<Player>, Without<Camera2d>)>,
    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    menu: Res<State<GameMenuState>>,
) {
    let force = 50000.0;
    let direction = get_direction(keys, axes, &my_gamepad);
    if let Ok((mut actor, mut player_force)) = player_query.get_single_mut() {
        if *menu == GameMenuState::Closed {
            player_force.force = direction * force;
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

fn pick_gold(
    mut commands: Commands,
    gold_query: Query<Entity, With<Gold>>,
    mut player_query: Query<&mut Player>,
    mut collision_events: EventReader<CollisionEvent>,
    mut writer: EventWriter<GameCommand>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                let _ = process_pick_event(
                    &mut commands,
                    &gold_query,
                    &mut player_query,
                    &mut writer,
                    &a,
                    &b,
                ) || process_pick_event(
                    &mut commands,
                    &gold_query,
                    &mut player_query,
                    &mut writer,
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
    writer: &mut EventWriter<GameCommand>,
    gold: &Entity,
    player: &Entity,
) -> bool {
    if let Ok(gold) = gold_query.get(*gold) {
        if let Ok(mut player) = player_query.get_mut(*player) {
            player.golds += 1;
            writer.send(GameCommand::SECancel);
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
    mut writer: EventWriter<ClientMessage>,
    mut game: EventWriter<GameCommand>,
) {
    if let Ok((entity, player, actor, transform)) = player_query.get_single() {
        if actor.life <= 0 {
            commands.entity(entity).despawn_recursive();

            game.send(GameCommand::SEHiyoko);
            game.send(GameCommand::StateMainMenu);

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
            (
                move_player,
                trigger_bullet,
                pick_gold,
                die_player,
                switch_intensity,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
