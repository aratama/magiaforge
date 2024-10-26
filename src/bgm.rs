use super::asset::GameAssets;
use super::states::GameState;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

#[cfg(not(feature = "debug"))]
const BGM_VOLUME: f32 = 0.2;

#[cfg(feature = "debug")]
const BGM_VOLUME: f32 = 0.0;

#[derive(Component)]
struct BGMAudioBundle;

fn update_bgm(
    mut commands: Commands,
    bgm_query: Query<(Entity, &AudioSink, &Handle<AudioSource>), With<BGMAudioBundle>>,
    bgm_resource: ResMut<BGM>,
) {
    let BGM(ref bgm_or_none) = *bgm_resource;

    if let Ok((audio, sink, source)) = bgm_query.get_single() {
        let volume = (sink.volume() - 0.002).max(0.0);

        if let Some(ref bgm_audio_source) = *bgm_or_none {
            if bgm_audio_source != source {
                if volume <= 0.0 {
                    commands.entity(audio).despawn();
                    commands.spawn((
                        Name::new("bgm"),
                        BGMAudioBundle,
                        AudioBundle {
                            source: bgm_audio_source.clone(),
                            settings: PlaybackSettings {
                                volume: Volume::new(BGM_VOLUME),
                                mode: PlaybackMode::Loop,
                                ..default()
                            },
                        },
                    ));
                } else {
                    sink.set_volume(volume);
                }
            }
        } else {
            sink.set_volume(volume);
        }
    } else if let Some(ref next) = *bgm_or_none {
        commands.spawn((
            BGMAudioBundle,
            AudioBundle {
                source: next.clone(),
                settings: PlaybackSettings {
                    volume: Volume::new(BGM_VOLUME),
                    mode: PlaybackMode::Loop,
                    ..default()
                },
            },
        ));
    }
}

#[derive(Resource, Default)]
pub struct BGM(pub Option<Handle<AudioSource>>);

pub struct BGMPlugin;

impl Plugin for BGMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_bgm);
        app.init_resource::<BGM>();

        app.add_systems(
            OnEnter(GameState::InGame),
            |mut next: ResMut<BGM>, assets: Res<GameAssets>| {
                *next = BGM(Some(assets.they.clone()));
            },
        );

        app.add_systems(
            OnEnter(GameState::MainMenu),
            |mut next: ResMut<BGM>, assets: Res<GameAssets>| {
                *next = BGM(Some(assets.gods_realm.clone()));
            },
        );
    }
}
