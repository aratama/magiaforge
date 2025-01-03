use crate::states::{GameState, TimeState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// 子エンティティのスプライトに付与し
/// y座標を変化させて落下させるコンポーネントです
#[derive(Default, Component, Reflect)]
pub struct Falling {
    pub velocity: f32,
    pub gravity: f32,
    pub just_landed: bool,
}

impl Falling {
    pub fn new(velocity: f32, gravity: f32) -> Self {
        Self {
            velocity,
            gravity,
            just_landed: false,
        }
    }
}

fn fall(mut child_query: Query<(&mut Transform, &mut Falling)>) {
    for (mut child_transform, mut falling) in child_query.iter_mut() {
        let next = child_transform.translation.y + falling.velocity;
        if next <= 0.0 {
            falling.just_landed = 0.0 < child_transform.translation.y;
            child_transform.translation.y = 0.0;
            falling.velocity = 0.0;
        } else {
            falling.just_landed = false;
            child_transform.translation.y = next;
            falling.velocity += falling.gravity;
        }
    }
}

pub struct FallingPlugin;

impl Plugin for FallingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            fall.run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
