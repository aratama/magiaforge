use crate::entity::actor::AnimationState;
use crate::entity::witch::ActorState;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

#[derive(Default, Component, Reflect)]
pub struct WitchWandSprite;

#[derive(Default, Component, Reflect)]
pub struct Footsteps(Handle<AudioInstance>);

fn update_volume(
    mut witch_query: Query<(&ActorState, &mut Footsteps), Changed<ActorState>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for (state, footsteps) in witch_query.iter_mut() {
        if let Some(instance) = audio_instances.get_mut(&footsteps.0) {
            let volume = match state.0 {
                AnimationState::Idle => 0.0,
                AnimationState::Walk => 0.6,
            };
            instance.set_volume(volume, AudioTween::linear(Duration::from_millis(200)));
        }
    }
}

pub struct FootStepsPlugin;

impl Plugin for FootStepsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_volume).run_if(in_state(GameState::InGame)));
    }
}
