use crate::{hud::overlay::OverlayEvent, states::GameState, level::NextLevel};
use bevy::prelude::*;

fn on_enter_warp(mut overlay_writer: EventWriter<OverlayEvent>, next_level: Res<NextLevel>) {
    overlay_writer.send(OverlayEvent::Close(match *next_level {
        NextLevel::Level(_, _) => GameState::InGame,
        NextLevel::None => GameState::InGame,
        NextLevel::MultiPlayArena(_) => GameState::NameInput,
    }));
}

pub struct WarpPagePlugin;

impl Plugin for WarpPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Warp), on_enter_warp);
    }
}
