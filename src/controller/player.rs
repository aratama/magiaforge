use crate::asset::GameAssets;
use crate::command::GameCommand;
use crate::constant::{MAX_ITEMS_IN_EQUIPMENT, MAX_WANDS};
use crate::controller::remote::RemoteMessage;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::gold::{spawn_gold, Gold};
use crate::equipment::Equipment;
use crate::input::{get_direction, get_fire_trigger, MyGamepad};
use crate::inventory::Inventory;
use crate::states::{GameMenuState, GameState};
use bevy::core::FrameCount;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};

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
    pub inventory: Inventory,
    pub equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
}

/// プレイヤーの移動
/// ここではまだ ExternalForce へはアクセスしません
fn move_player(
    mut player_query: Query<&mut Actor, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok(mut actor) = player_query.get_single_mut() {
        match *menu.get() {
            GameMenuState::Closed => {
                actor.move_direction = get_direction(keys, axes, &gamepads);
            }
            _ => {
                actor.move_direction = Vec2::ZERO;
            }
        }
    }
}

fn apply_intensity_by_lantern(mut player_query: Query<(&Player, &mut Actor)>) {
    if let Ok((player, mut actor)) = player_query.get_single_mut() {
        let equiped_lantern = player.equipments.iter().any(|e| match e {
            Some(Equipment::Lantern) => true,
            _ => false,
        });
        actor.intensity = if equiped_lantern { 3.0 } else { 0.0 };
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
        match *menu.get() {
            GameMenuState::Closed => {
                if get_fire_trigger(buttons, gamepad_buttons, &my_gamepad) {
                    player.fire_state = ActorFireState::Fire;
                } else {
                    player.fire_state = ActorFireState::Idle;
                }
            }
            _ => {
                player.fire_state = ActorFireState::Idle;
            }
        }
    }
}

fn switch_wand(
    mut witch_query: Query<&mut Actor, With<Player>>,
    mut wheel: EventReader<MouseWheel>,
    mut writer: EventWriter<GameCommand>,
) {
    for event in wheel.read() {
        if let Ok(mut actor) = witch_query.get_single_mut() {
            let next = (actor.current_wand as i32 - event.y.signum() as i32)
                .max(0)
                .min(MAX_WANDS as i32 - 1) as usize;
            if next != actor.current_wand {
                actor.current_wand = next;
                writer.send(GameCommand::SESwitch(None));
            }
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
            writer.send(GameCommand::SEPickUp(Some(
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

            game.send(GameCommand::SECry(Some(transform.translation.truncate())));
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
                apply_intensity_by_lantern,
                switch_wand,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
