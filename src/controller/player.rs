use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::component::counter::CounterAnimated;
use crate::component::life::Life;
use crate::constant::ENTITY_LAYER_Z;
use crate::constant::MAX_WANDS;
use crate::controller::remote::send_remote_message;
use crate::controller::remote::RemoteMessage;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorState;
use crate::entity::bullet::Bullet;
use crate::entity::bullet::Trigger;
use crate::entity::gold::Gold;
use crate::input::get_direction;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::player_state::PlayerState;
use crate::se::SEEvent;
use crate::se::SE;
use crate::spell::SpellType;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
use crate::wand::WandSpell;
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
    pub fn new(name: String, getting_up: bool) -> Self {
        Self {
            name,
            last_idle_frame_count: FrameCount(0),
            last_ilde_x: 0.0,
            last_ilde_y: 0.0,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: 0,
            last_idle_max_life: 0,
            getting_up: if getting_up { 240 } else { 0 },
            discovered_spells: HashSet::new(),
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

/// 現在プレイヤーが操作対象としている召喚キャラクターです
/// これを付与したアクターは、そのエンティティ本来の操作と、プレイヤーの操作が、衝突しないように注意します
#[derive(Component, Debug, Clone)]
pub struct PlayerServant;

#[derive(Component, Debug, Clone)]
pub struct PlayerDown;

/// プレイヤーの移動
/// ここではまだ ExternalForce へはアクセスしません
/// Actor側で ExternalForce にアクセスして、移動を行います
fn move_player(
    mut player_query: Query<&mut Actor, With<Player>>,
    mut servant_query: Query<&mut Actor, (With<PlayerServant>, Without<Player>)>,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<State<GameMenuState>>,
) {
    if let Ok(mut actor) = player_query.get_single_mut() {
        match *menu.get() {
            GameMenuState::Closed => {
                let direction = get_direction(keys);
                let state = if direction != Vec2::ZERO {
                    ActorState::Run
                } else {
                    ActorState::Idle
                };
                if let Some(mut servant) = servant_query.iter_mut().next() {
                    actor.move_direction = Vec2::ZERO;
                    actor.state = ActorState::Idle;
                    servant.move_direction = direction;
                    servant.move_force = 50000.0;
                    servant.state = state;
                } else {
                    actor.move_direction = direction;
                    actor.state = state;
                }
            }
            GameMenuState::PlayerInActive => {
                // 起き上がりアニメーションのときは ActorState::GettingUp になっているはず
                actor.move_direction = Vec2::ZERO;
                actor.state = ActorState::GettingUp;
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

        for wand in actor.wands.iter() {
            for slot in wand.slots {
                match slot {
                    Some(WandSpell {
                        spell_type: SpellType::Lantern,
                        ..
                    }) => {
                        point_light_radius += 160.0;
                    }
                    _ => {}
                }
            }
        }

        actor.point_light_radius = point_light_radius;
    }
}

/// 魔法の発射
pub fn actor_cast(
    mut commands: Commands,
    mut player_query: Query<&mut Actor, (With<Player>, Without<Camera2d>)>,
    servant_query: Query<Entity, (With<PlayerServant>, Without<Player>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    menu: Res<State<GameMenuState>>,
    camera_query: Query<&GameCamera>,
    mut se: EventWriter<SEEvent>,
) {
    let camera = camera_query.single();
    if camera.target.is_some() {
        return;
    }

    if let Ok(mut actor) = player_query.get_single_mut() {
        match *menu.get() {
            GameMenuState::Closed => {
                // プライマリ魔法の発射
                actor.fire_state = if buttons.pressed(MouseButton::Left) {
                    ActorFireState::Fire
                } else {
                    ActorFireState::Idle
                };

                // 杖が空の場合の失敗音
                if actor.wands[actor.current_wand].is_empty()
                    && buttons.just_pressed(MouseButton::Left)
                {
                    se.send(SEEvent::new(SE::Sen));
                }

                if let Some(servant) = servant_query.iter().next() {
                    // 憑依の解除
                    if buttons.just_pressed(MouseButton::Right) {
                        commands.entity(servant).remove::<PlayerServant>();
                    }
                } else {
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
                        se.send(SEEvent::new(SE::Sen));
                    }
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
    mut next: ResMut<LevelSetup>,
) {
    if let Ok((entity, actor, player_life, transform, player)) = player_query.get_single() {
        if player_life.life <= 0 {
            commands.entity(entity).despawn_recursive();

            game.send(SEEvent::pos(SE::Cry, transform.translation.truncate()));

            // ダウン後はキャンプに戻る
            next.next_level = GameLevel::Level(0);

            // 次のシーンのためにプレイヤーの状態を保存
            let mut player_state = PlayerState::from_player(&player, &actor, &player_life);
            // 全回復させてから戻る
            player_state.life = player_state.max_life;
            next.next_state = Some(player_state);

            // 倒れるアニメーションを残す
            commands.spawn((
                PlayerDown,
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

fn getting_up(
    mut player_query: Query<(&mut Actor, &mut Player)>,
    mut next: ResMut<NextState<GameMenuState>>,
) {
    for (mut actor, mut player) in player_query.iter_mut() {
        if 0 < player.getting_up {
            player.getting_up -= 1;
            if player.getting_up == 0 {
                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;
                next.set(GameMenuState::Closed);
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
            (update_pointer_by_mouse,)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );

        app.add_systems(
            // FixedUpdateでスケジュールされたシステムには、before(PhysicsSet::SyncBackend) でスケジュールをする必要があります
            // これがない場合、変更が正しく rapier に通知されず、数回に一度のような再現性の低いバグが起きることがあるようです
            // https://taintedcoders.com/bevy/physics/rapier
            FixedUpdate,
            (
                getting_up,
                pick_gold,
                die_player,
                apply_intensity_by_lantern,
                insert_discovered_spells,
                move_player,
            )
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend),
        );

        app.add_systems(
            FixedUpdate,
            (
                actor_cast,
                switch_wand,
                rotate_holded_bullets,
                release_holded_bullets,
            )
                .chain()
                .run_if(
                    in_state(GameState::InGame)
                        .and(in_state(TimeState::Active))
                        .and(in_state(GameMenuState::Closed)),
                )
                .before(PhysicsSet::SyncBackend),
        );
    }
}
