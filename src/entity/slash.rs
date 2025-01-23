use crate::actor::ActorEvent;
use crate::actor::ActorGroup;
use crate::collision::PLAYER_GROUPS;
use crate::component::counter::Counter;
use crate::component::life::Life;
use crate::constant::PARTICLE_LAYER_Z;
use crate::entity::grass::Grasses;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::Group;
use bevy_rapier2d::prelude::QueryFilter;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Velocity;
use core::f32;

#[derive(Component)]
struct Slash;

pub fn spawn_slash(
    commands: &mut Commands,
    registry: &Registry,
    se: &mut EventWriter<SEEvent>,
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    context_query: &mut Query<&mut RapierContext, With<DefaultRapierContext>>,
    actor_group: ActorGroup,
    actor_event: &mut EventWriter<ActorEvent>,
    life_query: &Query<&Transform, With<Life>>,
    grass_query: &Query<(Entity, &Transform), (With<Grasses>, Without<Life>)>,
    damage: u32,
) {
    let rotation = Quat::from_rotation_z(angle);
    commands.spawn((
        Slash,
        Counter::default(),
        AseSpriteAnimation {
            aseprite: registry.assets.slash.clone(),
            animation: "default".into(),
        },
        Transform::from_translation(position.extend(PARTICLE_LAYER_Z)).with_rotation(rotation),
        (
            RigidBody::KinematicVelocityBased,
            Collider::ball(1.0),
            Velocity {
                linvel: velocity,
                angvel: 0.0,
            },
            CollisionGroups::new(Group::NONE, Group::NONE),
        ),
    ));
    se.send(SEEvent::pos(SE::Ken2, position));

    let context = context_query.single_mut();

    // 破壊可能オブジェクトにダメージ
    context.intersections_with_shape(
        position,
        0.0,
        &Collider::ball(32.0),
        QueryFilter::default().groups(actor_group.to_bullet_group()),
        |e| {
            if let Ok(transform) = life_query.get(e) {
                let target = transform.translation.truncate();
                let t = (target - position).angle_to(Vec2::from_angle(angle));
                if t.abs() < f32::consts::PI * 0.5 {
                    actor_event.send(ActorEvent::Damaged {
                        actor: e,
                        position: target,
                        damage,
                        fire: false,
                        impulse: Vec2::ZERO,
                        stagger: 30,
                        metamorphose: None,
                        dispel: false,
                    });
                }
            }
            true
        },
    );

    // 草を刈る
    context.intersections_with_shape(
        position,
        0.0,
        &Collider::ball(32.0),
        QueryFilter::default().groups(*PLAYER_GROUPS),
        |e| {
            if let Ok((grass, transform)) = grass_query.get(e) {
                let target = transform.translation.truncate();
                let t = (target - position).angle_to(Vec2::from_angle(angle));
                if t.abs() < f32::consts::PI * 0.5 {
                    commands.entity(grass).despawn_recursive();
                }
            }
            true
        },
    );
}

fn update_impact(mut commands: Commands, mut query: Query<(Entity, &Counter), With<Slash>>) {
    for (entity, counter) in query.iter_mut() {
        if 30 <= counter.count {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct SlashPlugin;

impl Plugin for SlashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_impact).in_set(FixedUpdateGameActiveSet),
        );
    }
}
