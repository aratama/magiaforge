use bevy::prelude::*;

use crate::{controller::player::Player, states::GameState};

use super::overlay::OverlayEvent;

#[derive(Resource, Default)]
struct GameOver {
    animation: u32,
}

fn setup(mut gameover: ResMut<GameOver>) {
    gameover.animation = 0;
}

fn gameover(
    player_query: Query<&Player>,
    mut gameover: ResMut<GameOver>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
) {
    if player_query.is_empty() {
        if gameover.animation == 300 {
            overlay_event_writer.send(OverlayEvent::Close(GameState::Warp));
        }
        gameover.animation += 1;
    }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameOver>();
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(Update, gameover.run_if(in_state(GameState::InGame)));
    }
}
