use bevy::{core::FrameCount, prelude::*};
use bevy_rapier2d::prelude::Velocity;
use bevy_simple_websocket::{ClientMessage, ReadyState, ServerMessage, WebSocketState};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    asset,
    config::GameConfig,
    entity::{
        actor::Actor,
        bullet::add_bullet,
        witch::{spawn_witch, WitchType},
    },
    states::GameState,
};

use super::player::Player;

/// オンライン対戦でリモート操作されているキャラクターを表します
#[derive(Component)]
pub struct RemotePlayer {
    pub last_update: FrameCount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoteMessage {
    Position {
        uuid: Uuid,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
    },
    Fire {
        uuid: Uuid,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
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
                {
                    let command = RemoteMessage::Position {
                        uuid: actor.uuid,
                        x: translate.x,
                        y: translate.y,
                        vx: velocity.linvel.x,
                        vy: velocity.linvel.y,
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
        (&mut RemotePlayer, &Actor, &mut Transform, &mut Velocity),
        With<RemotePlayer>,
    >,
    assets: Res<asset::GameAssets>,
    frame_count: Res<FrameCount>,
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
                    RemoteMessage::Position { uuid, x, y, vx, vy } => {
                        let target = remotes
                            .iter_mut()
                            .find(|(_, actor, _, _)| actor.uuid == uuid);
                        if let Some((mut remote, _, mut transform, mut velocity)) = target {
                            remote.last_update = *frame_count;
                            transform.translation.x = x;
                            transform.translation.y = y;
                            velocity.linvel.x = vx;
                            velocity.linvel.y = vy;
                        } else {
                            spawn_witch(
                                &mut commands,
                                &assets,
                                x,
                                y,
                                uuid,
                                WitchType::RemoteWitch,
                                *frame_count,
                            );
                            info!("Remote player {} spawned", uuid);
                        }
                    }
                    RemoteMessage::Fire { uuid, x, y, vx, vy } => {
                        add_bullet(
                            &mut commands,
                            assets.asset.clone(),
                            Vec2::new(x, y),
                            Vec2::new(vx, vy),
                            Some(uuid),
                        );
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
            commands.entity(entity).despawn();
        }
    }
}

pub struct RemotePlayerPlugin;

impl Plugin for RemotePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, send_player_states);

        app.add_systems(OnEnter(GameState::InGame), on_enter);

        app.add_systems(OnExit(GameState::InGame), on_exit);

        app.add_systems(
            FixedUpdate,
            receive_events.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            FixedUpdate,
            despown_no_contact_remotes.run_if(in_state(GameState::InGame)),
        );
    }
}
