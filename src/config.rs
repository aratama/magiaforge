use bevy::prelude::*;
use bevy_pkv::PkvStore;

#[derive(Resource, Clone, Debug)]
pub struct GameConfig {
    pub online: bool,
    pub bgm_volume: f32,
    pub se_volume: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            online: true,
            bgm_volume: 0.5,
            se_volume: 0.5,
        }
    }
}

#[allow(dead_code)]
fn startup(pkv: Res<PkvStore>, mut config: ResMut<GameConfig>) {
    if let Ok(v) = pkv.get::<f32>("bgm_volume") {
        config.bgm_volume = v;
    }
    if let Ok(v) = pkv.get::<f32>("sfx_volume") {
        config.se_volume = v;
    };
}

#[allow(dead_code)]
fn on_change(mut pkv: ResMut<PkvStore>, config: Res<GameConfig>) {
    if config.is_changed() {
        if let Err(err) = pkv.set::<f32>("bgm_volume", &config.bgm_volume) {
            warn!("Failed to save bgm volume: {}", err);
        }
        if let Err(err) = pkv.set::<f32>("sfx_volume", &config.se_volume) {
            warn!("Failed to save bgm volume: {}", err);
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
