use crate::audio::NextBGM;
use crate::config::GameConfig;
use crate::constant::HUD_Z_INDEX;
use crate::hud::overlay::OverlayEvent;
use crate::language::Languages;
use crate::level::CurrentLevel;
use crate::se::{SEEvent, SE};
use crate::ui::on_press::OnPress;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use git_version::git_version;

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
struct LanguageButton;

#[derive(Component)]
struct ClickToStart;

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut next_bgm: ResMut<NextBGM>,
    mut current: ResMut<CurrentLevel>,
) {
    commands.spawn((Camera2d::default(), StateScoped(GameState::MainMenu)));

    *next_bgm = NextBGM(Some(assets.boubaku.clone()));
    *current = CurrentLevel::default();

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
            ClickToStart,
            Text::new(""),
            TextColor::from(Color::WHITE),
            TextFont {
                font_size: 16.0,
                font: assets.dotgothic.clone(),
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
            font: assets.dotgothic.clone(),
            font_size: 12.0,
            ..default()
        },
    ));

    commands
        .spawn((
            Name::new("language_button"),
            LanguageButton,
            StateScoped(GameState::MainMenu),
            GlobalZIndex(HUD_Z_INDEX),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(40.0),
                bottom: Val::Px(40.0),
                padding: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(8.0), Val::Px(8.0)),
                ..default()
            },
            Button,
            BackgroundColor::from(Color::hsva(0.0, 0.0, 1.0, 0.3)),
        ))
        .with_child((
            Text::new("English / 日本語"),
            TextColor::from(Color::hsl(0.0, 0.0, 0.0)),
            TextFont {
                font_size: 16.0,
                font: assets.dotgothic.clone(),
                ..default()
            },
        ));
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

fn toggle_language(
    mut query: Query<
        (&mut BackgroundColor, &Interaction),
        (With<LanguageButton>, Changed<Interaction>),
    >,
    mut config: ResMut<GameConfig>,
    mut changed: ResMut<LanguageChanged>,
) {
    changed.0 = false;

    for (mut background, interaction) in &mut query.iter_mut() {
        match interaction {
            Interaction::None => {
                background.0 = Color::hsva(0.0, 0.0, 1.0, 0.3);
            }
            Interaction::Hovered => {
                background.0 = Color::hsva(0.0, 0.0, 1.0, 0.8);
            }
            Interaction::Pressed => {
                background.0 = Color::WHITE;
                config.language = match config.language {
                    Languages::En => Languages::Ja,
                    Languages::Ja => Languages::En,
                };
                changed.0 = true;
            }
        }
    }
}

fn update_click_to_start_text(
    mut query: Query<&mut Text, With<ClickToStart>>,
    config: Res<GameConfig>,
) {
    if config.is_changed() {
        for mut text in &mut query.iter_mut() {
            text.0 = (match config.language {
                Languages::Ja => "クリックでスタート",
                Languages::En => "Click to Start",
            })
            .to_string();
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
                (toggle_language, start_game).chain(),
                update_click_to_start_text,
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
