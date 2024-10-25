use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameConfig {
    pub online: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { online: false }
    }
}

pub struct GameConfigPlugin;

impl bevy::app::Plugin for GameConfigPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GameConfig::default());
    }
}
