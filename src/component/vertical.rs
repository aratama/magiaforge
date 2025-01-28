use crate::{
    actor::Actor, entity::impact::SpawnImpact, registry::Registry, set::FixedUpdateGameActiveSet,
};
use bevy::prelude::*;

/// 子エンティティのスプライトに付与し
/// y座標を変化させて落下させるコンポーネントです
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct Vertical {
    pub velocity: f32,
    pub gravity: f32,
    pub just_landed: bool,
    pub v: f32,
}

impl Default for Vertical {
    fn default() -> Self {
        Self {
            velocity: 0.0,
            gravity: -0.2,
            just_landed: false,
            v: 0.0,
        }
    }
}

impl Vertical {
    pub fn new(velocity: f32, gravity: f32) -> Self {
        Self {
            velocity,
            gravity,
            ..default()
        }
    }
}

fn fall(
    registry: Registry,
    mut child_query: Query<(Entity, &Actor, &mut Vertical, &Transform)>,
    mut spawn: EventWriter<SpawnImpact>,
) {
    for (entity, actor, mut vertical, transform) in child_query.iter_mut() {
        let next = vertical.v + vertical.velocity;
        if next <= 0.0 {
            vertical.just_landed = 0.0 < vertical.v;
            vertical.v = 0.0;
            vertical.velocity = 0.0;
            if vertical.just_landed {
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
            vertical.just_landed = false;
            vertical.v = next;
            vertical.velocity += vertical.gravity;
        }
    }
}

/// 子エンティティのスプライトに付与し
/// y座標を変化させて落下させるコンポーネントです
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct ApplyFalling;

fn apply_falling_transform(
    mut child_query: Query<(&mut Transform, &Vertical), With<ApplyFalling>>,
) {
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
