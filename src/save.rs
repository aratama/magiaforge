use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::life::Life;
use crate::page::in_game::Interlevel;
use crate::player_state::PlayerState;
use crate::states::GameState;
use crate::{config::GameConfig, constant::CRATE_NAME};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

fn startup(pkv: Res<PkvStore>, mut config: ResMut<GameConfig>) {
    if let Ok(v) = pkv.get::<String>("config") {
        if let Ok(deserialized) = serde_json::from_str(v.as_str()) {
            *config = deserialized;
        } else {
            warn!("Failed to deserialize config");
        }
    } else {
        warn!("key `config` not found");
    }
}

fn on_change(mut pkv: ResMut<PkvStore>, config: Res<GameConfig>) {
    if config.is_changed() {
        if let Ok(serialized) = serde_json::to_string(&config.into_inner()) {
            if let Err(err) = pkv.set::<String>("config", &serialized) {
                warn!("Failed to save config: {}", err);
            }
        } else {
            warn!("Failed to serialize config");
        }
    }
}

fn load(pkv: Res<PkvStore>, mut interlevel: ResMut<Interlevel>) {
    if let Ok(v) = pkv.get::<String>("state") {
        if let Ok(deserialized) = serde_json::from_str(v.as_str()) {
            interlevel.next_state = deserialized;
            info!("State loaded");
        } else {
            warn!("Failed to deserialize state");
        }
    } else {
        warn!("key `state` not found");
    }
}

fn save(
    mut pkv: ResMut<PkvStore>,
    frame_count: Res<FrameCount>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    if frame_count.0 % 60 == 0 {
        if let Ok((player, actor, life)) = player_query.get_single() {
            let player_state = PlayerState::new(&player, &actor, &life);
            if let Ok(serialized) = serde_json::to_string(&player_state) {
                if let Err(err) = pkv.set::<String>("state", &serialized) {
                    warn!("Failed to save state: {}", err);
                }
            } else {
                warn!("Failed to serialize state");
            }
        }
    }
}

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PkvStore::new(CRATE_NAME, CRATE_NAME));
        app.add_systems(Startup, startup);
        app.add_systems(Update, on_change);
        app.add_systems(OnEnter(GameState::MainMenu), load);
        app.add_systems(Update, save.run_if(in_state(GameState::InGame)));
    }
}
