// use crate::constant::*;
// use crate::controller::enemy::Enemy;
// use crate::entity::actor::{Actor, ActorFireState, ActorState};
// use crate::component::life::Life;
// use crate::component::entity_depth::EntityDepth;
// use crate::spell::SpellType;
// use crate::states::GameState;
// use crate::wand::{Wand, WandType};
// use bevy::prelude::*;
// use bevy_aseprite_ultra::prelude::*;
// use bevy_rapier2d::prelude::*;
// use uuid::*;

// #[derive(Component)]
// pub struct Buer;

// pub fn spawn_buer(commands: &mut Commands, aseprite: Handle<Aseprite>, position: Vec2) {
//     let mut slots = [None; MAX_SPELLS_IN_WAND];
//     slots[0] = Some(SpellType::MagicBolt);

//     commands.spawn((
//         Name::new("buer"),
//         StateScoped(GameState::InGame),
//         Enemy,
//         Buer,
//         Life {
//             life: 15,
//             max_life: 20,
//             amplitude: 0.0,
//         },
//         Actor {
//             uuid: Uuid::new_v4(),
//             spell_delay: 0,
//             pointer: Vec2::ZERO,
//             intensity: 0.0,
//             move_direction: Vec2::ZERO,
//             move_force: 0.0,
//             fire_state: ActorFireState::Idle,
//             group: ENEMY_GROUP,
//             filter: ENTITY_GROUP | WALL_GROUP | WITCH_GROUP,
//             current_wand: 0,
//             effects: default(),
//             wands: [
//                 Some(Wand {
//                     wand_type: WandType::CypressWand,
//                     slots,
//                     index: 0,
//                 }),
//                 None,
//                 None,
//                 None,
//             ],
//         },
//         ActorState::default(),
//         EntityDepth,
//         CounterAnimated,
//         AseSpriteAnimation {
//             aseprite: aseprite,
//             animation: Animation::default().with_tag("idle"),
//         },
//         Transform::from_translation(position.extend(0.0)),
//         (
//             RigidBody::Dynamic,
//             Collider::ball(48.0),
//             GravityScale(0.0),
//             LockedAxes::ROTATION_LOCKED,
//             Damping {
//                 linear_damping: 10.0,
//                 angular_damping: 1.0,
//             },
//             ExternalForce::default(),
//             ExternalImpulse::default(),
//             ActiveEvents::COLLISION_EVENTS,
//             CollisionGroups::new(
//                 ENEMY_GROUP,
//                 ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | WITCH_BULLET_GROUP | ENEMY_GROUP,
//             ),
//         ),
//     ));
// }
