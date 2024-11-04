use crate::{command::GameCommand, states::GameState};
use bevy::prelude::*;

fn on_enter_warp(mut step: Local<i32>) {
    *step = 0;
}

fn update_warp(mut step: Local<i32>, mut writer: EventWriter<GameCommand>) {
    *step += 1;
    if 60 <= *step {
        *step = 0;
        writer.send(GameCommand::StateInGame);
    }
}

pub struct WarpPagePlugin;

impl Plugin for WarpPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Warp), on_enter_warp);

        app.add_systems(FixedUpdate, update_warp.run_if(in_state(GameState::Warp)));
    }
}
