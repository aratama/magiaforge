use super::{serialize::PlayerData, set::GameSet, states::GameState};
use crate::game::player::*;
use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;

#[derive(Component)]
pub struct CameraScaleFactor(f32);

static BLIGHTNESS_IN_GAME: f32 = 0.05;

pub fn setup_camera(mut commands: Commands) {
    // println!("setup_camera");
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 2.0_f32.powf(initial_scale_factor);

    commands.spawn((
        camera_bundle,
        CameraScaleFactor(initial_scale_factor),
        // カメラにAmbiendLight2dを追加すると、画面全体が暗くなり、
        // 光が当たっていない部分の明るさを設定できます
        AmbientLight2d {
            // color: Color::hsl(250.0, 0.8, 0.5),
            brightness: BLIGHTNESS_IN_GAME,
            // brightness: 1.,
            ..default()
        },
    ));
}

fn on_enter_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_data: Res<PlayerData>,
) {
    // println!("on_enter_camera");
    if let Ok(mut camera) = camera_query.get_single_mut() {
        camera.translation.x = player_data.x;
        camera.translation.y = player_data.y;
    }
}

fn update_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<
        (
            &mut Transform,
            &mut OrthographicProjection,
            &mut CameraScaleFactor,
        ),
        (With<Camera2d>, Without<Player>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player = player_query.single();
    if let Ok((mut camera, mut ortho, mut scale_factor)) = camera_query.get_single_mut() {
        let t = 0.001;
        camera.translation.x += (player.translation.x - camera.translation.x) * t;
        camera.translation.y += (player.translation.y - camera.translation.y) * t;

        if keys.just_pressed(KeyCode::KeyR) {
            *scale_factor = CameraScaleFactor(scale_factor.0 - 1.0);
        }
        if keys.just_pressed(KeyCode::KeyF) {
            *scale_factor = CameraScaleFactor(scale_factor.0 + 1.0);
        }
        let s = ortho.scale.log2();
        ortho.scale = (2.0_f32).powf(s + (scale_factor.0 - s) * 0.2);
    }
}

fn update_camera_brightness(
    mut camera_query: Query<&mut AmbientLight2d, With<Camera2d>>,
    state: Res<State<GameState>>,
) {
    if let Ok(mut light) = camera_query.get_single_mut() {
        light.brightness = match state.get() {
            GameState::InGame => BLIGHTNESS_IN_GAME,
            _ => 0.0,
        };
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), on_enter_camera);
        app.add_systems(
            FixedUpdate,
            update_camera
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );

        app.add_systems(
            FixedUpdate,
            update_camera_brightness
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
