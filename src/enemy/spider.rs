use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::registry::*;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;

pub fn default_spider(registry: &Registry) -> Actor {
    let props = registry.get_actor_props(ActorType::Spider);
    Actor {
        extra: ActorExtra::Spider,
        actor_group: ActorGroup::Enemy,
        life: 40,
        max_life: 40,
        golds: 6,
        wands: Wand::from_vec(&props.wands),
        ..default()
    }
}

pub fn spawn_spider(
    mut commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    spawn_basic_actor(
        &mut commands,
        &registry,
        registry.assets.spider.clone(),
        position,
        None,
        actor,
    )
}
