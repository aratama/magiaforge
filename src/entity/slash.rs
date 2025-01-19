use crate::asset::GameAssets;
use crate::collision::PLAYER_GROUPS;
use crate::component::counter::Counter;
use crate::component::life::Life;
use crate::constant::PARTICLE_LAYER_Z;
use crate::entity::actor::{ActorEvent, ActorGroup};
use crate::se::{SEEvent, SE};
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::plugin::{DefaultRapierContext, RapierContext};
use bevy_rapier2d::prelude::{Collider, QueryFilter};
use core::f32;

use super::grass::Grasses;

#[derive(Component)]
struct Slash;

pub fn spawn_slash(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    se: &mut EventWriter<SEEvent>,
    parent: Entity,
    position: Vec2,
    angle: f32,
    context_query: &mut Query<&mut RapierContext, With<DefaultRapierContext>>,
    actor_group: ActorGroup,
    actor_event: &mut EventWriter<ActorEvent>,
    life_query: &Query<&Transform, With<Life>>,
    grass_query: &Query<(Entity, &Transform), (With<Grasses>, Without<Life>)>,
) {
    let rotation = Quat::from_rotation_z(angle);
    let entity = commands
        .spawn((
            Slash,
            Counter::default(),
            AseSpriteAnimation {
                aseprite: assets.slash.clone(),
                animation: "default".into(),
            },
            Transform::from_translation(Vec2::ZERO.extend(PARTICLE_LAYER_Z))
                .with_rotation(rotation),
        ))
        .id();
    commands.entity(parent).add_child(entity);
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
                        damage: 10,
                        fire: false,
                        impulse: Vec2::ZERO,
                        stagger: 30,
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
