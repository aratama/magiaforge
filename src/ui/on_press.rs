use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct OnPress(pub SystemId);

fn interaction(
    mut commands: Commands,
    mut interactions: Query<(&OnPress, &Interaction), Changed<Interaction>>,
) {
    for (button_type, interaction) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                commands.run_system(button_type.0);
            }
            _ => {}
        }
    }
}

pub struct OnPressPlugin;

impl Plugin for OnPressPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interaction);
    }
}
