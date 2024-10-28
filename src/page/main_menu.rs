use crate::audio::play_se;
use crate::bgm::BGM;
use crate::config::GameConfig;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::hud::overlay::OverlayNextState;
use crate::ui::button::button;
use crate::ui::on_press::OnPress;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::ecs::system::SystemId;
use bevy::gizmos::config;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteSliceUiBundle;
use bevy_kira_audio::Audio;

#[derive(Resource)]
struct ButtonShots {
    start: SystemId,
    config: SystemId,
    exit: SystemId,
}

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            start: world.register_system(start_game),
            config: world.register_system(config_game),
            exit: world.register_system(exit_game),
        }
    }
}

fn start_game(
    assets: Res<GameAssets>,
    mut query: Query<&mut Visibility, With<OnPress>>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    mut next_bgm: ResMut<BGM>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
    menu_next_state.set(MainMenuPhase::Paused);
    *overlay_next_state = OverlayNextState(Some(GameState::InGame));
    *next_bgm = BGM(None);

    play_se(&audio, &config, assets.kettei.clone());
}

fn config_game(
    mut overlay_next_state: ResMut<OverlayNextState>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    *overlay_next_state = OverlayNextState(Some(GameState::Config));
    play_se(&audio, &config, assets.kettei.clone());
}

fn exit_game(mut commands: Commands, window_query: Query<Entity, With<Window>>) {
    for window in window_query.iter() {
        commands.entity(window).despawn();
    }
}

fn setup_main_menu(
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
            StateScoped(GameState::MainMenu),
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
                shots.start,
                GameState::MainMenu,
                "Start Game",
                30.0,
                96.0,
                84.0,
                16.0,
            );

            button(
                parent,
                &assets,
                shots.config,
                GameState::MainMenu,
                "Config",
                30.0,
                123.0,
                84.0,
                16.0,
            );
            button(
                parent,
                &assets,
                shots.exit,
                GameState::MainMenu,
                "Exit",
                30.0,
                142.0,
                84.0,
                16.0,
            );
        });

    commands.spawn((
        StateScoped(GameState::MainMenu),
        ImageBundle {
            z_index: ZIndex::Global(-1000),
            style: Style {
                width: Val::Px(1280.0),
                height: Val::Px(720.0),
                ..default()
            },
            ..default()
        },
        AsepriteSliceUiBundle {
            slice: "all".into(),
            aseprite: assets.title.clone(),
            ..default()
        },
    ));
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.init_resource::<ButtonShots>();
    }
}
