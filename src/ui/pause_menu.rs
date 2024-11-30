use super::label::spawn_label;
use crate::command::GameCommand;
use crate::config::GameConfig;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::input::MyGamepad;
use crate::language::{Dict, Languages};
use crate::level::NextLevel;
use crate::states::GameMenuState;
use crate::ui::menu_button::menu_button;
use crate::ui::range::spawn_range;
use crate::{asset::GameAssets, states::GameState};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::plugin::{PhysicsSet, RapierConfiguration};
use bevy_simple_websocket::ClientMessage;

#[derive(Resource)]
struct ButtonShots {
    close: SystemId,
    exit: SystemId,
    bgm_volume_up: SystemId,
    bgm_volume_down: SystemId,
    se_volume_up: SystemId,
    se_volume_down: SystemId,
    ja: SystemId,
    en: SystemId,
    fullscreen_on: SystemId,
    fullscreen_off: SystemId,
    wait: i32,
}

#[derive(Component)]
struct PauseMenuRoot;

#[derive(Component)]
struct BGMVolumeLabel;

#[derive(Component)]
struct SEVolumeLabel;

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            close: world.register_system(resume),
            exit: world.register_system(exit),
            bgm_volume_up: world.register_system(volume_up),
            bgm_volume_down: world.register_system(volume_down),
            se_volume_up: world.register_system(se_volume_up),
            se_volume_down: world.register_system(se_volume_down),
            ja: world.register_system(ja),
            en: world.register_system(en),
            fullscreen_on: world.register_system(fullscreen_on),
            fullscreen_off: world.register_system(fullscreen_off),
            wait: 0,
        }
    }
}

fn resume(mut state: ResMut<NextState<GameMenuState>>, mut writer: EventWriter<GameCommand>) {
    state.set(GameMenuState::PauseMenuClosing);
    writer.send(GameCommand::SEClick(None));
}

fn exit(
    mut writer: EventWriter<GameCommand>,
    mut next: ResMut<NextLevel>,
    mut config: ResMut<GameConfig>,
    mut websocket: EventWriter<ClientMessage>,
) {
    writer.send(GameCommand::StateMainMenu);
    writer.send(GameCommand::SEClick(None));
    *next = NextLevel::None;

    if config.online {
        websocket.send(ClientMessage::Close);
        config.online = false;
    }
}

fn volume_up(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.bgm_volume = (config.bgm_volume + 0.1).min(1.0);
    writer.send(GameCommand::SEClick(None));
}

fn volume_down(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.bgm_volume = (config.bgm_volume - 0.1).max(0.0);
    writer.send(GameCommand::SEClick(None));
}

fn se_volume_up(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.se_volume = (config.se_volume + 0.1).min(1.0);
    writer.send(GameCommand::SEClick(None));
}

fn se_volume_down(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.se_volume = (config.se_volume - 0.1).max(0.0);
    writer.send(GameCommand::SEClick(None));
}

fn ja(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.language = Languages::Ja;
    writer.send(GameCommand::SEClick(None));
}

fn en(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.language = Languages::En;
    writer.send(GameCommand::SEClick(None));
}

fn fullscreen_on(
    mut config: ResMut<GameConfig>,
    mut writer: EventWriter<GameCommand>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    window.mode = WindowMode::SizedFullscreen;
    writer.send(GameCommand::SEClick(None));
    config.fullscreen = true;
}

fn fullscreen_off(
    mut config: ResMut<GameConfig>,
    mut writer: EventWriter<GameCommand>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    window.mode = WindowMode::Windowed;
    writer.send(GameCommand::SEClick(None));
    config.fullscreen = false;
}

fn setup_game_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    shots: Res<ButtonShots>,
    config: Res<GameConfig>,
) {
    commands
        .spawn((
            PauseMenuRoot,
            StateScoped(GameState::InGame),
            Name::new("Pause Menu"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(10.0),
                    ..Default::default()
                },
                background_color: Color::hsla(0.0, 0.0, 0.05, 1.0).into(),
                z_index: ZIndex::Global(GAME_MENU_Z_INDEX),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_label(
                        parent,
                        &assets,
                        Dict {
                            ja: "ポーズ中",
                            en: "Paused",
                        },
                    );

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                column_gap: Val::Px(4.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_label(
                                parent,
                                &assets,
                                Dict {
                                    ja: "言語/Language",
                                    en: "言語/Language",
                                },
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.ja,
                                120.0,
                                40.0,
                                Dict {
                                    ja: "日本語",
                                    en: "日本語",
                                },
                            );
                            menu_button(
                                parent,
                                &assets,
                                shots.en,
                                80.0,
                                40.0,
                                Dict { ja: "En", en: "En" },
                            );
                        });

                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                column_gap: Val::Px(4.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_label(
                                parent,
                                &assets,
                                Dict {
                                    ja: "フルスクリーン",
                                    en: "Full Screen",
                                },
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.fullscreen_on,
                                80.0,
                                40.0,
                                Dict {
                                    ja: "オン",
                                    en: "On",
                                },
                            );
                            menu_button(
                                parent,
                                &assets,
                                shots.fullscreen_off,
                                80.0,
                                40.0,
                                Dict {
                                    ja: "オフ",
                                    en: "Off",
                                },
                            );
                        });

                    spawn_range(
                        parent,
                        &assets,
                        BGMVolumeLabel,
                        (10.0 * config.bgm_volume).round() as u32,
                        shots.bgm_volume_up,
                        shots.bgm_volume_down,
                        Dict {
                            ja: "BGM音量",
                            en: "BGM Volume",
                        },
                    );

                    spawn_range(
                        parent,
                        &assets,
                        SEVolumeLabel,
                        (10.0 * config.se_volume).round() as u32,
                        shots.se_volume_up,
                        shots.se_volume_down,
                        Dict {
                            ja: "効果音量",
                            en: "SFX Volume",
                        },
                    );

                    menu_button(
                        parent,
                        &assets,
                        shots.close,
                        280.0,
                        60.0,
                        Dict {
                            ja: "再開",
                            en: "Back",
                        },
                    );

                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(280.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        ..default()
                    });

                    menu_button(
                        parent,
                        &assets,
                        shots.exit,
                        280.0,
                        60.0,
                        Dict {
                            ja: "タイトルに戻る",
                            en: "Return to Title",
                        },
                    );
                });
        });
}

fn update_game_menu(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    mut query: Query<&mut Visibility, With<PauseMenuRoot>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let mut visibility = query.single_mut();
    *visibility = match state.get() {
        GameMenuState::PauseMenuOpen => Visibility::Visible,
        _ => Visibility::Hidden,
    };

    if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
        if gamepad_buttons.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }) {
            next.set(match state.get() {
                GameMenuState::Closed => GameMenuState::PauseMenuOpen,
                _ => GameMenuState::PauseMenuClosing,
            });
        }
    }
}

fn handle_escape_key(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match *state.get() {
            GameMenuState::Closed => {
                next.set(GameMenuState::PauseMenuOpen);
            }
            _ => {
                next.set(GameMenuState::Closed);
            }
        }
    }
}

fn switch_physics_activation(
    state: Res<State<GameMenuState>>,
    mut rapier_state: ResMut<RapierConfiguration>,
) {
    if state.is_changed() {
        match *state.get() {
            GameMenuState::PauseMenuOpen => {
                rapier_state.physics_pipeline_active = false;
                rapier_state.query_pipeline_active = false;
            }
            _ => {
                rapier_state.physics_pipeline_active = true;
                rapier_state.query_pipeline_active = true;
            }
        }
    }
}

fn update_bgm_volume_label(
    config: Res<GameConfig>,
    mut query: Query<&mut Text, With<BGMVolumeLabel>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", (10.0 * config.bgm_volume).round() as u32);
    }
}

fn update_se_volume_label(
    config: Res<GameConfig>,
    mut query: Query<&mut Text, With<SEVolumeLabel>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", (10.0 * config.se_volume).round() as u32);
    }
}

fn closing_to_closed(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    mut res: ResMut<ButtonShots>,
) {
    match state.get() {
        GameMenuState::PauseMenuOpen => {
            res.wait = 20;
        }
        GameMenuState::PauseMenuClosing => {
            res.wait = (res.wait - 1).max(0);
            if res.wait <= 0 {
                next.set(GameMenuState::Closed);
            }
        }
        _ => {}
    }
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game_menu);
        app.add_systems(
            Update,
            (
                update_game_menu,
                update_se_volume_label,
                update_bgm_volume_label,
                handle_escape_key,
                switch_physics_activation,
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            closing_to_closed
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.init_resource::<ButtonShots>();
    }
}
