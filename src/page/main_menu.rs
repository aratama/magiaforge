use super::in_game::GameLevel;
use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::config::GameConfig;
use crate::constant::HUD_Z_INDEX;
use crate::hud::overlay::OverlayEvent;
use crate::language::language_to_font;
use crate::language::Languages;
use crate::language::M18NTtext;
use crate::message::CLICK_TO_START;
use crate::page::in_game::Interlevel;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::states::MainMenuPhase;
use crate::ui::on_press::OnPress;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use git_version::git_version;
use strum::IntoEnumIterator;

const SCALE: f32 = 4.0;

#[derive(Component)]
struct WitchAnimation;

#[derive(Component)]
struct CloudAnimation {
    left: f32,
    speed: f32,
    offset: f32,
}

#[derive(Event, PartialEq, Eq, Debug, Clone, Copy)]
enum Events {
    Start,
}

#[derive(Component)]
struct LanguageButton {
    language: Languages,
}

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut next_bgm: ResMut<NextBGM>,
    mut current: ResMut<Interlevel>,
) {
    commands.spawn((Camera2d::default(), StateScoped(GameState::MainMenu)));

    *next_bgm = NextBGM(Some(assets.boubaku.clone()));

    current.next_level = GameLevel::Level(0);

    commands.spawn((
        Name::new("main_menu"),
        StateScoped(GameState::MainMenu),
        GlobalZIndex(-1000),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Node {
            width: Val::Px(1280.0),
            height: Val::Px(720.0),
            ..default()
        },
        AseUiSlice {
            name: "all".into(),
            aseprite: assets.title.clone(),
        },
    ));

    spawn_cloud(
        &mut commands,
        &assets.title_cloud,
        CloudAnimation {
            left: 0.0,
            speed: 0.0003,
            offset: 0.0,
        },
        -950,
    );
    spawn_cloud(
        &mut commands,
        &assets.title_cloud,
        CloudAnimation {
            left: 1024.0,
            speed: 0.0003,
            offset: 0.0,
        },
        -950,
    );
    spawn_cloud(
        &mut commands,
        &assets.title_cloud2,
        CloudAnimation {
            left: 0.0,
            speed: 0.0001,
            offset: -600.0,
        },
        -900,
    );
    spawn_cloud(
        &mut commands,
        &assets.title_cloud2,
        CloudAnimation {
            left: 1024.0,
            speed: 0.0001,
            offset: -600.0,
        },
        -900,
    );

    commands.spawn((
        Name::new("witch"),
        WitchAnimation,
        StateScoped(GameState::MainMenu),
        GlobalZIndex(-800),
        Node {
            left: Val::Px(800.0),
            top: Val::Px(0.0),
            width: Val::Px(96.0 * SCALE),
            height: Val::Px(96.0 * SCALE),
            ..default()
        },
        AseUiAnimation {
            aseprite: assets.title_witch.clone(),
            animation: "idle".into(),
        },
    ));

    commands.spawn((
        Name::new("title_logo"),
        StateScoped(GameState::MainMenu),
        GlobalZIndex(-700),
        Node {
            left: Val::Px(400.0),
            top: Val::Px(300.0),
            width: Val::Px(128.0 * SCALE),
            height: Val::Px(48.0 * SCALE),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "title_logo".into(),
        },
    ));

    commands
        .spawn((
            Name::new("click_to_start"),
            StateScoped(GameState::MainMenu),
            GlobalZIndex(-700),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(20.0),
                width: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_child((
            M18NTtext(CLICK_TO_START.to_string()),
            TextColor::from(Color::WHITE),
            TextFont {
                font_size: 16.0,
                font: assets.noto_sans_jp.clone(),
                ..default()
            },
        ));

    commands.spawn((
        Name::new("Git Version"),
        StateScoped(GameState::MainMenu),
        GlobalZIndex(HUD_Z_INDEX),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(700.0),
            ..default()
        },
        Text::new(format!(
            "Version: {} ({})",
            git_version!(),
            std::env!("BUILD_DATETIME")
        )),
        TextColor::from(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        TextFont {
            font: assets.noto_sans_jp.clone(),
            font_size: 12.0,
            ..default()
        },
    ));

    commands
        .spawn((
            StateScoped(GameState::MainMenu),
            GlobalZIndex(HUD_Z_INDEX),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                bottom: Val::Px(20.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                ..default()
            },
        ))
        .with_children(|builder| {
            for language in Languages::iter() {
                builder
                    .spawn((
                        Name::new("language_button"),
                        LanguageButton { language },
                        Button,
                        BackgroundColor::from(Color::hsva(0.0, 0.0, 1.0, 0.05)),
                    ))
                    .with_children(|builder| {
                        builder
                            .spawn((Node {
                                padding: UiRect::new(
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                    Val::Px(4.0),
                                    Val::Px(4.0),
                                ),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                ..default()
                            },))
                            .with_children(|builder| {
                                builder.spawn((
                                    Text::new(match language {
                                        Languages::Ja => "日本語",
                                        Languages::En => "English",
                                        Languages::ZhCn => "中文",
                                        Languages::Es => "Español",
                                        Languages::Fr => "Français",
                                        Languages::Pt => "Português",
                                        Languages::Ru => "Русский",
                                        Languages::De => "Deutsch",
                                        Languages::Ko => "한국어",
                                    }),
                                    Label,
                                    TextColor::from(Color::hsl(0.0, 0.0, 0.0)),
                                    TextFont {
                                        font_size: 12.0,
                                        font: language_to_font(&assets, language),
                                        ..default()
                                    },
                                ));
                            });
                    });
            }
        });
}

fn spawn_cloud<T: Component>(
    commands: &mut Commands,
    aseprite: &Handle<Aseprite>,
    marker: T,
    z_index: i32,
) {
    commands.spawn((
        Name::new("cloud"),
        marker,
        StateScoped(GameState::MainMenu),
        GlobalZIndex(z_index),
        Node {
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Px(1024.0 * SCALE),
            height: Val::Px(180.0 * SCALE),
            ..default()
        },
        AseUiAnimation {
            aseprite: aseprite.clone(),
            animation: "default".into(),
        },
    ));
}

#[derive(Resource, Default)]
struct LanguageChanged(bool);

const UNSELECTED: Color = Color::hsva(0.0, 0.0, 1.0, 0.1);

const HOVERED: Color = Color::hsva(0.0, 0.0, 1.0, 0.3);

const SELECTED: Color = Color::hsva(0.0, 0.0, 1.0, 0.6);

fn toggle_language(
    mut query: Query<(&mut BackgroundColor, &Interaction, &LanguageButton), Changed<Interaction>>,
    mut config: ResMut<GameConfig>,
    mut changed: ResMut<LanguageChanged>,
    mut writer: EventWriter<SEEvent>,
) {
    changed.0 = false;

    for (mut background, interaction, button) in &mut query.iter_mut() {
        match interaction {
            Interaction::None => {
                background.0 = if config.language == button.language {
                    SELECTED
                } else {
                    UNSELECTED
                };
            }
            Interaction::Hovered => {
                background.0 = if config.language == button.language {
                    SELECTED
                } else {
                    HOVERED
                };
            }
            Interaction::Pressed => {
                background.0 = SELECTED;
                config.language = button.language;
                changed.0 = true;
                writer.send(SEEvent::new(SE::Click));
            }
        }
    }
}

fn toggle_language_with_key(
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<GameConfig>,
    mut writer: EventWriter<SEEvent>,
) {
    let delta: i32 = if keys.just_pressed(KeyCode::KeyW) {
        -1
    } else if keys.just_pressed(KeyCode::KeyS) {
        1
    } else {
        0
    };

    if delta != 0 {
        let current = Languages::iter().position(|l| l == config.language);
        let next =
            (Languages::iter().count() as i32 + current.unwrap_or(0) as i32 + delta) as usize;
        config.language = Languages::iter().cycle().skip(next).next().unwrap();
        writer.send(SEEvent::new(SE::Click));
    }
}

fn update_language_button_background(
    mut query: Query<(&mut BackgroundColor, &LanguageButton)>,
    config: Res<GameConfig>,
) {
    if config.is_changed() {
        for (mut background, button) in &mut query.iter_mut() {
            background.0 = if config.language == button.language {
                SELECTED
            } else {
                UNSELECTED
            }
        }
    }
}

fn start_game(
    buttons: Res<ButtonInput<MouseButton>>,
    mut writer: EventWriter<Events>,
    changed: Res<LanguageChanged>,
) {
    if !changed.0 && buttons.any_just_pressed(vec![MouseButton::Left, MouseButton::Right]) {
        writer.send(Events::Start);
    }
}

fn read_events(
    mut query: Query<&mut Visibility, With<OnPress>>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut writer: EventWriter<SEEvent>,
    mut reader: EventReader<Events>,
    mut next_bgm: ResMut<NextBGM>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
) {
    for event in reader.read() {
        match event {
            Events::Start => {
                for mut visibility in &mut query {
                    *visibility = Visibility::Hidden;
                }
                menu_next_state.set(MainMenuPhase::Paused);
                writer.send(SEEvent::new(SE::Click));
                overlay_event_writer.send(OverlayEvent::Close(GameState::InGame));
                *next_bgm = NextBGM(None);
            }
        }
    }
}

fn witch_animation(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Node, With<WitchAnimation>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left = Val::Px(750.0 + (frame_count.0 as f32 * 0.007).sin() * 100.0);
        style.top = Val::Px(0.0 + (frame_count.0 as f32 * 0.02).cos() * 50.0);
    }
}

fn cloud_animation(frame_count: Res<FrameCount>, mut query: Query<(&mut Node, &CloudAnimation)>) {
    for (mut style, animation) in &mut query.iter_mut() {
        style.left = Val::Px(
            animation.left * SCALE
                - (frame_count.0 as f32 * animation.speed).fract() * 1024.0 * SCALE
                + animation.offset,
        );
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LanguageChanged>();
        app.add_event::<Events>();
        app.add_systems(OnEnter(GameState::MainMenu), setup);
        app.add_systems(
            Update,
            (
                read_events,
                witch_animation,
                cloud_animation,
                (
                    update_language_button_background,
                    toggle_language,
                    toggle_language_with_key,
                    start_game,
                )
                    .chain(),
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
