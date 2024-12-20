use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGold;
use crate::entity::actor::{Actor, ActorFireState, ActorGroup, ActorState};
use crate::entity::bullet::HomingTarget;
use crate::entity::life::{Life, LifeBeingSprite};
use crate::entity::EntityDepth;
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::inventory::Inventory;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::{Wand, WandSpell, WandType};
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
    spell: SpellType,
    move_force: f32,
    golds: u32,
    actor_group: ActorGroup,
) {
    let mut slots = [None; MAX_SPELLS_IN_WAND];
    slots[0] = Some(WandSpell {
        spell_type: spell,
        price: 0,
    });

    commands
        .spawn((
            Name::new(name.to_string()),
            StateScoped(GameState::InGame),
            DespawnWithGold { golds },
            marker,
            Actor {
                uuid: Uuid::new_v4(),
                pointer: Vec2::ZERO,
                intensity: 0.0,
                move_direction: Vec2::ZERO,
                move_force: move_force,
                fire_state: ActorFireState::Idle,
                fire_state_secondary: ActorFireState::Idle,
                current_wand: 0,
                effects: default(),
                actor_group,
                golds,
                inventory: Inventory::new(),
                equipments: [None; MAX_ITEMS_IN_EQUIPMENT],
                wands: [
                    Some(Wand::with_slots(WandType::CypressWand, slots)),
                    None,
                    None,
                    None,
                ],
            },
            ActorState::default(),
            EntityDepth,
            Life {
                life: 15,
                max_life: 15,
                amplitude: 0.0,
            },
            HomingTarget,
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
                    match actor_group {
                        ActorGroup::Enemy => ENEMY_GROUP,
                        ActorGroup::Player => WITCH_GROUP,
                    },
                    match actor_group {
                        ActorGroup::Enemy => WITCH_BULLET_GROUP,
                        ActorGroup::Player => ENEMY_BULLET_GROUP,
                    } | ENTITY_GROUP
                        | WALL_GROUP
                        | WITCH_GROUP
                        | ENEMY_GROUP,
                ),
            ),
        ))
        .with_children(|mut parent| {
            parent.spawn((
                LifeBeingSprite,
                AseSpriteAnimation {
                    aseprite,
                    animation: Animation::default().with_tag("idle"),
                },
            ));

            spawn_life_bar(&mut parent, &life_bar_locals);
        });
}
