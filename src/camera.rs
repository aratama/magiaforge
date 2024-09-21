use crate::{player::*, PlayerData};
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraScaleFactor(f32);

pub fn setup_camera(mut commands: Commands, player_data: PlayerData) {
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 2.0_f32.powf(initial_scale_factor);
    camera_bundle.transform.translation.x = player_data.x;
    camera_bundle.transform.translation.y = player_data.y;

    commands.spawn((camera_bundle, CameraScaleFactor(initial_scale_factor)));
}

pub fn update_camera(
    player_query: Query<&Transform, (With<Person>, Without<Camera2d>)>,
    mut camera_query: Query<
        (
            &mut Transform,
            &mut OrthographicProjection,
            &mut CameraScaleFactor,
        ),
        (With<Camera2d>, Without<Person>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player = player_query.single();
    let (mut camera, mut ortho, mut scale_factor) = camera_query.single_mut();
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
    ortho.scale = (2.0_f32).powf(s + (scale_factor.0 - s) * 0.1);
}
