use bevy::audio::PlaybackMode;
use bevy::prelude::*;

pub fn play_se(commands: &mut Commands, source: Handle<AudioSource>) {
    commands.spawn((
        Name::new(format!(
            "se:{}",
            source.path().map_or("".to_string(), |p| p.to_string())
        )),
        AudioBundle {
            source,
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
            ..default()
        },
    ));
}
