use crate::controller::player::Player;
use crate::entity::actor::ActorState;
use crate::states::GameState;
use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
pub struct WitchWandSprite;

fn update_volume(
    mut witch_query: Query<(&ActorState, &AudioSink), (With<Player>, Changed<ActorState>)>,
) {
    for (state, sink) in witch_query.iter_mut() {
        sink.set_volume(match state {
            ActorState::Idle => 0.0,
            ActorState::Run => 0.4,
        });
    }
}

pub struct FootStepsPlugin;

impl Plugin for FootStepsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_volume).run_if(in_state(GameState::InGame)));
    }
}
