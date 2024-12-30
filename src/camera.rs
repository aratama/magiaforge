use crate::constant::CAMERA_SPEED;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::explosion::ExplosionPointLight;
use crate::entity::explosion::EXPLOSION_COUNT;
use crate::page::in_game::GameLevel;
use crate::page::in_game::Interlevel;
use crate::physics::InGameTime;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Component)]
pub struct GameCamera {
    pub x: f32,
    pub y: f32,
    pub scale_factor: f32,
    pub vibration: f32,
    pub target: Option<Entity>,
}

static BLIGHTNESS_IN_GAME: f32 = 0.2;

pub fn setup_camera(commands: &mut Commands, position: Vec2) {
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let camera = Camera2d::default();
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 2.0_f32.powf(initial_scale_factor);

    commands.spawn((
        Name::new("default camera"),
        StateScoped(GameState::InGame),
        camera,
        projection,
        // 1ピクセルのアーティファクトが出ることがありますが、Msaa::Offで直るっぽい
        // 特にショップの扉でアーティファクトが出てました
        // https://github.com/bevyengine/bevy/issues/16918#issuecomment-2557851700
        Msaa::Off,
        GameCamera {
            x: position.x,
            y: position.y,
            scale_factor: initial_scale_factor,
            vibration: 0.0,
            target: None,
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        // カメラにAmbiendLight2dを追加すると、画面全体が暗くなり、
        // 光が当たっていない部分の明るさを設定できます
        AmbientLight2d {
            color: Color::WHITE,
            brightness: BLIGHTNESS_IN_GAME,
        },
    ));
}

fn update_camera_position(
    player_query: Query<(&Transform, &Actor), With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection, &mut GameCamera),
        (With<Camera2d>, Without<Player>),
    >,
    frame_count: Res<FrameCount>,
    target_query: Query<&GlobalTransform, (Without<Player>, Without<Camera2d>)>,
    in_game_time: Res<InGameTime>,
) {
    if !in_game_time.active {
        return;
    }
    if let Ok((player, actor)) = player_query.get_single() {
        if let Ok((mut camera_transform, mut ortho, mut game_camera)) =
            camera_query.get_single_mut()
        {
            // ポインターのある方向にカメラをずらして遠方を見やすくする係数
            // カメラがブレるように感じて酔いやすい？
            let point_by_mouse_factor = 0.0; // 0.2;

            let p = player.translation.truncate()
                + actor.pointer.normalize_or_zero()
                    * (actor.pointer.length() * point_by_mouse_factor).min(50.0);

            let vrp: Vec2 = match game_camera.target {
                Some(target) => {
                    if let Ok(global_transform) = target_query.get(target) {
                        global_transform.translation().truncate()
                    } else {
                        p
                    }
                }
                _ => p,
            };

            // 目的の値を計算
            game_camera.x = vrp.x;
            game_camera.y = vrp.y;
            game_camera.vibration = (game_camera.vibration - 0.5).max(0.0);
            game_camera.scale_factor = match game_camera.target {
                Some(_) => -2.0,
                _ => actor.get_total_scale_factor(),
            };

            // 実際の値を設定
            let vibration = (frame_count.0 as f32 * 5.0).sin() * game_camera.vibration;
            camera_transform.translation.x +=
                (vrp.x - camera_transform.translation.x) * CAMERA_SPEED;
            camera_transform.translation.y +=
                (vrp.y - camera_transform.translation.y) * CAMERA_SPEED + vibration;

            let s = ortho.scale.log2();
            ortho.scale = (2.0_f32).powf(s + (game_camera.scale_factor - s) * 0.2);
        }
    }
}

fn update_camera_brightness(
    mut camera_query: Query<&mut AmbientLight2d, With<Camera2d>>,
    state: Res<State<GameState>>,
    level: Res<Interlevel>,
    explosion_query: Query<&ExplosionPointLight>,
    in_game_time: Res<InGameTime>,
) {
    if !in_game_time.active {
        return;
    }
    if let Ok(mut light) = camera_query.get_single_mut() {
        // 爆発エフェクトを考慮しない、レベルごとの画面の明るさ
        let brightness = match state.get() {
            GameState::InGame => match level.level {
                Some(GameLevel::Level(2)) => 0.02,
                _ => BLIGHTNESS_IN_GAME,
            },
            _ => 1.0,
        };

        let max_explosion = explosion_query
            .iter()
            .map(|e| e.lifetime)
            .max()
            .unwrap_or(0);

        if max_explosion > 0 {
            light.brightness = brightness * max_explosion as f32 / EXPLOSION_COUNT as f32;
        } else {
            light.brightness = brightness;
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_camera_position, update_camera_brightness)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
