use crate::asset::GameAssets;
use crate::audio::play_se;
use crate::config::GameConfig;
use crate::hud::overlay::OverlayNextState;
use crate::states::GameState;
use crate::ui::button::button;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

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

fn back(
    assets: Res<GameAssets>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
    play_se(&audio, &config, assets.kettei.clone());
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
            button(
                parent,
                &assets,
                shots.back,
                GameState::Config,
                "Back",
                10.0,
                10.0,
                84.0,
                16.0,
            );
        });
}

pub struct ConfigPagePlugin;

impl Plugin for ConfigPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Config), setup);
        app.init_resource::<ButtonShots>();
    }
}
