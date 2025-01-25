use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::registry::Registry;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;

pub fn default_salamander() -> Actor {
    Actor {
        extra: ActorExtra::Salamander,
        actor_group: ActorGroup::Enemy,
        wands: Wand::single(Some(Spell::new("Fireball"))),
        life: 100,
        max_life: 100,
        golds: 10,
        ..default()
    }
}

pub fn spawn_salamander(
    mut commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    spawn_basic_actor(
        &mut commands,
        &registry,
        registry.assets.salamander.clone(),
        position,
        None,
        actor,
    )
}
