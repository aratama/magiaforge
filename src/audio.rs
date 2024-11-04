use crate::config::GameConfig;
use bevy::prelude::*;
use bevy_kira_audio::{
    prelude::Volume, Audio, AudioControl, AudioEasing, AudioInstance, AudioSource, AudioTween,
};
use bevy_rapier2d::plugin::PhysicsSet;
use std::time::Duration;

pub fn play_se(
    audio: &Res<Audio>,
    config: &GameConfig,
    source: Handle<AudioSource>,
) -> Handle<AudioInstance> {
    audio
        .play(source.clone())
        .with_volume(Volume::Amplitude(config.se_volume as f64))
        .handle()
}

/// 次に再生するBGMを表すリソース
#[derive(Resource, Default)]
pub struct BGM(pub Option<Handle<AudioSource>>);

struct SourceAndInstance {
    instance: Handle<AudioInstance>,
    source: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct CurrentBGM(Option<SourceAndInstance>);

fn change_bgm(
    mut current_bgm: ResMut<CurrentBGM>,
    next_bgm: ResMut<BGM>,
    audio: Res<Audio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    config: Res<GameConfig>,
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
                    .with_volume(Volume::Amplitude(config.bgm_volume as f64))
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
            .with_volume(Volume::Amplitude(config.bgm_volume as f64))
            .looped()
            .handle();
        current_bgm.0 = Some(SourceAndInstance {
            instance: instance.clone(),
            source: next.clone(),
        });
    }
}

fn update_bgm_volue(
    mut current_bgm: ResMut<CurrentBGM>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    config: Res<GameConfig>,
) {
    if config.is_changed() {
        if let Some(ref mut current_handle) = &mut current_bgm.0 {
            if let Some(instance) = audio_instances.get_mut(&current_handle.instance) {
                instance.set_volume(
                    Volume::Amplitude(config.bgm_volume as f64),
                    AudioTween::linear(Duration::from_millis(100)),
                );
            }
        }
    }
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, change_bgm.before(PhysicsSet::SyncBackend));
        app.add_systems(Update, update_bgm_volue);
        app.init_resource::<BGM>();
        app.init_resource::<CurrentBGM>();
    }
}
