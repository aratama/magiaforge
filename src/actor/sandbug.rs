use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;

/// Sandbugは敵のアクターグループですが攻撃を行いません
#[derive(Component)]
pub struct Sandbag {
    home: Vec2,
}

impl Sandbag {
    pub fn new(home: Vec2) -> Self {
        Self { home }
    }
}

pub fn default_sandbag() -> Actor {
    Actor {
        extra: ActorExtra::Sandbag,
        actor_group: ActorGroup::Neutral,
        wands: Wand::single(Some(Spell::new("Jump"))),
        life: 10000,
        max_life: 10000,
        ..default()
    }
}

pub fn spawn_sandbag(
    mut commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    spawn_basic_actor(
        &mut commands,
        &registry,
        registry.assets.sandbug.clone(),
        position,
        None,
        actor,
    )
}

fn go_back(mut query: Query<(&mut Actor, &Transform, &Sandbag)>) {
    for (mut actor, witch_transform, dummy) in query.iter_mut() {
        let diff = dummy.home - witch_transform.translation.truncate();
        actor.move_direction = if 8.0 < diff.length() {
            diff.normalize_or_zero()
        } else {
            Vec2::ZERO
        };
    }
}

pub struct SandbagPlugin;

impl Plugin for SandbagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (go_back).in_set(FixedUpdateGameActiveSet));
    }
}
