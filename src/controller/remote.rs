use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::constant::ARENA;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::bullet::spawn_bullet;
use crate::entity::bullet::SpawnBullet;
use crate::entity::gold::spawn_gold;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::page::in_game::setup_level;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::CRY;
use crate::set::FixedUpdateInGameSet;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::Velocity;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::ServerMessage;
use bevy_simple_websocket::WebSocketState;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use uuid::Uuid;

/// ネットワークに接続したクライアントは、常に互いの位置を送信しあっているため、
/// プレイヤーキャラクターがどこのレベルにいるのかに関わらず、常にその位置をお互いに把握しています。
/// また、実際に画面上にスポーンはしないものの、モンスター等の情報も定期的に把握しています。
///
/// プレイヤーキャラクターが新たなレベルに到達したとき、
/// そのレベルに別のプレイヤーがいる場合は、現在までに受信しているそのレベルのモンスターを自分のワールドにスポーンします。
/// そのレベルに別のプレイヤーがいない場合は、現在受信しているそのレベルのモンスターは無視し、
/// 新たにレベルとモンスターを生成してプレイを開始します。
/// なおこのとき、同じレベルに同時にプレイヤーが到達した場合、
/// 双方が同時にモンスターをスポーンするため、通常の2倍のモンスターが生成されることがあります。
/// この場合、優先権の高い側のプレイヤーは低い側の通知を無視するため、問題ありません。
/// 優先権の低い側のプレイヤーには一時的に2倍のモンスターが生成されますが、
/// ホスト権がないためこの余計なモンスターの情報が他者に通知されることはなく、
/// タイムアウト後に余計なモンスターは削除されます。
///
/// そのレベルの「ホスト」はそのレベルにいる最もUUIDの大きいプレイヤーです。
/// ホストはモンスターの動きを判定し、他のプレイヤーに通知します。
/// 自分よりuUIDの小さいユーザーから通知が来た場合、その通知は無視されます。
///
#[derive(Component)]
pub struct RemotePlayer {
    pub name: String,
    pub golds: u32,
    pub last_update: FrameCount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoteMessage {
    // エンティティの現在位置を通知します
    // 前回の通知と比較して、位置が変更されたか60フレーム以上経過した場合、
    // 他のプレイヤーから Join が送られたときは再通知します
    Position {
        sender: Uuid,
        uuid: Uuid,
        name: String,
        golds: u32,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        life: u32,
        max_life: u32,
        angle: f32,
        point_light_radius: f32,
    },
    // 弾を発射したことを通知します
    Fire(SpawnBullet),

    ServantSeed {
        from: Vec2,
        to: Vec2,
        actor_group: ActorGroup,
        servant_type: ActorType,
    },

    // ダメージを受けたことを通知します
    Hit {
        sender: Uuid,
        uuid: Uuid,
        damage: i32,
    },
    Die {
        sender: Uuid,
        uuid: Uuid,
    },
}

fn send_player_states(
    mut writer: EventWriter<ClientMessage>,
    mut query: Query<(&mut Player, &Actor, &GlobalTransform, &Velocity)>,
    state: Res<WebSocketState>,
    frame_count: Res<FrameCount>,
    current: Res<LevelSetup>,
) {
    if current.level == Some(GameLevel::new(ARENA)) && state.ready_state == ReadyState::OPEN {
        if let Ok((mut player, actor, transform, velocity)) = query.get_single_mut() {
            if actor.life <= 0 {
                return;
            }

            let translate = transform.translation();

            if 60 < (frame_count.0 as i32 - player.last_idle_frame_count.0 as i32)
                || translate.x != player.last_ilde_x
                || translate.y != player.last_ilde_y
                || actor.life != player.last_idle_life
                || actor.max_life != player.last_idle_max_life
            {
                let command = RemoteMessage::Position {
                    sender: actor.uuid,
                    uuid: actor.uuid,
                    name: player.name.clone(),
                    golds: actor.golds,
                    x: translate.x,
                    y: translate.y,
                    vx: velocity.linvel.x,
                    vy: velocity.linvel.y,
                    life: actor.life,
                    max_life: actor.max_life,
                    angle: actor.pointer.to_angle(),
                    point_light_radius: actor.point_light_radius,
                };
                let serialized = bincode::serialize(&command).unwrap();
                writer.send(ClientMessage::Binary(serialized));
                player.last_idle_frame_count = frame_count.clone();
                player.last_ilde_x = translate.x;
                player.last_ilde_y = translate.y;
                player.last_idle_vx = velocity.linvel.x;
                player.last_idle_vy = velocity.linvel.y;
            }
        }
    }
}

fn on_enter(mut writer: EventWriter<ClientMessage>, current: Res<LevelSetup>) {
    if current.level != Some(GameLevel::new(ARENA)) && current.next_level == GameLevel::new(ARENA) {
        info!("Connecting to {}", WEBSOCKET_URL);
        writer.send(ClientMessage::Open(WEBSOCKET_URL.to_string()));
    }
}

fn on_exit(mut writer: EventWriter<ClientMessage>, current: Res<LevelSetup>) {
    if current.level == Some(GameLevel::new(ARENA)) && current.next_level != GameLevel::new(ARENA) {
        info!("Closing {}", WEBSOCKET_URL);
        writer.send(ClientMessage::Close);
    }
}

#[allow(dead_code)]
enum RemoteEntityContent {
    Witch,
    Slime,
    Chest,
    BookShelf,
}

#[allow(dead_code)]
struct RemoteEntity {
    last_update: FrameCount,
    content: RemoteEntityContent,
    position: Vec2,
    level: i32,
}

#[allow(dead_code)]
struct RemoteStates {
    entities: HashMap<Uuid, RemoteEntity>,
}

fn receive_events(
    mut commands: Commands,
    registry: Registry,
    mut reader: EventReader<ServerMessage>,
    mut remotes: Query<
        (
            Entity,
            &mut RemotePlayer,
            &mut Actor,
            &mut Transform,
            &mut Velocity,
        ),
        With<RemotePlayer>,
    >,
    frame_count: Res<FrameCount>,
    mut writer: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEvent>,
) {
    // キャラクターを生成されたときに実際に反映させるのは次のフレームからですが、
    // 1フレームに複数のメッセージが届くことがあるため、
    // 1フレームに複数のキャラクターが生成されないようにセットで管理します
    let spawned_players: HashSet<Uuid> = HashSet::new();

    for message in reader.read() {
        match message {
            ServerMessage::String(text) => {
                info!("Received text message: {}", text);
            }
            ServerMessage::Binary(bin) => match bincode::deserialize::<RemoteMessage>(bin) {
                Err(err) => {
                    warn!("Failed to deserialize: {:?}", err);
                }
                Ok(command) => {
                    match command {
                        RemoteMessage::Position {
                            sender: _sender,
                            uuid,
                            name: _name,
                            golds,
                            x,
                            y,
                            vx,
                            vy,
                            life,
                            max_life,
                            angle,
                            point_light_radius: intensity,
                        } => {
                            let target = remotes
                                .iter_mut()
                                .find(|(_, _, actor, _, _)| actor.uuid == uuid);
                            if let Some((_, mut remote, mut actor, mut transform, mut velocity)) =
                                target
                            {
                                remote.last_update = *frame_count;
                                remote.golds = golds;
                                transform.translation.x = x;
                                transform.translation.y = y;
                                velocity.linvel.x = vx;
                                velocity.linvel.y = vy;
                                actor.life = life;
                                actor.max_life = max_life;
                                actor.pointer = Vec2::from_angle(angle);
                                actor.point_light_radius = intensity;
                            } else if !spawned_players.contains(&uuid) {
                                // spawned_players.insert(uuid);
                                // let entity = spawn_basic_actor(
                                //     &mut commands,
                                //     &registry,
                                //     Vec2::new(x, y),
                                //     Some(name.clone()),
                                //     get_default_actor(&registry, ActorType::Witch),
                                //     false,
                                // );
                                // commands.entity(entity).insert(RemotePlayer {
                                //     name,
                                //     golds,
                                //     last_update: *frame_count,
                                // });
                                // info!("Remote player spawned: {}", uuid);
                                unimplemented!();
                            }
                        }
                        RemoteMessage::Fire(spawning) => {
                            spawn_bullet(&mut commands, &registry, &mut writer, &spawning);
                        }
                        RemoteMessage::Hit {
                            sender: _sender,
                            uuid,
                            damage,
                        } => {
                            let target = remotes
                                .iter_mut()
                                .find(|(_, _, actor, _, _)| actor.uuid == uuid);

                            if let Some((_, mut remote, mut actor_life, _, _)) = target {
                                actor_life.life = (actor_life.life as i32 - damage).max(0) as u32;
                                remote.last_update = *frame_count;
                            }
                        }
                        RemoteMessage::Die {
                            sender: _sender,
                            uuid,
                        } => {
                            let target = remotes
                                .iter_mut()
                                .find(|(_, _, actor, _, _)| actor.uuid == uuid);

                            if let Some((entity, _, _, transform, _)) = target {
                                let position = transform.translation.truncate();
                                writer.send(SEEvent::pos(CRY, position));
                                commands.entity(entity).despawn_recursive();

                                let player_defeat_bonus = 100;
                                for _ in 0..player_defeat_bonus {
                                    spawn_gold(&mut commands, &registry, position);
                                }
                            }
                        }
                        RemoteMessage::ServantSeed {
                            from,
                            to,
                            actor_group,
                            servant_type,
                        } => {
                            spawn.send(SpawnEvent {
                                position: from,
                                spawn: Spawn::Seed {
                                    to,
                                    actor_group,
                                    owner: None,
                                    servant_type,
                                    remote: false,
                                    servant: false,
                                },
                            });
                        }
                    };
                }
            },
            _ => {}
        }
    }
}

/// 最終の Ping から120フレーム以上経過したリモートプレイヤーを削除します
fn despawn_no_contact_remotes(
    mut commands: Commands,
    mut remotes: Query<(Entity, &Actor, &RemotePlayer)>,
    frame_count: Res<FrameCount>,
) {
    for (entity, actor, remote) in remotes.iter_mut() {
        if 120 < (frame_count.0 as i32 - remote.last_update.0 as i32) {
            info!("Remote player {} despawned", actor.uuid);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn send_remote_message(
    writer: &mut EventWriter<ClientMessage>,
    online: bool,
    message: &RemoteMessage,
) {
    if online {
        let serialized = bincode::serialize::<RemoteMessage>(message).unwrap();
        writer.send(ClientMessage::Binary(serialized));
    }
}

pub struct RemotePlayerPlugin;

impl Plugin for RemotePlayerPlugin {
    fn build(&self, app: &mut App) {
        // setup_level も OnEnter(GameState::InGame) に登録されていますが、
        // setup_level が完了すると current.level が更新されるため、
        // on_enter の条件分岐が正しく動かず、オンラインになりません
        // on_enter を先にやります
        app.add_systems(OnEnter(GameState::InGame), on_enter.before(setup_level));

        app.add_systems(OnExit(GameState::InGame), on_exit);

        app.add_systems(
            FixedUpdate,
            (
                send_player_states,
                receive_events,
                despawn_no_contact_remotes,
            )
                .in_set(FixedUpdateInGameSet),
        );
    }
}
