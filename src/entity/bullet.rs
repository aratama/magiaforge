use super::actor::Actor;
use crate::asset::GameAssets;
use crate::constant::{BULLET_GROUP, ENEMY_GROUP, WALL_GROUP};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Aseprite, AsepriteSliceBundle};
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

const SLICE_NAME: &str = "bullet";

static BULLET_Z: f32 = 10.0;

static BULLET_IMPULSE: f32 = 20000.0;

pub const BULLET_RADIUS: f32 = 10.0;

const BULLET_DAMAGE: i32 = 5;

const BULLET_LIFETIME: u32 = 2000;

// 弾丸発射時の、キャラクターと弾丸の間隔
// 小さすぎると、キャラクターの移動時に発射したときに自分自身が衝突してしまうが、
// 大きすぎるとキャラクターと弾丸の位置が離れすぎて不自然
pub const BULLET_SPAWNING_MARGIN: f32 = 10.0;

#[derive(Component, Reflect)]
pub struct Bullet {
    life: u32,
    damage: i32,
    impulse: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    name: Name,
    bullet: Bullet,
    transform: Transform,
}

pub fn spawn_bullet(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    velocity: Vec2,
    assets: &Res<GameAssets>,
) {
    commands.spawn((
        Name::new("bullet"),
        StateScoped(GameState::InGame),
        Bullet {
            life: BULLET_LIFETIME,
            damage: BULLET_DAMAGE,
            impulse: BULLET_IMPULSE,
        },
        AsepriteSliceBundle {
            aseprite,
            slice: SLICE_NAME.into(),
            transform: Transform::from_xyz(position.x, position.y, BULLET_Z)
                * Transform::from_rotation(Quat::from_rotation_z(velocity.to_angle())), // .looking_to(velocity.extend(BULLET_Z), Vec3::Z)
            ..default()
        },
        (
            Velocity {
                linvel: velocity,
                angvel: 0.0,
            },
            KinematicCharacterController::default(),
            RigidBody::KinematicVelocityBased,
            // 弾丸が大きくなると衝突時の位置の精度が悪化するので小さくしてあります
            Collider::ball(BULLET_RADIUS),
            GravityScale(0.0),
            // https://rapier.rs/docs/user_guides/bevy_plugin/colliders#active-collision-types
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS,
            Sleeping::disabled(),
            Ccd::enabled(),
            // https://rapier.rs/docs/user_guides/bevy_plugin/colliders#collision-groups-and-solver-groups
            CollisionGroups::new(BULLET_GROUP, WALL_GROUP | ENEMY_GROUP),
        ),
    ));
}

pub fn update_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet, &Transform, &Velocity)>,
    mut enemy_query: Query<(&mut Actor, &mut ExternalImpulse)>,

    assets: Res<GameAssets>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    // 弾丸のライフタイムを減らし、ライフタイムが尽きたら削除
    for (entity, mut bullet, _, _) in bullet_query.iter_mut() {
        bullet.life -= 1;
        if bullet.life <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }

    let mut despownings: HashSet<Entity> = HashSet::new();

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                if !process_bullet_event(
                    &mut commands,
                    &assets,
                    &mut bullet_query,
                    &mut enemy_query,
                    &mut despownings,
                    &a,
                    &b,
                ) {
                    process_bullet_event(
                        &mut commands,
                        &assets,
                        &mut bullet_query,
                        &mut enemy_query,
                        &mut despownings,
                        &b,
                        &a,
                    );
                }
            }
            _ => {}
        }
    }
}

fn process_bullet_event(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    query: &Query<(Entity, &mut Bullet, &Transform, &Velocity)>,

    // TODO プレイヤーキャラくらーにもダメージが入るようにする
    actors: &mut Query<(&mut Actor, &mut ExternalImpulse)>,

    respownings: &mut HashSet<Entity>,
    a: &Entity,
    b: &Entity,
) -> bool {
    if let Ok((bullet_entity, bullet, bullet_transform, bullet_velocity)) = query.get(*a) {
        let bullet_position = bullet_transform.translation.truncate();

        // 弾丸が壁の角に当たった場合、衝突イベントが同時に複数回発生するため、
        // すでにdespownしたentityに対して再びdespownしてしまうことがあり、
        // 警告が出るのを避けるため、処理済みのentityを識別するセットを使っています
        // https://github.com/bevyengine/bevy/issues/5617
        if !respownings.contains(&bullet_entity) {
            respownings.insert(bullet_entity.clone());
            commands.entity(bullet_entity).despawn_recursive();

            true
        } else {
            false
        }
    } else {
        false
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_bullet.run_if(in_state(GameState::InGame)),
        );
        app.register_type::<Bullet>();
    }
}
