use super::entity_depth::EntityDepth;
use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::collision::SENSOR_GROUPS;
use crate::entity::explosion::SpawnExplosion;
use crate::level::world::GameLevel;
use crate::level::world::LevelScoped;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Sensor;

/// 触れると爆発する性質です
#[derive(Component, Clone)]
pub struct Mine {
    actor_group: ActorGroup,
}

pub fn spawn_mine(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    actor_group: ActorGroup,
    level: &GameLevel,
    position: Vec2,
) {
    commands.spawn((
        StateScoped(GameState::InGame),
        LevelScoped(level.clone()),
        Mine { actor_group },
        AseSpriteAnimation {
            aseprite: asset_server.load("entity/explosive_mashroom.aseprite"),
            ..default()
        },
        EntityDepth::default(),
        Transform::from_translation(position.extend(0.0)),
        Sensor,
        Collider::ball(16.0),
        ActiveEvents::COLLISION_EVENTS,
        *SENSOR_GROUPS,
    ));
}

fn sensor(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mine_query: Query<(&Mine, &GlobalTransform)>,
    actor_query: Query<(Entity, &Actor, &Transform)>,
    mut spawn: EventWriter<SpawnExplosion>,
) {
    for collision_event in collision_events.read() {
        match identify(collision_event, &mine_query, &actor_query) {
            IdentifiedCollisionEvent::Started(mine_entity, actor_entity) => {
                let (mine, mine_transform) = mine_query.get(mine_entity).unwrap();
                let (_, target_actor, _) = actor_query.get(actor_entity).unwrap();
                if target_actor.actor_group != ActorGroup::Entity
                    && mine.actor_group != target_actor.actor_group
                {
                    commands.entity(mine_entity).despawn_recursive();
                    let position = mine_transform.translation().truncate();
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
