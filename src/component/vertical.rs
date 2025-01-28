use crate::{
    actor::Actor, entity::impact::SpawnImpact, registry::Registry, set::FixedUpdateGameActiveSet,
};
use bevy::prelude::*;

fn fall(
    registry: Registry,
    mut child_query: Query<(Entity, &mut Actor, &Transform)>,
    mut spawn: EventWriter<SpawnImpact>,
) {
    for (entity, mut actor, transform) in child_query.iter_mut() {
        let next = actor.v + actor.velocity;
        if next <= 0.0 {
            actor.just_landed = 0.0 < actor.v;
            actor.v = 0.0;
            actor.velocity = 0.0;
            if actor.just_landed {
                let props = registry.get_actor_props(&actor.actor_type);
                if 0.0 < props.impact_radius {
                    let position = transform.translation.truncate();
                    spawn.send(SpawnImpact {
                        position,
                        radius: props.impact_radius,
                        impulse: 16.0,
                        owner: Some(entity),
                    });
                }
            }
        } else {
            actor.just_landed = false;
            actor.v = next;
            actor.velocity += actor.gravity;
        }
    }
}

/// 子エンティティのスプライトに付与し
/// y座標を変化させて落下させるコンポーネントです
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct ApplyFalling;

fn apply_falling_transform(mut child_query: Query<(&mut Transform, &Actor), With<ApplyFalling>>) {
    for (mut child_transform, vertical) in child_query.iter_mut() {
        child_transform.translation.y = vertical.v;
    }
}

pub struct VerticalPlugin;

impl Plugin for VerticalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fall, apply_falling_transform)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
