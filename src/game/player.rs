use super::asset::GameAssets;
use super::bullet::add_bullet;
use super::constant::*;
use super::gamepad::{get_direction, get_fire_trigger, MyGamepad};
use super::serialize::*;
use super::states::GameState;
use bevy::prelude::*;
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
    pub latest_damage: i32,

    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,
}

fn setup_player(mut commands: Commands, player_data: Res<PlayerData>, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("player"),
        StateScoped(GameState::InGame),
        Player {
            cooltime: 0,
            life: 250,
            max_life: 250,
            latest_damage: 0,
            pointer: Vec2::ZERO,
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
    mut camera_query: Query<&mut Transform, (With<Camera>, With<Camera2d>, Without<Player>)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    buttons: Res<ButtonInput<MouseButton>>,

    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
) {
    let force = 50000.0;

    let direction = get_direction(keys, axes, &my_gamepad);

    let (mut player, mut player_transform, mut player_force, _, mut player_sprite) =
        player_query.single_mut();

    player_transform.translation.z =
        ENTITY_LAYER_Z - player_transform.translation.y * Z_ORDER_SCALE;
    player_force.force = direction * force;

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.translation.x +=
            (player_transform.translation.x - camera_transform.translation.x) * CAMERA_SPEED;
        camera_transform.translation.y +=
            (player_transform.translation.y - camera_transform.translation.y) * CAMERA_SPEED;

        // プレイヤーの向き
        let angle = player.pointer.to_angle();

        // println!("angle: {}", angle);
        if angle < -PI * 0.5 || PI * 0.5 < angle {
            player_sprite.flip_x = true;
        } else {
            player_sprite.flip_x = false;
        }

        // 魔法の発射
        if get_fire_trigger(buttons, gamepad_buttons, &my_gamepad) && player.cooltime == 0 {
            // 魔法の拡散
            let scattering = 0.3;

            // 魔法弾の速度
            // pixels_per_meter が 100.0 に設定されているので、
            // 200は1フレームに2ピクセル移動する速度です
            let speed = 200.0;

            // 次の魔法を発射するまでの待機フレーム数
            let cooltime = 16;

            // 一度に発射する弾丸の数
            let bullets_unit = 1;

            let normalized = player.pointer.normalize();

            for _ in 0..bullets_unit {
                let angle_with_random = angle + (random::<f32>() - 0.5) * scattering;
                let direction = Vec2::from_angle(angle_with_random);
                add_bullet(
                    &mut commands,
                    assets.asset.clone(),
                    player_transform.translation.truncate() + normalized * 10.0,
                    direction * speed,
                );
            }

            player.cooltime = cooltime;
        } else {
            player.cooltime = (player.cooltime - 1).max(0);
        }
    }
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
