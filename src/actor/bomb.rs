use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::LifeBeingSprite;
use crate::collision::*;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::falling::Falling;
use crate::entity::explosion::SpawnExplosion;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ActorSpriteGroup;

#[derive(Default, Component, Reflect)]
struct Bomb;

pub fn default_bomb() -> Actor {
    Actor {
        life: 10,
        max_life: 10,
        extra: ActorExtra::Bomb,
        ..default()
    }
}

pub fn spawn_bomb(
    commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    let aseprite = registry.assets.bomb.clone();
    commands
        .spawn((
            Name::new("bomb"),
            actor,
            Bomb,
            Counter::up(0),
            Transform::from_translation(position.extend(0.0)),
            Falling,
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping::default(),
                Collider::ball(6.0),
                *ENTITY_GROUPS,
                ExternalImpulse::default(),
            ),
        ))
        .with_children(move |parent| {
            parent.spawn(ActorSpriteGroup).with_child((
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: aseprite.clone(),
                    animation: "default".into(), // TODO
                },
            ));
        })
        .id()
}

fn explode_bomb(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Actor, &Counter), With<Bomb>>,
    mut explosion_writer: EventWriter<SpawnExplosion>,
) {
    for (entity, transform, life, counter) in query.iter() {
        if life.life <= 0 || 180 <= counter.count {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();

            explosion_writer.send(SpawnExplosion {
                position,
                radius: 60.0,
                impulse: 100000.0,
                damage: 100,
            });
        }
    }
}

fn set_bomb_rotation(
    mut query: Query<(&Children, &Transform), With<Bomb>>,
    mut sprite_query: Query<&mut Transform, (With<AseSpriteAnimation>, Without<Bomb>)>, // TODO
) {
    for (children, transform) in query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut child) = sprite_query.get_mut(*child) {
                child.rotation = Quat::from_rotation_z(transform.translation.x * -0.1);
            }
        }
    }
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (explode_bomb, set_bomb_rotation).in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Bomb>();
    }
}
