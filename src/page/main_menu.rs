use crate::command::GameCommand;
use crate::config::GameConfig;
use crate::constant::{GAME_MENU_Z_INDEX, HUD_Z_INDEX};
use crate::ui::button::button;
use crate::ui::on_press::OnPress;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteSliceUiBundle;
use git_version::git_version;

#[derive(Resource)]
struct ButtonShots {
    single: SystemId,

    #[allow(dead_code)]
    exit: SystemId,
}

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            single: world.register_system(start_single_player),
            exit: world.register_system(exit_game),
        }
    }
}

fn start_single_player(
    mut query: Query<&mut Visibility, With<OnPress>>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut writer: EventWriter<GameCommand>,
    mut config: ResMut<GameConfig>,
) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
    config.online = false;
    menu_next_state.set(MainMenuPhase::Paused);
    writer.send(GameCommand::SEKettei(None));
    writer.send(GameCommand::StateInGame);
    writer.send(GameCommand::BGMNone);
}

fn exit_game(mut commands: Commands, window_query: Query<Entity, With<Window>>) {
    for window in window_query.iter() {
        commands.entity(window).despawn();
    }
}

fn setup_main_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    shots: Res<ButtonShots>,
    mut writer: EventWriter<GameCommand>,
) {
    writer.send(GameCommand::BGMBoubaku);

    commands
        .spawn((
            StateScoped(GameState::MainMenu),
            Name::new("main menu"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(400.0),
                    left: Val::Px(60.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..Default::default()
                },
                z_index: ZIndex::Global(GAME_MENU_Z_INDEX),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            button(parent, &assets, shots.single, "Start", 84.0, 16.0);

            #[cfg(not(target_arch = "wasm32"))]
            button(parent, &assets, shots.exit, "Exit", 84.0, 16.0);
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

    commands.spawn((
        StateScoped(GameState::MainMenu),
        Name::new("Git Version"),
        TextBundle {
            text: Text::from_section(
                format!("Version: {}", git_version!()),
                TextStyle {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                    font_size: 12.0,
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(700.0),
                ..default()
            },
            z_index: ZIndex::Global(HUD_Z_INDEX),

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
