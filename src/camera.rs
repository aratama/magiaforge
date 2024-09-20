use crate::player::*;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;

    commands.spawn(camera_bundle);
}

pub fn update_camera(
    player_query: Query<&Transform, (With<Person>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Person>)>,
) {
    let player = player_query.single();
    let mut camera = camera_query.single_mut();
    camera.translation.x += (player.translation.x - camera.translation.x) * 0.1;
    camera.translation.y += (player.translation.y - camera.translation.y) * 0.1;
}
