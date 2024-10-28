use bevy::prelude::Resource;

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

pub struct GameConfigPlugin;

impl bevy::app::Plugin for GameConfigPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GameConfig::default());
    }
}
