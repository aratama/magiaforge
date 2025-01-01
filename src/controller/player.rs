use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::component::counter::CounterAnimated;
use crate::constant::ENTITY_LAYER_Z;
use crate::constant::MAX_WANDS;
use crate::controller::remote::send_remote_message;
use crate::controller::remote::RemoteMessage;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorState;
use crate::entity::gold::Gold;
use crate::component::life::Life;
use crate::equipment::EquipmentType;
use crate::input::get_direction;
use crate::input::get_fire_trigger;
use crate::page::in_game::GameLevel;
use crate::page::in_game::Interlevel;
use crate::physics::InGameTime;
use crate::player_state::PlayerState;
use crate::se::SEEvent;
use crate::se::SE;
use crate::spell::SpellType;
use crate::states::GameMenuState;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
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

    /// 起き上がりアニメーションのフレーム数
    pub getting_up: u32,
    pub discovered_spells: HashSet<SpellType>,
}

impl Player {
    pub fn update_discovered_spells(&mut self, actor: &Actor) {
        self.discovered_spells = self
            .discovered_spells
            .union(&actor.get_owned_spell_types())
            .cloned()
            .collect();
    }
}

/// プレイヤーの移動
/// ここではまだ ExternalForce へはアクセスしません
/// Actor側で ExternalForce にアクセスして、移動を行います
fn move_player(
    mut player_query: Query<(&mut Actor, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok((mut actor, player)) = player_query.get_single_mut() {
        if player.getting_up > 0 {
            return;
        }
        match *menu.get() {
            GameMenuState::Closed => {
                let direction = get_direction(keys);
                actor.move_direction = direction;
                actor.state = if direction != Vec2::ZERO {
                    ActorState::Run
                } else {
                    ActorState::Idle
                };
            }
            _ => {
                actor.move_direction = Vec2::ZERO;
                actor.state = ActorState::Idle;
            }
        }
    }
}

fn apply_intensity_by_lantern(mut player_query: Query<&mut Actor, With<Player>>) {
    if let Ok(mut actor) = player_query.get_single_mut() {
        let mut point_light_radius: f32 = 0.0;
        for equiped_lantern in actor.equipments {
            match equiped_lantern {
                Some(Equipment {
                    equipment_type: EquipmentType::Lantern,
                    ..
                }) => {
                    point_light_radius += 160.0;
                }
                _ => {}
            }
        }
        actor.point_light_radius = point_light_radius;
    }
}

/// 魔法の発射
pub fn actor_cast(
    mut player_query: Query<(&mut Actor, &Player), Without<Camera2d>>,
    buttons: Res<ButtonInput<MouseButton>>,
    menu: Res<State<GameMenuState>>,
    camera_query: Query<&GameCamera>,
) {
    let camera = camera_query.single();
    if camera.target.is_some() {
        return;
    }

    if let Ok((mut actor, player)) = player_query.get_single_mut() {
        if player.getting_up > 0 {
            return;
        }
        match *menu.get() {
            GameMenuState::Closed => {
                actor.fire_state = if get_fire_trigger(&buttons) {
                    ActorFireState::Fire
                } else {
                    ActorFireState::Idle
                };
                if buttons.pressed(MouseButton::Right) {
                    actor.fire_state_secondary = ActorFireState::Fire;
                } else {
                    actor.fire_state_secondary = ActorFireState::Idle;
                }
            }
            _ => {
                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;
            }
        }
    }
}

fn switch_wand(
    mut witch_query: Query<&mut Actor, With<Player>>,
    mut wheel: EventReader<MouseWheel>,
    mut writer: EventWriter<SEEvent>,
    state: Res<State<GameMenuState>>,
) {
    if *state == GameMenuState::PauseMenuOpen {
        return;
    }
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
    mut gold_query: Query<(Entity, &mut Gold, &Transform)>,
    mut player_query: Query<(&mut Actor, &Transform), With<Player>>,
    mut writer: EventWriter<SEEvent>,
) {
    if let Ok((mut actor, player_transform)) = player_query.get_single_mut() {
        let mut got_gold = false;

        for (gold_entity, mut gold, gold_transform) in gold_query.iter_mut() {
            let diff =
                player_transform.translation.truncate() - gold_transform.translation.truncate();
            if diff.length() < 16.0 {
                actor.golds += 1;
                got_gold = true;
                commands.entity(gold_entity).despawn_recursive();
            } else if diff.length() < 48.0 {
                gold.magnet = true;
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
    player_query: Query<(Entity, &Actor, &Life, &Transform, &Player)>,
    mut writer: EventWriter<ClientMessage>,
    mut game: EventWriter<SEEvent>,
    websocket: Res<WebSocketState>,
    mut next: ResMut<Interlevel>,
) {
    if let Ok((entity, actor, player_life, transform, player)) = player_query.get_single() {
        if player_life.life <= 0 {
            commands.entity(entity).despawn_recursive();

            game.send(SEEvent::pos(SE::Cry, transform.translation.truncate()));

            // ダウン後はキャンプに戻る
            next.next_level = GameLevel::Level(0);

            // 次のシーンのためにプレイヤーの状態を保存
            next.next_state = PlayerState::from_player(&player, &actor, &player_life);

            // 全回復させてから戻る
            next.next_state.life = next.next_state.max_life;

            // 倒れるアニメーションを残す
            commands.spawn((
                StateScoped(GameState::InGame),
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.witch.clone(),
                    animation: "get_down".into(),
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

fn getting_up(mut player_query: Query<(&mut Actor, &mut Player)>, in_game_time: Res<InGameTime>) {
    if !in_game_time.active {
        return;
    }

    for (mut actor, mut player) in player_query.iter_mut() {
        if 0 < player.getting_up {
            player.getting_up -= 1;
            if player.getting_up == 0 {
                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;
            } else {
                actor.state = ActorState::GettingUp;
            }
        }
    }
}

/// マウスポインタの位置を参照してプレイヤーアクターのポインターを設定します
/// この関数はプレイヤーのモジュールに移動する？
fn update_pointer_by_mouse(
    mut player_query: Query<(&mut Actor, &GlobalTransform), With<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    state: Res<State<GameMenuState>>,
) {
    if *state.get() != GameMenuState::Closed {
        return;
    }

    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        if player.state != ActorState::GettingUp {
            if let Ok(window) = q_window.get_single() {
                if let Some(cursor_in_screen) = window.cursor_position() {
                    if let Ok((camera, camera_global_transform)) = camera_query.get_single() {
                        if let Ok(mouse_in_world) =
                            camera.viewport_to_world(camera_global_transform, cursor_in_screen)
                        {
                            player.pointer = mouse_in_world.origin.truncate()
                                - player_transform.translation().truncate();
                        }
                    }
                }
            }
        }
    }
}

fn insert_discovered_spells(mut player_query: Query<(&mut Player, &Actor)>) {
    for (mut player, actor) in player_query.iter_mut() {
        player.update_discovered_spells(actor);
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
                update_pointer_by_mouse,
                getting_up,
                move_player,
                actor_cast,
                pick_gold,
                die_player,
                apply_intensity_by_lantern,
                switch_wand,
                insert_discovered_spells,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
