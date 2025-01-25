use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::registry::Registry;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;

pub fn default_eyeball() -> Actor {
    Actor {
        extra: ActorExtra::Eyeball,
        actor_group: ActorGroup::Enemy,
        wands: Wand::single(Some(Spell::new("PurpleBolt"))),
        life: 25,
        max_life: 25,
        ..default()
    }
}

pub fn spawn_eyeball(
    mut commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    let actor_group = actor.actor_group;
    spawn_basic_actor(
        &mut commands,
        &registry,
        match actor_group {
            ActorGroup::Friend => registry.assets.eyeball_friend.clone(),
            ActorGroup::Enemy => registry.assets.eyeball.clone(),
            ActorGroup::Neutral => registry.assets.eyeball_friend.clone(),
            ActorGroup::Entity => registry.assets.eyeball_friend.clone(),
        },
        position,
        None,
        actor,
    )
}
