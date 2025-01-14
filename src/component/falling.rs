use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

/// 子エンティティのスプライトに付与し
/// y座標を変化させて落下させるコンポーネントです
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct Falling {
    pub velocity: f32,
    pub gravity: f32,
    pub just_landed: bool,
    pub v: f32,
}

impl Default for Falling {
    fn default() -> Self {
        Self {
            velocity: 0.0,
            gravity: -0.2,
            just_landed: false,
            v: 0.0,
        }
    }
}

impl Falling {
    pub fn new(velocity: f32, gravity: f32) -> Self {
        Self {
            velocity,
            gravity,
            just_landed: false,
            v: 0.0,
        }
    }
}

fn fall(mut child_query: Query<&mut Falling>) {
    for mut falling in child_query.iter_mut() {
        let next = falling.v + falling.velocity;
        if next <= 0.0 {
            falling.just_landed = 0.0 < falling.v;
            falling.v = 0.0;
            falling.velocity = 0.0;
        } else {
            falling.just_landed = false;
            falling.v = next;
            falling.velocity += falling.gravity;
        }
    }
}

/// 子エンティティのスプライトに付与し
/// y座標を変化させて落下させるコンポーネントです
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct ApplyFalling;

fn apply_falling_transform(mut child_query: Query<(&mut Transform, &Falling), With<ApplyFalling>>) {
    for (mut child_transform, falling) in child_query.iter_mut() {
        child_transform.translation.y = falling.v;
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
