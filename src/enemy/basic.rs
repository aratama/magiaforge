use crate::bullet_type::BulletType;
use crate::constant::*;
use crate::controller::enemy::Enemy;
use crate::entity::actor::ActorMoveState;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::breakable::{Breakable, BreakableSprite};
use crate::entity::EntityDepth;
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

pub fn spawn_basic_enemy<T: Component>(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
    marker: T,
    name: &str,
    bullet_type: BulletType,
    bullet_lifetime: u32,
    reload_speed: i32,
) {
    commands
        .spawn((
            Name::new(name.to_string()),
            StateScoped(GameState::InGame),
            Enemy,
            marker,
            Actor {
                uuid: Uuid::new_v4(),
                cooltime: 0,
                reload_speed,
                mana: 1000,
                max_mana: 1000,
                life: 20,
                max_life: 20,
                pointer: Vec2::ZERO,
                intensity: 0.0,
                move_state: ActorMoveState::Idle,
                fire_state: ActorFireState::Idle,
                online: false,
                group: ENEMY_GROUP,
                filter: ENTITY_GROUP | WALL_GROUP | WITCH_GROUP,
                bullet_type,
            },
            EntityDepth,
            Breakable {
                life: 0,
                amplitude: 0.0,
            },
            Transform::from_translation(position.extend(5.0)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            (
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
                CollisionGroups::new(
                    ENEMY_GROUP,
                    ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | WITCH_BULLET_GROUP | ENEMY_GROUP,
                ),
            ),
        ))
        .with_children(|mut parent| {
            parent.spawn((
                BreakableSprite,
                AsepriteAnimationBundle {
                    aseprite: aseprite,
                    animation: Animation::default().with_tag("idle"),
                    ..default()
                },
            ));

            spawn_life_bar(&mut parent, &life_bar_locals);
        });
}
