use crate::command::GameCommand;
use crate::constant::HUD_Z_INDEX;
use crate::ui::on_press::OnPress;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AsepriteAnimationUiBundle, AsepriteSliceUiBundle};
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

fn setup_main_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut writer: EventWriter<GameCommand>,
) {
    writer.send(GameCommand::BGMBoubaku);

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

    spawn_cloud(&mut commands, &assets, CloudAnimation0, -950);
    spawn_cloud(&mut commands, &assets, CloudAnimation1, -950);
    spawn_cloud(&mut commands, &assets, CloudAnimation2, -900);
    spawn_cloud(&mut commands, &assets, CloudAnimation3, -900);

    commands.spawn((
        WitchAnimation,
        StateScoped(GameState::MainMenu),
        ImageBundle {
            z_index: ZIndex::Global(-800),
            style: Style {
                left: Val::Px(800.0),
                top: Val::Px(0.0),
                width: Val::Px(96.0 * SCALE),
                height: Val::Px(96.0 * SCALE),
                ..default()
            },
            ..default()
        },
        AsepriteAnimationUiBundle {
            aseprite: assets.title_witch.clone(),
            animation: "default".into(),
            ..default()
        },
    ));

    commands.spawn((
        StateScoped(GameState::MainMenu),
        ImageBundle {
            z_index: ZIndex::Global(-700),
            style: Style {
                left: Val::Px(400.0),
                top: Val::Px(300.0),
                width: Val::Px(128.0 * SCALE),
                height: Val::Px(48.0 * SCALE),
                ..default()
            },
            ..default()
        },
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "title_logo".into(),
            ..default()
        },
    ));

    commands.spawn((
        StateScoped(GameState::MainMenu),
        ImageBundle {
            z_index: ZIndex::Global(-700),
            style: Style {
                left: Val::Px(540.0),
                top: Val::Px(640.0),
                width: Val::Px(64.0 * SCALE),
                height: Val::Px(16.0 * SCALE),
                ..default()
            },
            ..default()
        },
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "click_to_start".into(),
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

fn spawn_cloud<T: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    marker: T,
    z_index: i32,
) {
    commands.spawn((
        marker,
        StateScoped(GameState::MainMenu),
        ImageBundle {
            z_index: ZIndex::Global(z_index),
            style: Style {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(1024.0 * SCALE),
                height: Val::Px(180.0 * SCALE),
                ..default()
            },
            ..default()
        },
        AsepriteAnimationUiBundle {
            aseprite: assets.title_cloud.clone(),
            animation: "default".into(),
            ..default()
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
) {
    for event in reader.read() {
        match event {
            Events::Start => {
                for mut visibility in &mut query {
                    *visibility = Visibility::Hidden;
                }
                menu_next_state.set(MainMenuPhase::Paused);
                writer.send(GameCommand::SEKettei(None));
                writer.send(GameCommand::StateInGame);
                writer.send(GameCommand::BGMNone);
            }
        }
    }
}

fn witch_animation(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Style, With<WitchAnimation>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left = Val::Px(800.0 + (frame_count.0 as f32 * 0.007).sin() * 40.0);
        style.top = Val::Px(0.0 + (frame_count.0 as f32 * 0.02).cos() * 10.0);
    }
}

fn cloud_animation0(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Style, With<CloudAnimation0>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left = Val::Px(0.0 - (frame_count.0 as f32 * 0.0003).fract() * 1024.0 * SCALE);
    }
}

fn cloud_animation1(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Style, With<CloudAnimation1>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left =
            Val::Px(1024.0 * SCALE - (frame_count.0 as f32 * 0.0003).fract() * 1024.0 * SCALE);
    }
}

fn cloud_animation2(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Style, With<CloudAnimation2>>,
) {
    for mut style in &mut query.iter_mut() {
        style.left =
            Val::Px(0.0 - (frame_count.0 as f32 * 0.0001).fract() * 1024.0 * SCALE - 600.0);
    }
}

fn cloud_animation3(
    frame_count: Res<FrameCount>,
    mut query: Query<&mut Style, With<CloudAnimation3>>,
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
