use crate::constant::*;
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::states::GameState;
use super::actor::Actor;
use crate::actor::enemy::Enemy;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

pub fn spawn_slime(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    commands
        .spawn((
            Name::new("enemy"),
            StateScoped(GameState::InGame),
            Enemy,
            Actor {
                uuid: Uuid::new_v4(),
                cooltime: 0,
                life: 20,
                max_life: 20,
                latest_damage: 0,
                pointer: Vec2::ZERO,
            },
            AsepriteAnimationBundle {
                aseprite: aseprite,
                transform: Transform::from_translation(position.extend(5.0)),
                animation: Animation::default().with_tag("idle").with_speed(0.2),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(8.0),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(ENEMY_GROUP, Group::ALL),
        ))
        .with_children(|mut parent| {
            spawn_life_bar(&mut parent, &life_bar_locals);
        });
}
