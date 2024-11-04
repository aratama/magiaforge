use crate::asset::GameAssets;
use crate::command::GameCommand;
use crate::states::GameState;
use crate::ui::button::button;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Resource)]
struct ButtonShots {
    back: SystemId,
}

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            back: world.register_system(back),
        }
    }
}

fn back(mut writer: EventWriter<GameCommand>) {
    writer.send(GameCommand::StateMainMenu);
    writer.send(GameCommand::SEKettei);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>, shots: Res<ButtonShots>) {
    commands
        .spawn((
            StateScoped(GameState::Config),
            Name::new("main menu"),
            NodeBundle {
                style: Style {
                    width: Val::Px(1280.0),
                    height: Val::Px(720.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            button(parent, &assets, shots.back, "Back", 10.0, 10.0, 84.0, 16.0);
        });
}

pub struct ConfigPagePlugin;

impl Plugin for ConfigPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Config), setup);
        app.init_resource::<ButtonShots>();
    }
}
