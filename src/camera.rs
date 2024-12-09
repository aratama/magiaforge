use crate::constant::CAMERA_SPEED;
use crate::entity::actor::Actor;
use crate::{controller::player::Player, set::GameSet, states::GameState};
use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Component)]
pub struct CameraScaleFactor(f32);

static BLIGHTNESS_IN_GAME: f32 = 0.03;

fn setup_camera(mut commands: Commands) {
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let camera = Camera2d::default();
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 2.0_f32.powf(initial_scale_factor);

    commands.spawn((
        camera,
        projection,
        CameraScaleFactor(initial_scale_factor),
        // カメラにAmbiendLight2dを追加すると、画面全体が暗くなり、
        // 光が当たっていない部分の明るさを設定できます
        AmbientLight2d {
            color: Color::WHITE,
            brightness: BLIGHTNESS_IN_GAME,
        },
    ));
}

fn update_camera(
    player_query: Query<(&Transform, &Actor), With<Player>>,
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
    if let Ok((player, actor)) = player_query.get_single() {
        if let Ok((mut camera, mut ortho, mut scale_factor)) = camera_query.get_single_mut() {
            // ポインターのある方向にカメラをずらして遠方を見やすくする係数
            // カメラがブレるように感じて酔いやすい？
            let point_by_mouse_factor = 0.0; // 0.2;

            let vrp = player.translation.truncate()
                + actor.pointer.normalize_or_zero()
                    * (actor.pointer.length() * point_by_mouse_factor).min(50.0);

            camera.translation.x += (vrp.x - camera.translation.x) * CAMERA_SPEED;
            camera.translation.y += (vrp.y - camera.translation.y) * CAMERA_SPEED;

            if keys.just_pressed(KeyCode::KeyR) {
                *scale_factor = CameraScaleFactor((scale_factor.0 - 0.5).max(-2.0));
            }
            if keys.just_pressed(KeyCode::KeyF) {
                *scale_factor = CameraScaleFactor((scale_factor.0 + 0.5).min(1.0));
            }
            let s = ortho.scale.log2();
            ortho.scale = (2.0_f32).powf(s + (scale_factor.0 - s) * 0.2);
        }
    }
}

fn update_camera_brightness(
    mut camera_query: Query<&mut AmbientLight2d, With<Camera2d>>,
    state: Res<State<GameState>>,
) {
    if let Ok(mut light) = camera_query.get_single_mut() {
        light.brightness = match state.get() {
            GameState::InGame => BLIGHTNESS_IN_GAME,
            _ => 1.0,
        };
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_camera
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );

        app.add_systems(
            FixedUpdate,
            update_camera_brightness.before(PhysicsSet::SyncBackend),
        );

        app.add_systems(OnEnter(GameState::Setup), setup_camera);
    }
}
