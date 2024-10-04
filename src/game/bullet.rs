use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteSliceBundle;
use bevy_rapier2d::prelude::*;

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
    commands.spawn((
        Bullet { life: 120 },
        AsepriteSliceBundle {
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
    ));
}

pub fn update_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bullet)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for (entity, mut bullet) in query.iter_mut() {
        bullet.life -= 1;
        if bullet.life <= 0 {
            commands.entity(entity).despawn();
        }
    }
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                // 弾丸は何かに接触した時点で消滅する
                if query.contains(*a) {
                    commands.entity(*a).despawn();
                }
                if query.contains(*b) {
                    commands.entity(*b).despawn();
                }
            }
            _ => {}
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bullet);
    }
}
