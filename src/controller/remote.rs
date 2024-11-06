use std::collections::HashSet;

use super::player::Player;
use crate::{
    asset::GameAssets,
    command::GameCommand,
    config::GameConfig,
    constant::{ENEMY_GROUP, ENTITY_GROUP, WALL_GROUP, WITCH_BULLET_GROUP, WITCH_GROUP},
    entity::{actor::Actor, bullet::spawn_bullet, gold::spawn_gold, witch::spawn_witch},
    hud::life_bar::LifeBarResource,
    states::GameState,
    world::CurrentLevel,
};
use bevy::{core::FrameCount, prelude::*};
use bevy_kira_audio::Audio;
use bevy_rapier2d::{plugin::PhysicsSet, prelude::Velocity};
use bevy_simple_websocket::{ClientMessage, ReadyState, ServerMessage, WebSocketState};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// オンライン対戦でリモート操作されているキャラクターを表します
#[derive(Component)]
pub struct RemotePlayer {
    pub name: String,
    pub golds: i32,
    pub last_update: FrameCount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoteMessage {
    /// このプレイヤーが参加を開始したことを通知し、他のプレイヤーの自己紹介を促します
    Join,
    // 現在位置を通知します
    // 前回の通知と比較して、位置が変更されたか60フレーム以上経過した場合、
    // 他のプレイヤーから Join が送られたときは再通知します
    Position {
        uuid: Uuid,
        name: String,
        golds: i32,
        level: i32,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        life: i32,
        max_life: i32,
        angle: f32,
        intensity: f32,
    },
    // 弾を発射したことを通知します
    Fire {
        uuid: Uuid,
        level: i32,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        bullet_lifetime: u32,
    },
    // ダメージを受けたことを通知します
    Hit {
        uuid: Uuid,
        damage: i32,
    },
    Die {
        uuid: Uuid,
    },
}

fn send_player_states(
    mut writer: EventWriter<ClientMessage>,
    mut query: Query<(&mut Player, &Actor, &GlobalTransform, &Velocity)>,
    config: Res<GameConfig>,
    state: Res<WebSocketState>,
    frame_count: Res<FrameCount>,
    current: Res<CurrentLevel>,
) {
    if config.online {
        if let Some(level) = current.0 {
            if let Ok((mut player, actor, transform, velocity)) = query.get_single_mut() {
                if actor.life <= 0 {
                    return;
                }

                if state.ready_state == ReadyState::OPEN {
                    let translate = transform.translation();

                    if 60 < (frame_count.0 as i32 - player.last_idle_frame_count.0 as i32)
                        || translate.x != player.last_ilde_x
                        || translate.y != player.last_ilde_y
                        || actor.life != player.last_idle_life
                        || actor.max_life != player.last_idle_max_life
                    {
                        let command = RemoteMessage::Position {
                            uuid: actor.uuid,
                            name: player.name.clone(),
                            golds: player.golds,
                            level,
                            x: translate.x,
                            y: translate.y,
                            vx: velocity.linvel.x,
                            vy: velocity.linvel.y,
                            life: actor.life,
                            max_life: actor.max_life,
                            angle: actor.pointer.to_angle(),
                            intensity: actor.intensity,
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
    }
}

fn on_enter(config: Res<GameConfig>, mut writer: EventWriter<ClientMessage>) {
    if config.online {
        let url = dotenv!("url");
        info!("Connecting to {}", url);
        writer.send(ClientMessage::Open(url.to_string()));
    }
}

fn on_exit(config: Res<GameConfig>, mut writer: EventWriter<ClientMessage>) {
    if config.online {
        writer.send(ClientMessage::Close);
    }
}

fn receive_events(
    mut commands: Commands,
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
    assets: Res<GameAssets>,
    frame_count: Res<FrameCount>,
    life_bar_res: Res<LifeBarResource>,
    current: Res<CurrentLevel>,
    mut writer: EventWriter<GameCommand>,
    audio: Res<Audio>,
) {
    // キャラクターを生成されたときに実際に反映させるのは次のフレームからですが、
    // 1フレームに複数のメッセージが届くことがあるため、
    // 1フレームに複数のキャラクターが生成されないようにセットで管理します
    let mut spawned_players = HashSet::new();

    for message in reader.read() {
        match message {
            ServerMessage::String(text) => {
                info!("Received text message: {}", text);
            }
            ServerMessage::Binary(bin) => {
                let command: RemoteMessage =
                    bincode::deserialize(bin).expect("Failed to deserialize");
                match command {
                    RemoteMessage::Join => {}
                    RemoteMessage::Position {
                        uuid,
                        name,
                        golds,
                        level,
                        x,
                        y,
                        vx,
                        vy,
                        life,
                        max_life,
                        angle,
                        intensity,
                    } => {
                        if let Some(current_level) = current.0 {
                            if current_level == level {
                                let target = remotes
                                    .iter_mut()
                                    .find(|(_, _, actor, _, _)| actor.uuid == uuid);
                                if let Some((
                                    _,
                                    mut remote,
                                    mut actor,
                                    mut transform,
                                    mut velocity,
                                )) = target
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
                                    actor.intensity = intensity;
                                } else if !spawned_players.contains(&uuid) {
                                    spawned_players.insert(uuid);
                                    spawn_witch(
                                        &mut commands,
                                        &assets,
                                        Vec2::new(x, y),
                                        angle,
                                        uuid,
                                        Some(name.clone()),
                                        life,
                                        max_life,
                                        &life_bar_res,
                                        RemotePlayer {
                                            name,
                                            golds,
                                            last_update: *frame_count,
                                        },
                                        true,
                                        3.0,
                                        &audio,
                                    );
                                    info!("Remote player spawned: {}", uuid);
                                }
                            }
                        }
                    }
                    RemoteMessage::Fire {
                        uuid,
                        level,
                        x,
                        y,
                        vx,
                        vy,
                        bullet_lifetime,
                    } => {
                        if let Some(current_level) = current.0 {
                            if current_level == level {
                                spawn_bullet(
                                    &mut commands,
                                    assets.asset.clone(),
                                    Vec2::new(x, y),
                                    Vec2::new(vx, vy),
                                    bullet_lifetime,
                                    Some(uuid),
                                    &mut writer,
                                    WITCH_BULLET_GROUP,
                                    WALL_GROUP | ENTITY_GROUP | WITCH_GROUP | ENEMY_GROUP,
                                );
                            }
                        }
                    }
                    RemoteMessage::Hit { uuid, damage } => {
                        let target = remotes
                            .iter_mut()
                            .find(|(_, _, actor, _, _)| actor.uuid == uuid);

                        if let Some((_, mut remote, mut actor, _, _)) = target {
                            actor.life -= damage;
                            remote.last_update = *frame_count;
                        }
                    }
                    RemoteMessage::Die { uuid } => {
                        let target = remotes
                            .iter_mut()
                            .find(|(_, _, actor, _, _)| actor.uuid == uuid);

                        if let Some((entity, _, _, transform, _)) = target {
                            writer.send(GameCommand::SEHiyoko(Some(
                                transform.translation.truncate(),
                            )));

                            commands.entity(entity).despawn_recursive();

                            for _ in 0..20 {
                                spawn_gold(
                                    &mut commands,
                                    &assets,
                                    transform.translation.x,
                                    transform.translation.y,
                                );
                            }
                        }
                    }
                }
            }
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

pub struct RemotePlayerPlugin;

impl Plugin for RemotePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), on_enter);

        app.add_systems(OnExit(GameState::InGame), on_exit);

        app.add_systems(
            FixedUpdate,
            (
                send_player_states,
                receive_events,
                despawn_no_contact_remotes,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
