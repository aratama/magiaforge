use super::audio::play_se;
use super::enemy::Enemy;
use super::states::GameState;
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteSliceBundle;
use bevy_light_2d::light::PointLight2d;
use bevy_particle_systems::{
    ColorOverTime, JitteredValue, ParticleBurst, ParticleSystem, ParticleSystemBundle, Playing,
};
use bevy_rapier2d::prelude::*;
// use std::path::Path;

const ASEPRITE_PATH: &str = "asset.aseprite";

const SLICE_NAME: &str = "bullet";

static BULLET_Z: f32 = 10.0;

#[derive(Component, Reflect)]
pub struct Bullet {
    life: u32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    name: Name,
    bullet: Bullet,
    transform: Transform,
}

pub fn add_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: Vec2,
    velocity: Vec2,
) {
    // TODO:
    // 現状ではアセットの埋め込みはしていません
    // 埋め込みができない理由は、world.rs のコメントに記載しています
    //
    // // https://bevyengine.org/examples/assets/embedded-asset/
    // アセットのパスは URL のような形式になっており、
    // プロトコルは embedded、それ以下のパスは crate 名とアセットのパスになります
    // embedded_asset 側の設定で、パスの game は省略されます
    // 実際のパスは Window では例えば、 embedded://my_bevy_game\asset/asset.aseprite になります
    // let path = Path::new(CRATE_NAME).join("asset/asset.aseprite");
    // let asset_path = AssetPath::from_path(&path).with_source(AssetSourceId::from("embedded"));

    commands.spawn((
        Name::new("bullet"),
        StateScoped(GameState::InGame),
        Bullet { life: 120 },
        AsepriteSliceBundle {
            // aseprite: asset_server.load(asset_path),
            aseprite: asset_server.load(ASEPRITE_PATH),
            slice: SLICE_NAME.into(),
            transform: Transform::from_xyz(position.x, position.y, BULLET_Z)
                * Transform::from_rotation(Quat::from_rotation_z(velocity.to_angle())), // .looking_to(velocity.extend(BULLET_Z), Vec3::Z)
            ..default()
        },
        Velocity {
            linvel: velocity,
            angvel: 0.0,
        },
        KinematicCharacterController::default(),
        RigidBody::KinematicVelocityBased,
        Collider::ball(5.0),
        GravityScale(0.0),
        Sensor,
        // https://rapier.rs/docs/user_guides/bevy_plugin/colliders#active-collision-types
        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
        Sleeping::disabled(),
        Ccd::enabled(),
        PointLight2d {
            radius: 50.0,
            intensity: 1.0,
            falloff: 10.0,
            color: Color::hsl(245.0, 1.0, 0.6),
            ..default()
        },
    ));
}

pub fn update_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bullet, &Transform, &Velocity)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemies: Query<(Entity, &mut Enemy)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut bullet, _, _) in query.iter_mut() {
        bullet.life -= 1;
        if bullet.life <= 0 {
            commands.entity(entity).despawn();
        }
    }
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                if let Ok((_, _, bullet_transform, bullet_velocity)) = query.get(*a) {
                    process_bullet_event(
                        &mut commands,
                        &asset_server,
                        &a,
                        &bullet_transform,
                        &bullet_velocity,
                        enemies.get_mut(*b),
                    );
                }
                if let Ok((_, _, bullet_transform, bullet_velocity)) = query.get(*b) {
                    process_bullet_event(
                        &mut commands,
                        &asset_server,
                        &b,
                        &bullet_transform,
                        &bullet_velocity,
                        enemies.get_mut(*a),
                    );
                }
            }
            _ => {}
        }
    }
}

fn process_bullet_event(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    bullet_entity: &Entity,
    bullet_transform: &Transform,
    bullet_velocity: &Velocity,
    enemy: Result<(Entity, Mut<'_, Enemy>), QueryEntityError>,
) {
    commands.entity(*bullet_entity).despawn();
    spawn_particle_system(&mut commands, bullet_transform.translation.truncate());

    if let Ok((enemy_entity, mut enemy)) = enemy {
        enemy.life -= 1;
        commands.entity(enemy_entity).insert(ExternalForce {
            force: bullet_velocity.linvel.normalize_or_zero() * 10000.0,
            torque: 0.0,
        });
        play_se(&mut commands, &asset_server, "dageki.ogg");
    } else {
        play_se(&mut commands, &asset_server, "shibafu.ogg");
    }
}

fn spawn_particle_system(commands: &mut Commands, position: Vec2) {
    commands
        // Add the bundle specifying the particle system itself.
        .spawn((
            Name::new("particle system"),
            StateScoped(GameState::InGame),
            ParticleSystemBundle {
                transform: Transform::from_translation(position.extend(BULLET_Z)),
                particle_system: ParticleSystem {
                    spawn_rate_per_second: 0.0.into(),
                    max_particles: 100,
                    initial_speed: JitteredValue::jittered(50.0, -50.0..50.0),
                    lifetime: JitteredValue::jittered(0.2, -0.05..0.05),
                    color: ColorOverTime::Constant(Color::WHITE),
                    bursts: vec![ParticleBurst {
                        // このシステムのスケジュールをUpdate意外に設定し、このtimeを0.0にすると、
                        // パーティクルシステムを設置してそのGlobalTransformが更新される前にパーティクルが生成されてしまうため、
                        // パーティクルの発生位置が原点になってしまうことに注意
                        // 0.1くらいにしておくと0.0ではないので大丈夫っぽい
                        time: 0.1,
                        count: 20,
                    }],
                    ..ParticleSystem::oneshot()
                },
                ..ParticleSystemBundle::default()
            },
            Playing,
        ));
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        // ここを FixedUpdate にするとパーティクルの発生位置がおかしくなる
        app.add_systems(
            FixedUpdate,
            update_bullet.run_if(in_state(GameState::InGame)),
        );
        app.register_type::<Bullet>();
    }
}
