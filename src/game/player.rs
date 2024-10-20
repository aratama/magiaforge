use super::asset::GameAssets;
use super::bullet::add_bullet;
use super::constant::*;
use super::serialize::*;
use super::states::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;
use rand::random;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Player {
    /// 次の魔法を発射できるまでのクールタイム
    pub cooltime: i32,
    pub life: i32,
    pub max_life: i32,
}

fn setup_player(mut commands: Commands, player_data: Res<PlayerData>, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("player"),
        StateScoped(GameState::InGame),
        Player {
            cooltime: 0,
            life: 220,
            max_life: 250,
        },
        AsepriteAnimationBundle {
            aseprite: assets.player.clone(),
            transform: Transform::from_xyz(player_data.x, player_data.y, 1.0),
            animation: Animation::default().with_tag("idle").with_speed(0.2),
            sprite: Sprite {
                // flip_x: true,
                // ここもanchorは効かないことに注意。Aseprite側のpivotで設定
                // anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, 1.0)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(5.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 6.0,
            angular_damping: 1.0,
        },
        ExternalForce::default(),
        ExternalImpulse::default(),
        CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP | WALL_GROUP),
        PointLight2d {
            radius: 100.0,
            intensity: 3.0,
            falloff: 10.0,
            ..default()
        },
    ));
}

fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut Player,
            &mut Transform,
            &mut ExternalForce,
            &GlobalTransform,
            &mut Sprite,
        ),
        Without<Camera2d>,
    >,
    mut camera_query: Query<
        (&Camera, &mut Transform, &GlobalTransform),
        (With<Camera2d>, Without<Player>),
    >,
    q_window: Query<&Window, With<PrimaryWindow>>,
    commands: Commands,
    assets: Res<GameAssets>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let force = 50000.0;

    let direction = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero();

    let (mut player, mut player_transform, mut player_force, _, mut player_sprite) =
        player_query.single_mut();

    player_transform.translation.z =
        ENTITY_LAYER_Z - player_transform.translation.y * Z_ORDER_SCALE;
    player_force.force = direction * force;

    if let Ok((camera, mut camera_transform, camera_global_transform)) =
        camera_query.get_single_mut()
    {
        camera_transform.translation.x +=
            (player_transform.translation.x - camera_transform.translation.x) * CAMERA_SPEED;
        camera_transform.translation.y +=
            (player_transform.translation.y - camera_transform.translation.y) * CAMERA_SPEED;

        // プレイヤーの向き
        if let Ok(window) = q_window.get_single() {
            let cursor = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor))
                .map(|ray| ray.origin.truncate())
                .unwrap_or(Vec2::ZERO);

            let ray = cursor - player_transform.translation.truncate();
            let angle = ray.y.atan2(ray.x);
            // println!("angle: {}", angle);
            if angle < -PI * 0.5 || PI * 0.5 < angle {
                player_sprite.flip_x = true;
            } else {
                player_sprite.flip_x = false;
            }

            // 魔法の発射
            if buttons.pressed(MouseButton::Left) && player.cooltime == 0 {
                // 魔法の拡散
                let scattering = 0.3;

                // 魔法弾の速度
                // pixels_per_meter が 100.0 に設定されているので、
                // 200は1フレームに2ピクセル移動する速度です
                let speed = 200.0;

                // 次の魔法を発射するまでの待機フレーム数
                let cooltime = 10;

                let normalized = ray.normalize_or_zero();
                let angle = normalized.y.atan2(normalized.x) + (random::<f32>() - 0.5) * scattering;
                let direction = Vec2::new(angle.cos(), angle.sin());

                add_bullet(
                    commands,
                    assets.asset.clone(),
                    player_transform.translation.truncate() + normalized * 10.0,
                    direction * speed,
                );

                player.cooltime = cooltime;
            } else {
                player.cooltime = (player.cooltime - 1).max(0);
            }
        }
    }
}

fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_player);
        app.add_systems(
            FixedUpdate,
            update_player.run_if(in_state(GameState::InGame)),
        );
    }
}
