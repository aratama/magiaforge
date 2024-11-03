use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub online: bool,
    pub bgm_volume: f32,
    pub se_volume: f32,
    pub player_name: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            online: true,
            bgm_volume: 0.0,
            se_volume: 0.8,
            player_name: "".to_string(),
        }
    }
}

#[allow(dead_code)]
fn startup(pkv: Res<PkvStore>, mut config: ResMut<GameConfig>) {
    if let Ok(v) = pkv.get::<String>("config") {
        if let Ok(deserialized) = serde_json::from_str(v.as_str()) {
            *config = deserialized;
        }
    };
}

#[allow(dead_code)]
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

pub struct GameConfigPlugin;

impl bevy::app::Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default());
        #[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
        app.add_systems(Startup, startup);
        #[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
        app.add_systems(Update, on_change);
    }
}
