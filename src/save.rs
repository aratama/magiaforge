use crate::actor::Actor;
use crate::config::GameConfig;
use crate::constant::CRATE_NAME;
use crate::controller::player::Player;
use crate::page::in_game::setup_level;
use crate::page::in_game::LevelSetup;
use crate::player_state::PlayerState;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

fn load_config(pkv: Res<PkvStore>, mut config: ResMut<GameConfig>) {
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

fn save_config_on_change(mut pkv: ResMut<PkvStore>, config: Res<GameConfig>) {
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

fn load_player(pkv: Res<PkvStore>, mut interlevel: ResMut<LevelSetup>) {
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

fn save_player(
    mut pkv: ResMut<PkvStore>,
    frame_count: Res<FrameCount>,
    player_query: Query<(&Player, &Actor)>,
) {
    if frame_count.0 % 60 == 0 {
        if let Ok((player, actor)) = player_query.get_single() {
            let player_state = PlayerState::new(&player, &actor);
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
        app.add_systems(Startup, load_config);
        app.add_systems(Update, save_config_on_change);
        app.add_systems(
            OnEnter(GameState::MainMenu),
            load_player.before(setup_level),
        );
        app.add_systems(Update, save_player.run_if(in_state(GameState::InGame)));
    }
}
