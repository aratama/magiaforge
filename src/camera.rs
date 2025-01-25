use crate::actor::Actor;
use crate::constant::CAMERA_SPEED;
use crate::controller::player::Player;
use crate::controller::player::PlayerServant;
use crate::entity::explosion::ExplosionPointLight;
use crate::entity::explosion::EXPLOSION_COUNT;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;

#[derive(Component)]
pub struct GameCamera {
    pub x: f32,
    pub y: f32,
    pub scale_factor: f32,
    pub vibration: f32,
    pub target: Option<Entity>,
}

impl GameCamera {
    pub fn vibrate(&mut self, camera_transform: &Transform, position: Vec2, amount: f32) {
        let distance = camera_transform.translation.truncate().distance(position);
        let max_range = 320.0; // 振動が起きる最大距離
        self.vibration = (amount * (max_range - distance).max(0.0) / max_range).min(10.0);
    }
}

static BLIGHTNESS_IN_GAME: f32 = 0.2;

pub fn setup_camera(commands: &mut Commands, position: Vec2) {
    let initial_scale_factor = -1.0;

    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let camera = Camera2d::default();
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 2.0_f32.powf(initial_scale_factor);

    // let gap = 10.0;
    // let listener = SpatialListener::new(gap);

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
        // listener.clone(),
    ));
}

fn update_camera_position(
    player_query: Query<(&Transform, &Actor), With<Player>>,
    servant_query: Query<&Transform, (With<PlayerServant>, Without<Player>)>,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection, &mut GameCamera),
        (With<Camera2d>, Without<Player>, Without<PlayerServant>),
    >,
    frame_count: Res<FrameCount>,
    target_query: Query<&GlobalTransform, (Without<Player>, Without<Camera2d>)>,
) {
    if let Ok((player, actor)) = player_query.get_single() {
        if let Ok((mut camera_transform, mut ortho, mut game_camera)) =
            camera_query.get_single_mut()
        {
            let vrp: Vec2 = match game_camera.target {
                Some(target) => {
                    if let Ok(global_transform) = target_query.get(target) {
                        global_transform.translation().truncate()
                    } else if let Ok(servant) = servant_query.get_single() {
                        servant.translation.truncate()
                    } else {
                        player.translation.truncate()
                    }
                }
                _ => {
                    if let Some(servant) = servant_query.iter().next() {
                        servant.translation.truncate()
                    } else {
                        player.translation.truncate()
                    }
                }
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
    registry: Registry,
    mut camera_query: Query<&mut AmbientLight2d, With<Camera2d>>,
    level: Res<LevelSetup>,
    explosion_query: Query<&ExplosionPointLight>,
) {
    if let Ok(mut light) = camera_query.get_single_mut() {
        let Some(level) = &level.level else {
            return;
        };
        let props = registry.get_level(&level);
        let brightness = props.brightness;

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
            (update_camera_position, update_camera_brightness).in_set(FixedUpdateGameActiveSet),
        );
    }
}
