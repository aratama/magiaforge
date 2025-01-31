use crate::{
    actor::Actor,
    collision::SENSOR_GROUPS,
    entity::explosion::SpawnExplosion,
    physics::{identify, IdentifiedCollisionEvent},
    set::FixedUpdateGameActiveSet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, Sensor};

/// 触れると爆発する性質です
#[derive(Component, Clone)]
pub struct Mine;

pub fn spawn_mine_child(builder: &mut ChildBuilder) {
    builder.spawn((
        Mine,
        Sensor,
        Collider::ball(16.0),
        ActiveEvents::COLLISION_EVENTS,
        *SENSOR_GROUPS,
    ));
}

fn sensor(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mine_query: Query<(&Parent, &Mine)>,
    actor_query: Query<(Entity, &Actor, &Transform)>,
    mut spawn: EventWriter<SpawnExplosion>,
) {
    for collision_event in collision_events.read() {
        match identify(collision_event, &mine_query, &actor_query) {
            IdentifiedCollisionEvent::Started(mine_entity, actor_entity) => {
                let (parent, _mine) = mine_query.get(mine_entity).unwrap();
                let (mine_entity, mine_actor, mine_transform) =
                    actor_query.get(parent.get()).unwrap();
                let (_, target_actor, _) = actor_query.get(actor_entity).unwrap();
                if mine_actor.actor_group != target_actor.actor_group {
                    commands.entity(mine_entity).despawn_recursive();
                    let position = mine_transform.translation.truncate();
                    spawn.send(SpawnExplosion {
                        position,
                        radius: 32.0,
                        impulse: 1000.0,
                        damage: 10,
                    });
                }
            }
            _ => {}
        }
    }
}

pub struct ExplosiveMashroomPlugin;

impl Plugin for ExplosiveMashroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (sensor).in_set(FixedUpdateGameActiveSet));
    }
}
