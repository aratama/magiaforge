use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorState;
use crate::camera::GameCamera;
use crate::component::counter::CounterAnimated;
use crate::component::metamorphosis::Metamorphosed;
use crate::constant::ENTITY_LAYER_Z;
use crate::constant::MAX_WANDS;
use crate::controller::remote::send_remote_message;
use crate::controller::remote::RemoteMessage;
use crate::entity::bullet::Bullet;
use crate::entity::bullet::Trigger;
use crate::entity::gold::Gold;
use crate::input::get_direction;
use crate::interpreter::Cmd;
use crate::interpreter::InterpreterEvent;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::PICK_UP;
use crate::se::SEN;
use crate::se::SWITCH;
use crate::set::FixedUpdateInGameSet;
use crate::set::FixedUpdatePlayerActiveSet;
use crate::spell::Spell;
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
use std::collections::HashSet;

/// プレイヤーキャラクター本体を表します
/// Playerは最大で1体です
/// このアクターが消滅したらゲームオーバーとなります
#[derive(Component, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub last_idle_frame_count: FrameCount,
    pub last_ilde_x: f32,
    pub last_ilde_y: f32,
    pub last_idle_vx: f32,
    pub last_idle_vy: f32,
    pub last_idle_life: u32,
    pub last_idle_max_life: u32,
    pub discovered_spells: HashSet<Spell>,
    pub broken_chests: u32,
}

impl Player {
    pub fn new(name: String, discovered_spells: &HashSet<Spell>) -> Self {
        Self {
            name,
            last_idle_frame_count: FrameCount(0),
            last_ilde_x: 0.0,
            last_ilde_y: 0.0,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: 0,
            last_idle_max_life: 0,
            discovered_spells: discovered_spells.clone(),
            broken_chests: 0,
        }
    }

    pub fn update_discovered_spells(&mut self, actor: &Actor) {
        self.discovered_spells = self
            .discovered_spells
            .union(&actor.get_owned_spell_types())
            .cloned()
            .collect();
    }
}

/// プレイヤーの操作対象となっているアクターを表します
/// Playerは最大1体ですが、分身によってPlayerControlledは複数存在することがあります
#[derive(Component, Debug, Clone)]
pub struct PlayerControlled;

/// 現在プレイヤーが操作対象としている召喚キャラクターです
/// これを付与したアクターは、そのエンティティ本来の操作と、プレイヤーの操作が、衝突しないように注意します
#[derive(Component, Debug, Clone)]
pub struct PlayerServant;

#[derive(Component, Debug, Clone)]
pub struct PlayerDown;

/// プレイヤーの移動
/// ここではまだ ExternalForce へはアクセスしません
/// Actor側で ExternalForce にアクセスして、移動を行います
/// メニューを開いた瞬間に立ち止まるため、FixedUpdateInGameSetにスケジュールされています
fn move_player(
    player_query: Query<&Witch, With<Player>>,
    mut controlled_query: Query<&mut Actor, With<PlayerControlled>>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<State<GameMenuState>>,
) {
    let Ok(_) = player_query.get_single() else {
        return;
    };

    let direction = get_direction(&keys);
    let state = if direction != Vec2::ZERO {
        ActorState::Run
    } else {
        ActorState::Idle
    };

    for mut actor in controlled_query.iter_mut() {
        if 0 < actor.getting_up {
            continue;
        }

        match *menu.get() {
            GameMenuState::Closed => {
                actor.move_direction = direction;
                actor.state = state;
            }
            _ => {
                actor.move_direction = Vec2::ZERO;
                actor.state = ActorState::Idle;
            }
        }
    }
}

/// 魔法の発射
pub fn actor_cast(
    player_query: Query<&Witch, With<Player>>,
    mut controlled_query: Query<&mut Actor, (With<PlayerControlled>, Without<Camera2d>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    menu: Res<State<GameMenuState>>,
    mut se: EventWriter<SEEvent>,
    camera_query: Query<&GameCamera, With<Camera2d>>,
) {
    let camera = camera_query.single();
    if camera.target.is_some() {
        return;
    }

    let Ok(_) = player_query.get_single() else {
        return;
    };

    for mut actor in controlled_query.iter_mut() {
        if 0 < actor.getting_up {
            continue;
        }

        match *menu.get() {
            GameMenuState::Closed => {
                // プライマリ魔法の発射
                actor.fire_state = if buttons.pressed(MouseButton::Left) {
                    ActorFireState::Fire
                } else {
                    ActorFireState::Idle
                };

                // 杖が空の場合の失敗音
                if actor.wands[actor.current_wand as usize].is_empty()
                    && buttons.just_pressed(MouseButton::Left)
                {
                    se.send(SEEvent::new(SEN));
                }

                // セカンダリ魔法の発射
                if buttons.pressed(MouseButton::Right) {
                    actor.fire_state_secondary = ActorFireState::Fire;
                } else {
                    actor.fire_state_secondary = ActorFireState::Idle;
                }

                // 杖が空の場合の失敗音
                if actor.wands[actor.wands.len() - 1].is_empty()
                    && buttons.just_pressed(MouseButton::Right)
                {
                    se.send(SEEvent::new(SEN));
                }
            }
            _ => {
                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;
            }
        }
    }
}

pub fn rotate_holded_bullets(
    player_query: Query<(Entity, &Actor, &Transform), (With<Player>, Without<Camera2d>)>,
    mut bullet_query: Query<(&Bullet, &mut Transform), Without<Actor>>,
) {
    if let Ok((player_entity, actor, player_transform)) = player_query.get_single() {
        let to = player_transform.translation.truncate() + actor.pointer;
        for (bullet, mut transform) in bullet_query.iter_mut() {
            if let Some((holder_entity, trigger)) = bullet.holder {
                if holder_entity == player_entity && trigger == Trigger::Primary {
                    let direction = to - transform.translation.truncate();
                    transform.rotation = Quat::from_rotation_z(direction.to_angle());
                }
            }
        }
    }
}

pub fn release_holded_bullets(
    player_query: Query<(Entity, &Actor, &Transform), (With<Player>, Without<Camera2d>)>,
    mut bullet_query: Query<(&mut Bullet, &mut Velocity, &mut Transform), Without<Actor>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        if let Ok((player_entity, actor, player_transform)) = player_query.get_single() {
            let to = player_transform.translation.truncate() + actor.pointer;
            for (mut bullet, mut velocity, mut transform) in bullet_query.iter_mut() {
                if let Some((holder_entity, trigger)) = bullet.holder {
                    if holder_entity == player_entity && trigger == Trigger::Primary {
                        let direction = to - transform.translation.truncate();
                        transform.rotation = Quat::from_rotation_z(direction.to_angle());
                        velocity.linvel = direction.normalize() * 300.0;
                        bullet.holder = None;
                    }
                }
            }
        }
    }
}

fn switch_wand(
    mut witch_query: Query<&mut Actor, With<PlayerControlled>>,
    mut wheel: EventReader<MouseWheel>,
    mut writer: EventWriter<SEEvent>,
    state: Res<State<GameMenuState>>,
) {
    if *state == GameMenuState::PauseMenuOpen {
        return;
    }
    for event in wheel.read() {
        for mut actor in witch_query.iter_mut() {
            let next = (actor.current_wand as i32 - event.y.signum() as i32)
                .max(0)
                .min(MAX_WANDS as i32 - 2) as u8;
            if next != actor.current_wand {
                actor.current_wand = next;
                writer.send(SEEvent::new(SWITCH));
            }
        }
    }
}

fn pick_gold(
    mut commands: Commands,
    mut gold_query: Query<(Entity, &mut Gold, &Transform)>,
    mut player_query: Query<(&mut Actor, &Transform), (With<Player>, With<Witch>)>,
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
                PICK_UP,
                player_transform.translation.truncate(),
            ));
        }
    }
}

fn die_player(
    mut commands: Commands,
    registry: Registry,
    player_query: Query<(&Actor, &Transform, &Player, Option<&Metamorphosed>)>,
    mut writer: EventWriter<ClientMessage>,
    websocket: Res<WebSocketState>,
    mut interpreter: EventWriter<InterpreterEvent>,
    mut next: ResMut<LevelSetup>,
) {
    if let Ok((actor, transform, player, morph)) = player_query.get_single() {
        if actor.life <= 0 {
            // 倒れるアニメーションを残す
            commands.spawn((
                PlayerDown,
                StateScoped(GameState::InGame),
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: registry.assets.witch.clone(),
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

            recovery(&mut next, &mut interpreter, &morph, player, actor);
        }
    }
}

pub fn recovery(
    level: &mut LevelSetup,
    interpreter: &mut EventWriter<InterpreterEvent>,
    morph: &Option<&Metamorphosed>,
    player: &Player,
    current_actor: &Actor,
) {
    // ダウン後はキャンプに戻る
    level.next_level = GameLevel::new("level_0_0");

    // 元に戻ったあとのActorを取得します
    // current_actor は変身済みのものである可能性があることに注意します
    let actor = morph.map(|m| &m.original_actor).unwrap_or(current_actor);

    // 次のシーンのためにプレイヤーの状態を保存
    let mut player_state = PlayerState::from_player(&player, &actor);
    // 全回復させてから戻る
    player_state.life = player_state.max_life;
    level.next_state = Some(player_state);

    // 数秒後にキャンプに戻る
    interpreter.send(InterpreterEvent::Play {
        commands: vec![Cmd::Wait { count: 300 }, Cmd::Home],
    });
}

/// マウスポインタの位置を参照してプレイヤーアクターのポインターを設定します
/// この関数はプレイヤーのモジュールに移動する？
fn update_pointer_by_mouse(
    mut player_query: Query<(&mut Actor, &GlobalTransform), With<PlayerControlled>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<PlayerControlled>)>,
) {
    for (mut player, player_transform) in player_query.iter_mut() {
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

fn insert_discovered_spells(mut player_query: Query<(&mut Player, &Actor)>) {
    for (mut player, actor) in player_query.iter_mut() {
        player.update_discovered_spells(actor);
    }
}

fn sync_wands(mut query: Query<(&mut Actor, Option<&Player>), With<PlayerControlled>>) {
    let Some(original) = query
        .iter()
        .find(|(_, player)| player.is_some())
        .map(|(actor, _)| actor.wands.clone())
    else {
        return;
    };
    for (mut actor, player) in query.iter_mut() {
        if player.is_some() {
            continue;
        }
        actor.wands = original.clone();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                pick_gold,
                die_player,
                insert_discovered_spells,
                move_player,
                sync_wands,
            )
                .in_set(FixedUpdateInGameSet),
        );

        app.add_systems(
            FixedUpdate,
            (
                update_pointer_by_mouse,
                actor_cast,
                switch_wand,
                rotate_holded_bullets,
                release_holded_bullets,
            )
                .chain()
                .in_set(FixedUpdatePlayerActiveSet),
        );
    }
}
