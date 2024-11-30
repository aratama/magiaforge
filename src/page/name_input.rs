use crate::audio::NextBGM;
use crate::command::GameCommand;
use crate::config::GameConfig;
use crate::language::Dict;
use crate::ui::menu_button::menu_button;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
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
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut config: ResMut<GameConfig>,
    query: Query<&TextInputValue>,
    mut writer: EventWriter<GameCommand>,
    mut next_bgm: ResMut<NextBGM>,
) {
    menu_next_state.set(MainMenuPhase::Paused);
    writer.send(GameCommand::StateInGame);
    *next_bgm = NextBGM(None);

    let q = query.single();
    config.player_name = q.0.clone();

    writer.send(GameCommand::SEClick(None));
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
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        Dict {
                            ja: "名前を入力してください",
                            en: "Input Your Name",
                        }
                        .get(config.language),
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
                                column_gap: Val::Px(10.0),
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

                            menu_button(
                                parent,
                                &assets,
                                shots.start,
                                160.0,
                                60.0,
                                Dict {
                                    ja: "スタート",
                                    en: "Start",
                                },
                            );
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
