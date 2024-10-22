use super::bgm::BGM;
use super::hud::overlay::OverlayNextState;
use super::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteSliceUiBundle;

pub fn setup_main_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
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
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    StateScoped(GameState::MainMenu),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::DEFAULT,
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
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

pub fn update_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Visibility),
        (Changed<Interaction>, With<Button>),
    >,
    menu_current_state: Res<State<MainMenuPhase>>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    mut next_bgm: ResMut<BGM>,
) {
    for (interaction, mut color, mut visibility) in &mut interaction_query {
        *visibility = if *menu_current_state == MainMenuPhase::Paused {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        match *interaction {
            Interaction::Pressed => {
                menu_next_state.set(MainMenuPhase::Paused);
                *overlay_next_state = OverlayNextState(Some(GameState::InGame));
                *next_bgm = BGM(None);
            }
            Interaction::Hovered => {
                *color = Color::hsl(0.0, 0.0, 0.5).into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

pub struct StartPagePlugin;

impl Plugin for StartPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.add_systems(
            FixedUpdate,
            update_main_menu.run_if(in_state(GameState::MainMenu)),
        );
    }
}
