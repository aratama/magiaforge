use crate::constant::*;
use crate::language::*;
use bevy::prelude::*;
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

pub struct GameConfigPlugin;

impl bevy::app::Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default());
    }
}
