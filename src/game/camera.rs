use crate::game::player::*;
use bevy::prelude::*;

use super::{set::GameSet, states::GameState};

#[derive(Component)]
pub struct CameraScaleFactor(f32);

fn setup_camera(mut commands: Commands, initial_camera_position: Vec2) {
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 2.0_f32.powf(initial_scale_factor);
    camera_bundle.transform.translation.x = initial_camera_position.x;
    camera_bundle.transform.translation.y = initial_camera_position.y;

    commands.spawn((camera_bundle, CameraScaleFactor(initial_scale_factor)));
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
        ortho.scale = (2.0_f32).powf(s + (scale_factor.0 - s) * 0.1);
    }
}

pub struct CameraPlugin {
    pub initial_camera_position: Vec2,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let pos = self.initial_camera_position;
        app.add_systems(
            Startup,
            (move |commands: Commands| {
                setup_camera(commands, pos);
            })
            .run_if(in_state(GameState::InGame))
            .in_set(GameSet),
        );
        app.add_systems(
            Update,
            update_camera
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
