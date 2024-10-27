use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

pub fn play_se(source: Handle<AudioSource>, audio: &Res<Audio>) {
    audio.play(source.clone());
}
