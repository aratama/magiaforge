use crate::constant::ARENA;
use crate::hud::overlay::OverlayEvent;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::states::GameState;
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut overlay_writer: EventWriter<OverlayEvent>,
    next_level: Res<LevelSetup>,
) {
    commands.spawn((StateScoped(GameState::Warp), Camera2d::default()));
    overlay_writer.send(OverlayEvent::Close(
        if next_level.next_level == GameLevel::new(ARENA) {
            GameState::NameInput
        } else {
            GameState::InGame
        },
    ));
}

pub struct WarpPagePlugin;

impl Plugin for WarpPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Warp), setup);
    }
}
