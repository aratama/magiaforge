use crate::{constant::LOADING_Z_INDEX, states::GameState};
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        StateScoped(GameState::Setup),
        Text::new("Loading..."),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(1100.0),
            top: Val::Px(670.0),
            ..default()
        },
        GlobalZIndex(LOADING_Z_INDEX),
    ));
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
