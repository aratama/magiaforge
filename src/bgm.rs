use std::time::Duration;

use super::asset::GameAssets;
use super::states::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{
    prelude::Volume, Audio, AudioControl, AudioEasing, AudioInstance, AudioSource, AudioTween,
};
use bevy_rapier2d::plugin::PhysicsSet;

#[cfg(not(feature = "debug"))]
const BGM_VOLUME: f64 = 0.2;

#[cfg(feature = "debug")]
const BGM_VOLUME: f64 = 0.0;

/// 次に再生するBGMを表すリソース
#[derive(Resource, Default)]
pub struct BGM(pub Option<Handle<AudioSource>>);

struct SourceAndInstance {
    instance: Handle<AudioInstance>,
    source: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct CurrentBGM(Option<SourceAndInstance>);

fn update_bgm(
    mut current_bgm: ResMut<CurrentBGM>,
    next_bgm: ResMut<BGM>,
    audio: Res<Audio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let BGM(ref next_bgm_or_none) = *next_bgm;

    if let Some(ref mut current_handle) = &mut current_bgm.0 {
        if let Some(ref next) = *next_bgm_or_none {
            if current_handle.source.id() != next.id() {
                if let Some(instance) = audio_instances.get_mut(&current_handle.instance) {
                    instance.stop(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
                }
                let instance = audio
                    .play(next.clone())
                    .with_volume(Volume::Amplitude(BGM_VOLUME))
                    .looped()
                    .handle();
                current_bgm.0 = Some(SourceAndInstance {
                    instance: instance.clone(),
                    source: next.clone(),
                });
            }
        }
    } else if let Some(ref next) = *next_bgm_or_none {
        let instance = audio
            .play(next.clone())
            .with_volume(Volume::Amplitude(BGM_VOLUME))
            .looped()
            .handle();
        current_bgm.0 = Some(SourceAndInstance {
            instance: instance.clone(),
            source: next.clone(),
        });
    }
}

pub struct BGMPlugin;

impl Plugin for BGMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_bgm.before(PhysicsSet::SyncBackend));
        app.init_resource::<BGM>();
        app.init_resource::<CurrentBGM>();

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
