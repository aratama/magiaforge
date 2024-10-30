use crate::audio::play_se;
use crate::bgm::BGM;
use crate::config::GameConfig;
use crate::hud::overlay::OverlayNextState;
use crate::ui::menu_button::menu_button;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_simple_text_input::{TextInputBundle, TextInputSettings, TextInputValue};

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Resource)]
struct ButtonShots {
    start: SystemId,
}

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            start: world.register_system(start_game),
        }
    }
}

fn start_game(
    assets: Res<GameAssets>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    mut next_bgm: ResMut<BGM>,
    audio: Res<Audio>,
    mut config: ResMut<GameConfig>,

    query: Query<&TextInputValue>,
) {
    menu_next_state.set(MainMenuPhase::Paused);
    *overlay_next_state = OverlayNextState(Some(GameState::InGame));
    *next_bgm = BGM(None);

    let q = query.single();
    config.player_name = q.0.clone();

    play_se(&audio, &config, assets.kettei.clone());
}

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    shots: Res<ButtonShots>,
    config: Res<GameConfig>,
) {
    commands
        .spawn((
            StateScoped(GameState::NameInput),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Input Your Name",
                        TextStyle {
                            font_size: 60.0,
                            font: assets.dotgothic.clone(),
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        border: UiRect::all(Val::Px(5.0)),
                                        padding: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    border_color: BORDER_COLOR_ACTIVE.into(),
                                    background_color: BACKGROUND_COLOR.into(),
                                    ..default()
                                },
                                TextInputBundle::default()
                                    .with_text_style(TextStyle {
                                        font_size: 40.,
                                        color: TEXT_COLOR,
                                        ..default()
                                    })
                                    .with_settings(TextInputSettings {
                                        retain_on_submit: true,
                                        ..default()
                                    })
                                    .with_value(config.player_name.clone()),
                            ));

                            menu_button(parent, &assets, shots.start, "Start", 120.0, 50.0);
                        });
                });
        });
}

pub struct NameInputPagePlugin;

impl Plugin for NameInputPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::NameInput), setup);
        app.init_resource::<ButtonShots>();
    }
}
