use crate::game::player::*;
use bevy::prelude::*;

use super::{serialize::PlayerData, set::GameSet, states::GameState};

#[derive(Component)]
pub struct CameraScaleFactor(f32);

fn setup_camera(mut commands: Commands) {
    // println!("setup_camera");
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 2.0_f32.powf(initial_scale_factor);

    commands.spawn((camera_bundle, CameraScaleFactor(initial_scale_factor)));
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
        ortho.scale = (2.0_f32).powf(s + (scale_factor.0 - s) * 0.1);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            setup_camera, // メインメニューなどのシーンでもカメラは必要なため、run_ifでの制御は行わない
                          // .run_if(in_state(GameState::InGame))
                          // .in_set(GameSet)
        );
        // TODO:GameState::InGameをデフォルトにして起動したとき、 StartUp より OnEnter のほうが先に実行されてしまう？
        // GameState::MainMenuだとStartUpが先に実行される？
        // https://github.com/bevyengine/bevy/issues/14740
        app.add_systems(OnEnter(GameState::InGame), on_enter_camera);
        app.add_systems(
            FixedUpdate,
            update_camera
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
