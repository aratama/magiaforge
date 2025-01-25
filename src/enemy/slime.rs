use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::registry::Registry;
use crate::wand::Wand;
use bevy::prelude::*;

pub fn default_slime(registry: &Registry) -> Actor {
    let props = registry.get_actor_props(ActorType::Slime);
    Actor {
        extra: ActorExtra::Slime,
        actor_group: ActorGroup::Enemy,
        wands: Wand::from_vec(&props.wands),
        life: 15,
        max_life: 15,
        ..default()
    }
}

pub fn spawn_slime(
    mut commands: &mut Commands,
    registry: &Registry,
    actor: Actor,
    position: Vec2,
    owner: Option<Entity>,
) -> Entity {
    let actor_group = actor.actor_group;
    spawn_basic_actor(
        &mut commands,
        &registry,
        match actor_group {
            ActorGroup::Friend => registry.assets.friend_slime.clone(),
            ActorGroup::Enemy => registry.assets.slime.clone(),
            ActorGroup::Neutral => registry.assets.friend_slime.clone(),
            ActorGroup::Entity => registry.assets.friend_slime.clone(),
        },
        position,
        owner,
        actor,
    )
}
