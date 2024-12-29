use crate::constant::*;
use crate::language::*;
use bevy::prelude::*;
#[cfg(feature = "save")]
use bevy_pkv::PkvStore;
use serde::Deserialize;
use serde::Serialize;

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub bgm_volume: f32,
    pub se_volume: f32,
    pub player_name: String,
    pub language: Languages,
    pub fullscreen: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            bgm_volume: DEFAULT_BGM_VOLUME,
            se_volume: DEFAULT_SE_VOLUME,
            player_name: "".to_string(),
            language: Languages::Ja,
            fullscreen: false,
        }
    }
}

#[cfg(feature = "save")]
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

#[cfg(feature = "save")]
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

        #[cfg(feature = "save")]
        app.add_systems(Startup, startup);

        #[cfg(feature = "save")]
        app.add_systems(Update, on_change);
    }
}
