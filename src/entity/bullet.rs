use super::actor::Actor;
use super::breakable::Breakable;
use super::EntityDepth;
use crate::command::GameCommand;
use crate::constant::*;
use crate::controller::remote::RemotePlayer;
use crate::states::GameState;
use crate::world::wall::WallCollider;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Aseprite, AsepriteSliceBundle};
use bevy_light_2d::light::PointLight2d;
use bevy_particle_systems::{
    ColorOverTime, JitteredValue, ParticleBurst, ParticleSystem, ParticleSystemBundle, Playing,
};
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;
use uuid::Uuid;

const SLICE_NAME: &str = "bullet";

static BULLET_Z: f32 = 10.0;

static BULLET_IMPULSE: f32 = 20000.0;

pub const BULLET_RADIUS: f32 = 5.0;

const BULLET_DAMAGE: i32 = 5;

// 弾丸発射時の、キャラクターと弾丸の間隔
// 小さすぎると、キャラクターの移動時に発射したときに自分自身が衝突してしまうが、
// 大きすぎるとキャラクターと弾丸の位置が離れすぎて不自然
pub const BULLET_SPAWNING_MARGIN: f32 = 4.0;

#[derive(Component, Reflect)]
pub struct Bullet {
    life: u32,
    damage: i32,
    impulse: f32,
    owner: Option<Uuid>,
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
    lifetime: u32,
    owner: Option<Uuid>,
    writer: &mut EventWriter<GameCommand>,
    group: Group,
    filter: Group,
) {
    writer.send(GameCommand::SESuburi(Some(position)));

    commands.spawn((
        Name::new("bullet"),
        StateScoped(GameState::InGame),
        Bullet {
            life: lifetime,
            damage: BULLET_DAMAGE,
            impulse: BULLET_IMPULSE,
            owner,
        },
        EntityDepth,
        AsepriteSliceBundle {
            aseprite,
            slice: SLICE_NAME.into(),
            transform: Transform::from_xyz(position.x, position.y, BULLET_Z)
                * Transform::from_rotation(Quat::from_rotation_z(velocity.to_angle())), // .looking_to(velocity.extend(BULLET_Z), Vec3::Z)
            ..default()
        },
        (
            // 衝突にはColliderが必要
            Collider::ball(BULLET_RADIUS),
            // 速度ベースで制御するので KinematicVelocityBased
            // これがないと Velocityを設定しても移動しない
            RigidBody::KinematicVelocityBased,
            // KinematicCharacterControllerは不要に見えるが、
            // これを外すと衝突イベントが起こらない不具合がたまに起こる？
            KinematicCharacterController::default(),
            // デフォルトでは KINEMATIC_STATIC が含まれず、KINEMATICである弾丸とSTATICの壁が衝突しないので
            // KINEMATIC_STATICを追加
            // https://rapier.rs/docs/user_guides/bevy_plugin/colliders#active-collision-types
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            // 衝突を発生されるには ActiveEvents も必要
            ActiveEvents::COLLISION_EVENTS,
            // https://rapier.rs/docs/user_guides/bevy_plugin/colliders#collision-groups-and-solver-groups
            CollisionGroups::new(group, filter),
            //
            Velocity {
                linvel: velocity,
                angvel: 0.0,
            },
            GravityScale(0.0),
            Sleeping::disabled(),
            Ccd::enabled(),
        ),
        PointLight2d {
            radius: 50.0,
            intensity: 1.0,
            falloff: 10.0,
            color: Color::hsl(245.0, 1.0, 0.6),
            ..default()
        },
    ));
}

fn despawn_bullet_by_lifetime(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet, &Transform, &Velocity)>,
) {
    // 弾丸のライフタイムを減らし、ライフタイムが尽きたら削除
    for (entity, mut bullet, _, _) in bullet_query.iter_mut() {
        bullet.life -= 1;
        if bullet.life <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn bullet_collision(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet, &Transform, &Velocity)>,
    mut actor_query: Query<
        (&mut Actor, &mut ExternalImpulse, &mut Breakable),
        Without<RemotePlayer>,
    >,
    mut breakable_query: Query<&mut Breakable, Without<Actor>>,
    mut collision_events: EventReader<CollisionEvent>,
    wall_collider_query: Query<Entity, With<WallCollider>>,
    mut writer: EventWriter<GameCommand>,
) {
    // 弾丸が壁の角に当たった場合、衝突イベントが同時に複数回発生するため、
    // すでにdespawnしたentityに対して再びdespawnしてしまうことがあり、
    // 警告が出るのを避けるため、処理済みのentityを識別するセットを使っています
    // https://github.com/bevyengine/bevy/issues/5617
    let mut despawnings: HashSet<Entity> = HashSet::new();

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                if !process_bullet_event(
                    &mut commands,
                    &mut bullet_query,
                    &mut actor_query,
                    &mut breakable_query,
                    &mut despawnings,
                    &a,
                    &b,
                    &wall_collider_query,
                    &mut writer,
                ) {
                    process_bullet_event(
                        &mut commands,
                        &mut bullet_query,
                        &mut actor_query,
                        &mut breakable_query,
                        &mut despawnings,
                        &b,
                        &a,
                        &wall_collider_query,
                        &mut writer,
                    );
                }
            }
            _ => {}
        }
    }
}

fn process_bullet_event(
    mut commands: &mut Commands,
    query: &Query<(Entity, &mut Bullet, &Transform, &Velocity)>,
    actors: &mut Query<(&mut Actor, &mut ExternalImpulse, &mut Breakable), Without<RemotePlayer>>,
    breakabke_query: &mut Query<&mut Breakable, Without<Actor>>,
    despownings: &mut HashSet<Entity>,
    a: &Entity,
    b: &Entity,
    wall_collider_query: &Query<Entity, With<WallCollider>>,
    writer: &mut EventWriter<GameCommand>,
) -> bool {
    if let Ok((bullet_entity, bullet, bullet_transform, bullet_velocity)) = query.get(*a) {
        let bullet_position = bullet_transform.translation.truncate();

        if !despownings.contains(&bullet_entity) {
            if let Ok((mut actor, mut impilse, mut breakable)) = actors.get_mut(*b) {
                info!("bullet hit actor: {:?}", actor.uuid);

                // 弾丸がアクターに衝突したとき
                // このクエリにはプレイヤーキャラクター自身、発射したキャラクター自身も含まれることに注意
                // 弾丸の詠唱者自身に命中した場合はダメージやノックバックはなし
                // リモートプレイヤーのダメージやノックバックはリモートで処理されるため、ここでは処理しない
                if bullet.owner == None || Some(actor.uuid) != bullet.owner {
                    actor.life = (actor.life - bullet.damage).max(0);
                    breakable.amplitude = 6.0;
                    impilse.impulse += bullet_velocity.linvel.normalize_or_zero() * bullet.impulse;
                    despownings.insert(bullet_entity.clone());
                    commands.entity(bullet_entity).despawn_recursive();
                    spawn_particle_system(&mut commands, bullet_position);
                    writer.send(GameCommand::SEDageki(Some(bullet_position)));
                }
            } else if let Ok(mut breakabke) = breakabke_query.get_mut(*b) {
                info!("bullet hit breakable: {:?}", b);
                breakabke.life -= bullet.damage;
                breakabke.amplitude = 2.0;
                despownings.insert(bullet_entity.clone());
                commands.entity(bullet_entity).despawn_recursive();
                spawn_particle_system(&mut commands, bullet_position);
                writer.send(GameCommand::SEDageki(Some(bullet_position)));
            } else if let Ok(_) = wall_collider_query.get(*b) {
                info!("bullet hit wall: {:?}", b);
                despownings.insert(bullet_entity.clone());
                commands.entity(bullet_entity).despawn_recursive();
                spawn_particle_system(&mut commands, bullet_position);
                writer.send(GameCommand::SEAsphalt(Some(bullet_position)));
            } else {
                info!("bullet hit unknown entity: {:?}", b);
                despownings.insert(bullet_entity.clone());
                commands.entity(bullet_entity).despawn_recursive();
                spawn_particle_system(&mut commands, bullet_position);
                writer.send(GameCommand::SEShibafu(Some(bullet_position)));
            }
            true
        } else {
            false
        }
    } else {
        false
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
                    system_duration_seconds: 0.2,
                    ..ParticleSystem::oneshot()
                },
                ..ParticleSystemBundle::default()
            },
            Playing,
            PointLight2d {
                radius: 50.0,
                intensity: 1.0,
                falloff: 10.0,
                color: Color::hsl(245.0, 1.0, 0.6),
                ..default()
            },
        ));
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (despawn_bullet_by_lifetime, bullet_collision).run_if(in_state(GameState::InGame)),
        );
        app.register_type::<Bullet>();
    }
}
