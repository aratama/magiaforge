use bevy::{core::FrameCount, prelude::*};
use bevy_kira_audio::Audio;
use bevy_rapier2d::{plugin::PhysicsSet, prelude::Velocity};
use bevy_simple_websocket::{ClientMessage, ReadyState, ServerMessage, WebSocketState};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    asset,
    config::GameConfig,
    entity::{actor::Actor, bullet::spawn_bullet, witch::spawn_witch},
    hud::life_bar::LifeBarResource,
    states::GameState,
};

use super::player::Player;

/// オンライン対戦でリモート操作されているキャラクターを表します
#[derive(Component)]
pub struct RemotePlayer {
    pub name: String,
    pub last_update: FrameCount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoteMessage {
    // 現在位置を通知します
    // 前回の通知と比較して、位置が変更されたか60フレーム以上経過した場合は再通知します
    Position {
        uuid: Uuid,
        name: String,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        life: i32,
        max_life: i32,
        angle: f32,
    },
    // 弾を発射したことを通知します
    Fire {
        uuid: Uuid,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
    },
    // ダメージを受けたことを通知します
    Hit {
        uuid: Uuid,
        damage: i32,
    },
}

fn send_player_states(
    mut writer: EventWriter<ClientMessage>,
    mut query: Query<(&mut Player, &Actor, &GlobalTransform, &Velocity)>,
    config: Res<GameConfig>,
    state: Res<WebSocketState>,
    frame_count: Res<FrameCount>,
) {
    if config.online {
        if let Ok((mut player, actor, transform, velocity)) = query.get_single_mut() {
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
                        x: translate.x,
                        y: translate.y,
                        vx: velocity.linvel.x,
                        vy: velocity.linvel.y,
                        life: actor.life,
                        max_life: actor.max_life,
                        angle: actor.pointer.to_angle(),
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
        (&mut RemotePlayer, &mut Actor, &mut Transform, &mut Velocity),
        With<RemotePlayer>,
    >,
    assets: Res<asset::GameAssets>,
    frame_count: Res<FrameCount>,
    life_bar_res: Res<LifeBarResource>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for message in reader.read() {
        match message {
            ServerMessage::String(text) => {
                info!("Received text message: {}", text);
            }
            ServerMessage::Binary(bin) => {
                let command: RemoteMessage =
                    bincode::deserialize(bin).expect("Failed to deserialize");
                match command {
                    RemoteMessage::Position {
                        uuid,
                        name,
                        x,
                        y,
                        vx,
                        vy,
                        life,
                        max_life,
                        angle,
                    } => {
                        let target = remotes
                            .iter_mut()
                            .find(|(_, actor, _, _)| actor.uuid == uuid);
                        if let Some((mut remote, mut actor, mut transform, mut velocity)) = target {
                            remote.last_update = *frame_count;
                            transform.translation.x = x;
                            transform.translation.y = y;
                            velocity.linvel.x = vx;
                            velocity.linvel.y = vy;
                            actor.life = life;
                            actor.max_life = max_life;
                            actor.pointer = Vec2::from_angle(angle);
                        } else {
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
                                    last_update: *frame_count,
                                },
                            );
                            info!("Remote player {} spawned", uuid);
                        }
                    }
                    RemoteMessage::Fire { uuid, x, y, vx, vy } => {
                        spawn_bullet(
                            &mut commands,
                            assets.asset.clone(),
                            Vec2::new(x, y),
                            Vec2::new(vx, vy),
                            Some(uuid),
                            &assets,
                            &audio,
                            &config,
                        );
                    }
                    RemoteMessage::Hit { uuid, damage } => {
                        let target = remotes
                            .iter_mut()
                            .find(|(_, actor, _, _)| actor.uuid == uuid);

                        if let Some((mut remote, mut actor, _, _)) = target {
                            actor.life -= damage;
                            remote.last_update = *frame_count;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// 最終の Ping から120フレーム以上経過したリモートプレイヤーを削除します
fn despown_no_contact_remotes(
    mut commands: Commands,
    mut remotes: Query<(Entity, &Actor, &RemotePlayer)>,
    frame_count: Res<FrameCount>,
) {
    for (entity, actor, remote) in remotes.iter_mut() {
        if 120 < (frame_count.0 as i32 - remote.last_update.0 as i32) {
            info!("Remote player {} despowned", actor.uuid);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct RemotePlayerPlugin;

impl Plugin for RemotePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            send_player_states.before(PhysicsSet::SyncBackend),
        );

        app.add_systems(OnEnter(GameState::InGame), on_enter);

        app.add_systems(OnExit(GameState::InGame), on_exit);

        app.add_systems(
            FixedUpdate,
            receive_events
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );

        app.add_systems(
            FixedUpdate,
            despown_no_contact_remotes
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
