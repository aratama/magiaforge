use crate::command::GameCommand;
use crate::config::GameConfig;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::input::MyGamepad;
use crate::states::GameMenuState;
use crate::ui::menu_button::menu_button;
use crate::ui::range::spawn_range;
use crate::{asset::GameAssets, states::GameState};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Resource)]
struct ButtonShots {
    close: SystemId,
    exit: SystemId,
    bgm_volume_up: SystemId,
    bgm_volume_down: SystemId,
    se_volume_up: SystemId,
    se_volume_down: SystemId,
    wait: i32,
}

#[derive(Component)]
struct GameMenuRoot;

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            close: world.register_system(resume),
            exit: world.register_system(exit),
            bgm_volume_up: world.register_system(volume_up),
            bgm_volume_down: world.register_system(volume_down),
            se_volume_up: world.register_system(se_volume_up),
            se_volume_down: world.register_system(se_volume_down),
            wait: 0,
        }
    }
}

fn resume(mut state: ResMut<NextState<GameMenuState>>, mut writer: EventWriter<GameCommand>) {
    state.set(GameMenuState::Closing);
    writer.send(GameCommand::SEKettei);
}

fn exit(mut writer: EventWriter<GameCommand>) {
    writer.send(GameCommand::StateMainMenu);
    writer.send(GameCommand::SEKettei);
}

fn volume_up(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.bgm_volume = (config.bgm_volume + 0.1).min(1.0);
    writer.send(GameCommand::SEKettei);
}

fn volume_down(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.bgm_volume = (config.bgm_volume - 0.1).max(0.0);
    writer.send(GameCommand::SEKettei);
}

fn se_volume_up(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.se_volume = (config.se_volume + 0.1).min(1.0);
    writer.send(GameCommand::SEKettei);
}

fn se_volume_down(mut config: ResMut<GameConfig>, mut writer: EventWriter<GameCommand>) {
    config.se_volume = (config.se_volume - 0.1).max(0.0);
    writer.send(GameCommand::SEKettei);
}

fn setup_game_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    shots: Res<ButtonShots>,
    config: Res<GameConfig>,
) {
    commands
        .spawn((
            GameMenuRoot,
            StateScoped(GameState::InGame),
            Name::new("game menu"),
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
                background_color: Color::hsla(0.0, 0.0, 0.1, 0.9).into(),
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
                    parent.spawn(TextBundle::from_section(
                        "Game Menu".to_string(),
                        TextStyle {
                            font_size: 40.0,
                            font: assets.dotgothic.clone(),
                            color: Color::srgba(0.9, 0.9, 0.9, 0.4),
                            ..default()
                        },
                    ));

                    spawn_range(
                        parent,
                        &assets,
                        BGMVolumeLabel,
                        "BGM Volume",
                        (10.0 * config.bgm_volume).round() as u32,
                        shots.bgm_volume_up,
                        shots.bgm_volume_down,
                    );

                    spawn_range(
                        parent,
                        &assets,
                        SEVolumeLabel,
                        "SFX Volume",
                        (10.0 * config.se_volume).round() as u32,
                        shots.se_volume_up,
                        shots.se_volume_down,
                    );

                    menu_button(parent, &assets, shots.close, "Resume", 280.0, 60.0);

                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(280.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        ..default()
                    });

                    menu_button(parent, &assets, shots.exit, "Quit", 280.0, 60.0);
                });
        });
}

fn update_game_menu(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    mut query: Query<&mut Visibility, With<GameMenuRoot>>,

    keys: Res<ButtonInput<KeyCode>>,

    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let mut visibility = query.single_mut();
    *visibility = match state.get() {
        GameMenuState::Open => Visibility::Visible,
        _ => Visibility::Hidden,
    };

    if keys.just_pressed(KeyCode::Escape) {
        next.set(match state.get() {
            GameMenuState::Closed => GameMenuState::Open,
            _ => GameMenuState::Closing,
        });
    }

    if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
        if gamepad_buttons.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }) {
            next.set(match state.get() {
                GameMenuState::Closed => GameMenuState::Open,
                _ => GameMenuState::Closing,
            });
        }
    }
}

#[derive(Component)]
struct BGMVolumeLabel;

fn update_bgm_volume_label(
    config: Res<GameConfig>,
    mut query: Query<&mut Text, With<BGMVolumeLabel>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", (10.0 * config.bgm_volume).round() as u32);
    }
}

#[derive(Component)]
struct SEVolumeLabel;

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
        GameMenuState::Open => {
            res.wait = 20;
        }
        GameMenuState::Closing => {
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
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            closing_to_closed.run_if(in_state(GameState::InGame)),
        );
        app.init_resource::<ButtonShots>();
    }
}
