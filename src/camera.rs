use bevy::prelude::*;

#[derive(Component)]
pub struct CameraScaleFactor(f32);

#[cfg(feature = "debug")]
static BLIGHTNESS_IN_GAME: f32 = 1.0;

#[cfg(not(feature = "debug"))]
static BLIGHTNESS_IN_GAME: f32 = 0.05;

pub fn setup_camera(mut commands: Commands) {
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 2.0_f32.powf(initial_scale_factor);

    commands.spawn((camera_bundle,));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {}
}
