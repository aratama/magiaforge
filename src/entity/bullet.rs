use crate::controller::enemy::Enemy;
use crate::controller::remote::RemotePlayer;
use crate::entity::actor::Actor;
use crate::entity::breakable::Breakable;
use crate::entity::bullet_particle::BulletParticleResource;
use crate::entity::damege::spawn_damage;
use crate::entity::EntityDepth;
use crate::firing::Firing;
use crate::level::wall::WallCollider;
use crate::states::GameState;
use crate::{command::GameCommand, entity::bullet_particle::spawn_particle_system};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AseSpriteSlice, Aseprite};
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;
use uuid::Uuid;

static BULLET_Z: f32 = 10.0;

// 魔法の拡散
// const BULLET_SCATTERING: f32 = 0.4;

// 弾丸発射時の、キャラクターと弾丸の間隔
// 小さすぎると、キャラクターの移動時に発射したときに自分自身が衝突してしまうが、
// 大きすぎるとキャラクターと弾丸の位置が離れすぎて不自然
pub const BULLET_SPAWNING_MARGIN: f32 = 9.0;

#[derive(Component, Reflect)]
pub struct Bullet {
    life: u32,
    damage: i32,
    impulse: f32,
    owner: Option<Uuid>,
    homing: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    name: Name,
    bullet: Bullet,
    transform: Transform,
}

/// 指定した種類の弾丸を発射します
/// このとき、アクターへのマナ消費、クールタイムの設定、弾丸の生成、リモート通信などを行います
/// この関数はすでに発射が確定している場合に呼ばれ、発射条件のチェックは行いません
/// 発射条件やコストの消費などは cast_spell で行います
pub fn spawn_bullet(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    writer: &mut EventWriter<GameCommand>,
    group: Group,
    filter: Group,
    firing: &Firing,
) {
    writer.send(GameCommand::SEFire(Some(firing.position)));

    let mut entity = commands.spawn((
        Name::new("bullet"),
        StateScoped(GameState::InGame),
        Bullet {
            life: firing.bullet_lifetime,
            damage: firing.damage,
            impulse: firing.impulse,
            owner: firing.sender,
            homing: firing.homing,
        },
        EntityDepth,
        Transform::from_xyz(firing.position.x, firing.position.y, BULLET_Z)
            * Transform::from_rotation(Quat::from_rotation_z(firing.velocity.to_angle())), // .looking_to(velocity.extend(BULLET_Z), Vec3::Z)
        AseSpriteSlice {
            aseprite,
            name: firing.slice.clone().into(),
        },
        (
            // 衝突にはColliderが必要
            Collider::ball(firing.collier_radius),
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
                linvel: firing.velocity,
                angvel: 0.0,
            },
            GravityScale(0.0),
            Sleeping::disabled(),
            Ccd::enabled(),
        ),
    ));

    if 0.0 < firing.light_intensity {
        entity.insert(PointLight2d {
            radius: firing.light_radius,
            intensity: firing.light_intensity,
            falloff: 10.0,
            color: Color::hsla(
                firing.light_color_hlsa[0],
                firing.light_color_hlsa[1],
                firing.light_color_hlsa[2],
                firing.light_color_hlsa[3],
            ),
            ..default()
        });
    }
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

fn bullet_homing(
    mut bullet_query: Query<(&mut Bullet, &mut Transform, &mut Velocity)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Bullet>)>,
) {
    for (bullet, mut bullet_transform, mut velocity) in bullet_query.iter_mut() {
        if 0.0 < bullet.homing {
            let bullet_position = bullet_transform.translation.truncate();
            let mut enemies = Vec::from_iter(enemy_query.iter());
            enemies.sort_by(|a, b| {
                let x = a.translation.truncate().distance(bullet_position);
                let y = b.translation.truncate().distance(bullet_position);
                x.total_cmp(&y)
            });
            if let Some(nearest) = enemies.first() {
                let bullet_angle = velocity.linvel.to_angle();
                let target_angle = (nearest.translation.truncate() - bullet_position).to_angle();
                let angle_diff = target_angle - bullet_angle;
                let next_angle = angle_diff.signum() * angle_diff.abs().min(bullet.homing);
                velocity.linvel =
                    Vec2::from_angle(bullet_angle + next_angle) * velocity.linvel.length();
                bullet_transform.rotation = Quat::from_rotation_z(velocity.linvel.to_angle());
            }
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
    resource: Res<BulletParticleResource>,
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
                    &resource,
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
                        &resource,
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
    resource: &Res<BulletParticleResource>,
) -> bool {
    if let Ok((bullet_entity, bullet, bullet_transform, bullet_velocity)) = query.get(*a) {
        let bullet_position = bullet_transform.translation.truncate();

        if !despownings.contains(&bullet_entity) {
            if let Ok((mut actor, mut impilse, mut breakable)) = actors.get_mut(*b) {
                trace!("bullet hit actor: {:?}", actor.uuid);

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
                    spawn_particle_system(&mut commands, bullet_position, resource);
                    spawn_damage(&mut commands, bullet.damage, bullet_position);
                    writer.send(GameCommand::SEDamage(Some(bullet_position)));
                }
            } else if let Ok(mut breakabke) = breakabke_query.get_mut(*b) {
                trace!("bullet hit breakable: {:?}", b);
                breakabke.life -= bullet.damage;
                breakabke.amplitude = 2.0;
                despownings.insert(bullet_entity.clone());
                commands.entity(bullet_entity).despawn_recursive();
                spawn_particle_system(&mut commands, bullet_position, resource);
                spawn_damage(&mut commands, bullet.damage, bullet_position);
                writer.send(GameCommand::SEDamage(Some(bullet_position)));
            } else if let Ok(_) = wall_collider_query.get(*b) {
                trace!("bullet hit wall: {:?}", b);
                despownings.insert(bullet_entity.clone());
                commands.entity(bullet_entity).despawn_recursive();
                spawn_particle_system(&mut commands, bullet_position, resource);
                writer.send(GameCommand::SESteps(Some(bullet_position)));
            } else {
                trace!("bullet hit unknown entity: {:?}", b);
                despownings.insert(bullet_entity.clone());
                commands.entity(bullet_entity).despawn_recursive();
                spawn_particle_system(&mut commands, bullet_position, resource);
                writer.send(GameCommand::SENoDamage(Some(bullet_position)));
            }
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
            (despawn_bullet_by_lifetime, bullet_collision, bullet_homing)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Bullet>();
    }
}
