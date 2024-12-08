use crate::audio::NextBGM;
use crate::command::GameCommand;
use crate::constant::HUD_Z_INDEX;
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
struct CloudAnimation0;

#[derive(Component)]
struct CloudAnimation1;

#[derive(Component)]
struct CloudAnimation2;

#[derive(Component)]
struct CloudAnimation3;

#[derive(Event, PartialEq, Eq, Debug, Clone, Copy)]
enum Events {
    Start,
}

fn setup_main_menu(mut commands: Commands, assets: Res<GameAssets>, mut next_bgm: ResMut<NextBGM>) {
    *next_bgm = NextBGM(Some(assets.boubaku.clone()));

    commands.spawn((
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

    spawn_cloud(&mut commands, &assets.title_cloud, CloudAnimation0, -950);
    spawn_cloud(&mut commands, &assets.title_cloud, CloudAnimation1, -950);
    spawn_cloud(&mut commands, &assets.title_cloud2, CloudAnimation2, -900);
    spawn_cloud(&mut commands, &assets.title_cloud2, CloudAnimation3, -900);

    commands.spawn((
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
            animation: "default".into(),
        },
    ));

    commands.spawn((
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

    commands.spawn((
        StateScoped(GameState::MainMenu),
        GlobalZIndex(-700),
        Node {
            left: Val::Px(520.0),
            top: Val::Px(640.0),
            width: Val::Px(64.0 * SCALE),
            height: Val::Px(16.0 * SCALE),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "click_to_start".into(),
        },
    ));

    commands.spawn((
        StateScoped(GameState::MainMenu),
        Name::new("Git Version"),
        GlobalZIndex(HUD_Z_INDEX),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(700.0),
            ..default()
        },
        Text::new(format!("Version: {}", git_version!())),
        TextColor::from(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        TextFont {
            font_size: 12.0,
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

fn on_click(buttons: Res<ButtonInput<MouseButton>>, mut writer: EventWriter<Events>) {
    if buttons.any_just_pressed(vec![MouseButton::Left, MouseButton::Right]) {
        writer.send(Events::Start);
    }
}

fn read_events(
    mut query: Query<&mut Visibility, With<OnPress>>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut writer: EventWriter<GameCommand>,
    mut reader: EventReader<Events>,
    mut next_bgm: ResMut<NextBGM>,
) {
    for event in reader.read() {
        match event {
            Events::Start => {
                for mut visibility in &mut query {
                    *visibility = Visibility::Hidden;
                }
                menu_next_state.set(MainMenuPhase::Paused);
                writer.send(GameCommand::SEClick(None));
                writer.send(GameCommand::StateInGame);
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

fn cloud_animation0(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Node, With<CloudAnimation0>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left = Val::Px(0.0 - (frame_count.0 as f32 * 0.0003).fract() * 1024.0 * SCALE);
    }
}

fn cloud_animation1(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Node, With<CloudAnimation1>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left =
            Val::Px(1024.0 * SCALE - (frame_count.0 as f32 * 0.0003).fract() * 1024.0 * SCALE);
    }
}

fn cloud_animation2(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Node, With<CloudAnimation2>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left =
            Val::Px(0.0 - (frame_count.0 as f32 * 0.0001).fract() * 1024.0 * SCALE - 600.0);
    }
}

fn cloud_animation3(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Node, With<CloudAnimation3>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left = Val::Px(
            1024.0 * SCALE - (frame_count.0 as f32 * 0.0001).fract() * 1024.0 * SCALE - 600.0,
        );
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Events>();
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.add_systems(
            Update,
            (
                on_click,
                read_events,
                witch_animation,
                cloud_animation0,
                cloud_animation1,
                cloud_animation2,
                cloud_animation3,
            )
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}
