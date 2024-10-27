use crate::audio::play_se;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::hud::overlay::OverlayNextState;
use crate::ui::button::button;
use crate::{asset::GameAssets, states::GameState};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

#[derive(Resource)]
struct ButtonShots {
    exit: SystemId,
}

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            exit: world.register_system(exit),
        }
    }
}

fn exit(
    mut overlay_next_state: ResMut<OverlayNextState>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
) {
    *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
    play_se(assets.kettei.clone(), &audio);
}

fn setup_game_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    shots: Res<ButtonShots>,
) {
    let mut camera = camera_query.single_mut();
    camera.translation.x = 0.0;
    camera.translation.y = 0.0;

    commands
        .spawn((
            StateScoped(GameState::InGame),
            Name::new("main menu"),
            NodeBundle {
                style: Style {
                    width: Val::Px(1280.0),
                    height: Val::Px(720.0),
                    ..Default::default()
                },
                z_index: ZIndex::Global(GAME_MENU_Z_INDEX),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            button(
                parent,
                &assets,
                shots.exit,
                GameState::InGame,
                "Back to Main Menu",
                240.0,
                8.0,
                124.0,
                16.0,
            );
        });
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game_menu);
        app.init_resource::<ButtonShots>();
    }
}
