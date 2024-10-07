use bevy::audio::PlaybackMode;
use bevy::prelude::*;

pub fn play_se(commands: &mut Commands, asset_server: &Res<AssetServer>, source_file: &str) {
    commands.spawn((
        Name::new(format!("se:{}", source_file)),
        AudioBundle {
            source: asset_server.load(source_file.to_string()),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
            ..default()
        },
    ));
}
