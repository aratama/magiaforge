use bevy::prelude::*;
use bevy_websocket_sync::{ClientMessage, ServerMessage};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::{config::GameConfig, states::GameState};

/// オンライン対戦でリモート操作されているキャラクターを表します
#[derive(Component)]
pub struct RemotePlayer {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoteMessage {
    Ping { uuid: Uuid, x: f32, y: f32 },
}

fn update(reader: EventReader<ServerMessage>) {}

fn on_enter(config: Res<GameConfig>, mut writer: EventWriter<ClientMessage>) {
    if config.online {
        let url = dotenv!("url");
        writer.send(ClientMessage::Open(url.to_string()));
    }
}

fn on_exit(config: Res<GameConfig>, mut writer: EventWriter<ClientMessage>) {
    if config.online {
        writer.send(ClientMessage::Close);
    }
}

pub struct RemotePlayerPlugin;

impl Plugin for RemotePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update);

        app.add_systems(OnEnter(GameState::InGame), on_enter);

        app.add_systems(OnExit(GameState::InGame), on_exit);
    }
}
