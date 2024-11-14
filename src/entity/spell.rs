use crate::entity::EntityDepth;
use crate::spell::SpellType;
use crate::spell_props::spell_to_props;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Component)]
pub struct SpellEntity(pub SpellType);

pub fn spawn_spell_entity(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    x: f32,
    y: f32,
    spell: SpellType,
) {
    let tx = x;
    let ty = y;
    let props = spell_to_props(spell);
    commands
        .spawn((
            Name::new(format!("spell {}", props.name)),
            StateScoped(GameState::InGame),
            SpellEntity(spell),
            EntityDepth,
            AsepriteSliceBundle {
                aseprite: assets.atlas.clone(),
                slice: "spell_frame".into(),
                transform: Transform::from_translation(Vec3::new(
                    tx + (random::<f32>() - 0.5) * 16.0,
                    ty + (random::<f32>() - 0.5) * 16.0,
                    0.0,
                )),
                ..default()
            },
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Fixed,
            Sensor,
            Collider::cuboid(8.0, 8.0),
            CollisionGroups::new(
                ENTITY_GROUP,
                ENTITY_GROUP | WALL_GROUP | PLAYER_INTERACTION_SENSOR_GROUP,
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                StateScoped(GameState::InGame),
                AsepriteSliceBundle {
                    aseprite: assets.atlas.clone(),
                    slice: props.icon.into(),
                    ..default()
                },
            ));
        });
}

// fn interaction(
//     mut spell_query: Query<(Entity, &mut SpellEntity)>,
//     mut player_query: Query<Entity, With<Player>>,
//     mut collision_events: EventReader<CollisionEvent>,
// ) {
//     for collision_event in collision_events.read() {
//         match collision_event {
//             CollisionEvent::Started(a, b, _) => {
//                 process_intersection_event(&spell_query, &player_query, a, b)
//                     || process_intersection_event(&spell_query, &player_query, b, a);
//             }
//             CollisionEvent::Stopped(a, b, _) => {}
//         }
//     }
// }

// fn process_intersection_event(
//     mut spell_query: &Query<(Entity, &mut SpellEntity)>,
//     mut player_query: &Query<&mut Player>,
//     a: &Entity,
//     b: &Entity,
// ) -> bool {
//     match (spell_query.get(*a), player_query.get(*b)) {
//         (Ok((entity, spell)), Ok(player)) => {
//             player.interaction_entity = Some(entity);
//             return true;
//         }
//         _ => {
//             return false;
//         }
//     }
// }

pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, _app: &mut App) {}
}
