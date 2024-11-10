use crate::{hud::overlay::OverlayEvent, states::GameState};
use bevy::prelude::*;

fn on_enter_warp(mut overlay_writer: EventWriter<OverlayEvent>) {
    overlay_writer.send(OverlayEvent::Close(GameState::InGame));
}

pub struct WarpPagePlugin;

impl Plugin for WarpPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Warp), on_enter_warp);
    }
}
