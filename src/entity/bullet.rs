use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::component::entity_depth::EntityDepth;
use crate::controller::remote::RemotePlayer;
use crate::entity::bullet_particle::spawn_particle_system;
use crate::entity::bullet_particle::BulletParticleResource;
use crate::entity::bullet_particle::SpawnParticle;
use crate::level::collision::WallCollider;
use crate::level::entities::SpawnEntity;
use crate::level::entities::SpawnEntityEvent;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::physics::identify_single;
use crate::physics::IdentifiedCollisionEvent;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
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
    freeze: u32,
    stagger: u32,
    actor_group: ActorGroup,
    pub holder: Option<(Entity, Trigger)>,
    levitation: u32,
    metamorphose: Option<ActorType>,
    dispel: bool,
    web: bool,
    slash: Option<u32>,
}

#[derive(Bundle)]
pub struct BulletBundle {
    name: Name,
    bullet: Bullet,
    transform: Transform,
}

#[derive(Component, Reflect)]
pub struct HomingTarget;

#[derive(Component, Reflect)]
pub struct BulletResidual {
    count: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Reflect, PartialEq, Eq)]
pub enum Trigger {
    Primary,
    Secondary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BulletImage {
    Slice { names: Vec<String> },
    Freeze,
}

/// 生成される弾丸の大半の情報を収めた構造体です
/// 実際に弾丸を生成する spawn_bullet 関数のパラメータとして使われるほか、
/// リモートで送信される RemoteMessage::Fire のデータとしても共通で使われることで、
/// ローカルとリモートの弾丸生成を共通化します
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct SpawnBullet {
    /// 発射したアクターのUUID
    pub sender: Option<Uuid>,

    // LightSwordで待機中
    pub holder: Option<(Entity, Trigger)>,

    /// 発射したアクターのグループ
    pub actor_group: ActorGroup,
    pub groups: CollisionGroups,

    pub uuid: Uuid,
    pub position: Vec2,
    pub velocity: Vec2,
    pub bullet_lifetime: u32,
    pub damage: i32,
    pub impulse: f32,
    pub slices: BulletImage,
    pub collier_radius: f32,
    pub light_intensity: f32,
    pub light_radius: f32,
    pub light_color_hlsa: [f32; 4],
    pub homing: f32,
    pub freeze: u32,
    pub stagger: u32,
    pub levitation: u32,
    pub metamorphose: Option<ActorType>,
    pub dispel: bool,
    pub web: bool,
    pub slash: Option<u32>,
}

/// 指定した種類の弾丸を発射します
/// このとき、アクターへのマナ消費、クールタイムの設定、弾丸の生成、リモート通信などを行います
/// この関数はすでに発射が確定している場合に呼ばれ、発射条件のチェックは行いません
/// 発射条件やコストの消費などは cast_spell で行います
///
/// 弾丸が物体に衝突した場合、それがActorまたはlifeであればダメージを与えてから消滅します
/// それ以外の物体に衝突した場合はそのまま消滅します
pub fn spawn_bullet(
    commands: &mut Commands,
    registry: &Registry,
    writer: &mut EventWriter<SEEvent>,
    spawn: &SpawnBullet,
) {
    let mut rng = rand::thread_rng();

    writer.send(SEEvent::pos(SE::Fire, spawn.position));

    let mut entity = commands.spawn((
        Name::new("bullet"),
        StateScoped(GameState::InGame),
        Bullet {
            life: spawn.bullet_lifetime,
            damage: spawn.damage,
            impulse: spawn.impulse,
            owner: spawn.sender,
            homing: spawn.homing,
            freeze: spawn.freeze,
            stagger: spawn.stagger,
            actor_group: spawn.actor_group,
            holder: spawn.holder,
            levitation: spawn.levitation,
            metamorphose: spawn.metamorphose,
            dispel: spawn.dispel,
            web: spawn.web,
            slash: spawn.slash,
        },
        EntityDepth::new(),
        Transform::from_xyz(spawn.position.x, spawn.position.y, BULLET_Z)
            * Transform::from_rotation(Quat::from_rotation_z(spawn.velocity.to_angle())), // .looking_to(velocity.extend(BULLET_Z), Vec3::Z)
        (
            // 衝突にはColliderが必要
            Collider::ball(spawn.collier_radius),
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
            spawn.groups,
            //
            Velocity {
                linvel: spawn.velocity,
                angvel: 0.0,
            },
            GravityScale(0.0),
            Sleeping::disabled(),
            Ccd::enabled(),
        ),
    ));

    match spawn.slices {
        BulletImage::Slice { ref names } => entity.insert(AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
            name: names.choose(&mut rng).unwrap().clone().into(),
        }),
        BulletImage::Freeze => entity.insert(AseSpriteAnimation {
            aseprite: registry.assets.freeze.clone(),
            animation: "default".into(),
        }),
    };

    if 0.0 < spawn.light_intensity {
        entity.insert(PointLight2d {
            radius: spawn.light_radius,
            intensity: spawn.light_intensity,
            falloff: 10.0,
            color: Color::hsla(
                spawn.light_color_hlsa[0],
                spawn.light_color_hlsa[1],
                spawn.light_color_hlsa[2],
                spawn.light_color_hlsa[3],
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
    enemy_query: Query<(&Actor, &Transform), (With<HomingTarget>, Without<Bullet>)>,
) {
    for (bullet, mut bullet_transform, mut velocity) in bullet_query.iter_mut() {
        if 0.0 < bullet.homing {
            // ターゲットを絞り込む
            let mut enemies = Vec::<Transform>::new();
            for (enemy, enemy_transform) in enemy_query.iter() {
                if bullet.actor_group != enemy.actor_group {
                    enemies.push(*enemy_transform);
                }
            }

            // 最も近いターゲットを選択
            let bullet_position = bullet_transform.translation.truncate();
            enemies.sort_by(|a, b| {
                let x = a.translation.truncate().distance(bullet_position);
                let y = b.translation.truncate().distance(bullet_position);
                x.total_cmp(&y)
            });

            // 選択したターゲットの方向へ回転
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

fn bullet_freeze_water(
    bullet_query: Query<(&Bullet, &Transform)>,
    mut level: ResMut<LevelSetup>,
    mut se: EventWriter<SEEvent>,
) {
    if let Some(ref mut chunk) = level.chunk {
        for (bullet, transform) in bullet_query.iter() {
            if 0 < bullet.freeze {
                let position = transform.translation.truncate();
                match chunk.get_tile_by_coords(position) {
                    Tile::Water => {
                        chunk.set_tile_by_position(position, Tile::Ice);
                        se.send(SEEvent::pos(SE::Freeze, position));
                    }
                    Tile::Lava => {
                        chunk.set_tile_by_position(position, Tile::Soil);
                        se.send(SEEvent::pos(SE::Freeze, position));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn bullet_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut actor_event: EventWriter<ActorEvent>,
    mut spawn: EventWriter<SpawnEntityEvent>,
    mut se: EventWriter<SEEvent>,
    mut actor_query: Query<&mut Actor, Without<RemotePlayer>>,
    bullet_query: Query<(&Bullet, &Transform, &Velocity)>,
    wall_collider_query: Query<Entity, With<WallCollider>>,
) {
    // 弾丸が壁の角に当たった場合、衝突イベントが同時に複数回発生することがあります
    // すでにdespawnしたentityに対して再びdespawnしてしまい警告が出るのを避けるため、
    // 処理済みのentityを識別するセットを使っています
    // https://github.com/bevyengine/bevy/issues/5617
    let mut despawnings: HashSet<Entity> = HashSet::new();

    for collision_event in collision_events.read() {
        let IdentifiedCollisionEvent::Started(bullet_entity, other_entity) =
            identify_single(&collision_event, &bullet_query)
        else {
            continue;
        };

        let (bullet, bullet_transform, bullet_velocity) = bullet_query.get(bullet_entity).unwrap();

        let bullet_position = bullet_transform.translation.truncate();

        let bullet_damage = if bullet.holder.is_some() {
            0
        } else {
            bullet.damage
        };

        if despawnings.contains(&bullet_entity) {
            continue;
        }

        despawnings.insert(bullet_entity.clone());
        commands.entity(bullet_entity).despawn_recursive();
        spawn_bullet_effect(&mut spawn, &bullet, bullet_position, &bullet_velocity);

        if let Ok(mut actor) = actor_query.get_mut(other_entity) {
            // 弾丸がアクターに衝突したとき
            // このクエリにはプレイヤーキャラクター自身、発射したキャラクター自身も含まれることに注意
            // リモートプレイヤーのダメージやノックバックはリモートで処理されるため、ここでは処理しない

            // TODO
            // Life と Actor は分離されているので、Damagedイベントでは扱わない
            // 一貫性がない？
            if actor.frozen == 0 && 0 < bullet.freeze {
                se.send(SEEvent::pos(SE::Freeze, bullet_position));
            }
            actor.frozen += bullet.freeze;

            if actor.levitation == 0 && 0 < bullet.levitation {
                se.send(SEEvent::pos(SE::Status2, bullet_position));
            }
            actor.levitation += bullet.levitation;

            actor_event.send(ActorEvent::Damaged {
                actor: other_entity,
                damage: bullet_damage as u32,
                position: bullet_position,
                fire: false,
                impulse: bullet_velocity.linvel.normalize_or_zero() * bullet.impulse,
                stagger: bullet.stagger,
                metamorphose: bullet.metamorphose,
                dispel: bullet.dispel,
            });
        } else {
            // リモートプレイヤーに命中した場合もここ
            // ヒット判定やダメージなどはリモート側で処理します
            se.send(SEEvent::pos(
                if wall_collider_query.contains(other_entity) {
                    SE::Steps
                } else {
                    SE::NoDamage
                },
                bullet_position,
            ));
        }
    }
}

fn spawn_bullet_effect(
    spawn: &mut EventWriter<SpawnEntityEvent>,
    bullet: &Bullet,
    bullet_position: Vec2,
    bullet_velocity: &Velocity,
) {
    spawn.send(SpawnEntityEvent {
        position: bullet_position,
        entity: SpawnEntity::Particle {
            spawn: SpawnParticle::default(),
        },
    });

    if bullet.web {
        spawn.send(SpawnEntityEvent {
            position: bullet_position,
            entity: SpawnEntity::Web {
                owner_actor_group: bullet.actor_group,
            },
        });
    }

    if let Some(damage) = bullet.slash {
        spawn.send(SpawnEntityEvent {
            position: bullet_position,
            entity: SpawnEntity::Slash {
                velocity: Vec2::ZERO,
                actor_group: bullet.actor_group,
                angle: bullet_velocity.linvel.to_angle(),
                damage,
            },
        });
    }
}

fn despawn_bullet_residual(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BulletResidual, &Transform)>,
    resource: Res<BulletParticleResource>,
) {
    for (entity, mut residual, transform) in query.iter_mut() {
        residual.count -= 1;
        if residual.count <= 0 {
            commands.entity(entity).despawn_recursive();

            spawn_particle_system(
                &mut commands,
                transform.translation.truncate(),
                &resource,
                &SpawnParticle::default(),
            );
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                despawn_bullet_by_lifetime,
                bullet_collision,
                bullet_homing,
                bullet_freeze_water,
                despawn_bullet_residual,
            )
                .in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Bullet>();
    }
}
