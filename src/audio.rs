use bevy::prelude::*;
use bevy_kira_audio::{prelude::Volume, Audio, AudioControl, AudioInstance, AudioSource};

use crate::config::GameConfig;

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
