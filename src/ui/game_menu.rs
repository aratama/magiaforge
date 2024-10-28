use crate::audio::play_se;
use crate::config::GameConfig;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::hud::overlay::OverlayNextState;
use crate::input::MyGamepad;
use crate::states::GameMenuState;
use crate::ui::button::button;
use crate::ui::menu_button::menu_button;
use crate::ui::range::spawn_range;
use crate::{asset::GameAssets, states::GameState};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_blur_regions::BlurRegion;
use bevy_kira_audio::Audio;

#[derive(Resource)]
struct ButtonShots {
    close: SystemId,
    exit: SystemId,
    bgm_volume_up: SystemId,
    bgm_volume_down: SystemId,
    se_volume_up: SystemId,
    se_volume_down: SystemId,
}

#[derive(Component)]
struct GameMenuRoot;

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            close: world.register_system(close),
            exit: world.register_system(exit),
            bgm_volume_up: world.register_system(volume_up),
            bgm_volume_down: world.register_system(volume_down),
            se_volume_up: world.register_system(se_volume_up),
            se_volume_down: world.register_system(se_volume_down),
        }
    }
}

fn close(
    mut state: ResMut<NextState<GameMenuState>>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    state.set(GameMenuState::Close);
    play_se(&audio, &config, assets.kettei.clone());
}

fn exit(
    mut overlay_next_state: ResMut<OverlayNextState>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
    play_se(&audio, &config, assets.kettei.clone());
}

fn volume_up(mut config: ResMut<GameConfig>, assets: Res<GameAssets>, audio: Res<Audio>) {
    config.bgm_volume = (config.bgm_volume + 0.1).min(1.0);
    play_se(&audio, &config, assets.kettei.clone());
}

fn volume_down(mut config: ResMut<GameConfig>, assets: Res<GameAssets>, audio: Res<Audio>) {
    config.bgm_volume = (config.bgm_volume - 0.1).max(0.0);
    play_se(&audio, &config, assets.kettei.clone());
}

fn se_volume_up(mut config: ResMut<GameConfig>, assets: Res<GameAssets>, audio: Res<Audio>) {
    config.se_volume = (config.se_volume + 0.1).min(1.0);
    play_se(&audio, &config, assets.kettei.clone());
}

fn se_volume_down(mut config: ResMut<GameConfig>, assets: Res<GameAssets>, audio: Res<Audio>) {
    config.se_volume = (config.se_volume - 0.1).max(0.0);
    play_se(&audio, &config, assets.kettei.clone());
}

fn setup_game_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    shots: Res<ButtonShots>,
    config: Res<GameConfig>,
) {
    let mut camera = camera_query.single_mut();
    camera.translation.x = 0.0;
    camera.translation.y = 0.0;

    commands
        .spawn((
            GameMenuRoot,
            StateScoped(GameState::InGame),
            Name::new("main menu"),
            BlurRegion,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    width: Val::Px(1280.0),
                    height: Val::Px(720.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    row_gap: Val::Px(10.0),
                    ..Default::default()
                },
                background_color: Color::hsla(0.0, 0.0, 0.0, 0.4).into(),
                z_index: ZIndex::Global(GAME_MENU_Z_INDEX),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    StateScoped(GameState::InGame),
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(80.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Menu".to_string(),
                        TextStyle {
                            font_size: 40.0,
                            font: assets.dotgothic.clone(),
                            color: Color::srgba(0.9, 0.9, 0.9, 0.4),
                            ..default()
                        },
                    ));
                });

            menu_button(parent, &assets, shots.close, "Close", 180.0, 60.0);

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

            menu_button(parent, &assets, shots.exit, "Back to Home", 280.0, 60.0);
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
        GameMenuState::Close => Visibility::Hidden,
        GameMenuState::Open => Visibility::Visible,
    };

    if keys.just_pressed(KeyCode::Escape) {
        next.set(match state.get() {
            GameMenuState::Close => GameMenuState::Open,
            GameMenuState::Open => GameMenuState::Close,
        });
    }

    if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
        if gamepad_buttons.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }) {
            next.set(match state.get() {
                GameMenuState::Close => GameMenuState::Open,
                GameMenuState::Open => GameMenuState::Close,
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

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game_menu);
        app.add_systems(Update, update_game_menu.run_if(in_state(GameState::InGame)));
        app.add_systems(
            Update,
            update_se_volume_label.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            update_bgm_volume_label.run_if(in_state(GameState::InGame)),
        );
        app.init_resource::<ButtonShots>();
    }
}
