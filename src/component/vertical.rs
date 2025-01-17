use crate::set::FixedUpdateGameActiveSet;
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
            just_landed: false,
            v: 0.0,
        }
    }
}

fn fall(mut child_query: Query<&mut Vertical>) {
    for mut vertical in child_query.iter_mut() {
        let next = vertical.v + vertical.velocity;
        if next <= 0.0 {
            vertical.just_landed = 0.0 < vertical.v;
            vertical.v = 0.0;
            vertical.velocity = 0.0;
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

pub struct FallingPlugin;

impl Plugin for FallingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fall, apply_falling_transform)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
