use crate::entity::EntityDepth;
use crate::equipment::equipment_to_props;
use crate::inventory_item::InventoryItem;
use crate::spell_props::spell_to_props;
use crate::ui::interaction_marker::spawn_interaction_marker;
use crate::wand_props::wand_to_props;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct DroppedItemEntity {
    pub item_type: InventoryItem,
}

#[derive(Component)]
struct SpellSprites {
    swing: f32,
}

pub fn spawn_dropped_item(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    item_type: InventoryItem,
) {
    let icon = match item_type {
        InventoryItem::Spell(spell) => {
            let props = spell_to_props(spell);
            props.icon
        }
        InventoryItem::Wand(wand) => {
            let props = wand_to_props(wand);
            props.icon
        }
        InventoryItem::Equipment(equipment) => {
            let props = equipment_to_props(equipment);
            props.icon
        }
    };
    let name = match item_type {
        InventoryItem::Spell(spell) => {
            let props = spell_to_props(spell);
            props.name.en
        }
        InventoryItem::Wand(wand) => {
            let props = wand_to_props(wand);
            props.name.en
        }
        InventoryItem::Equipment(equipment) => {
            let props = equipment_to_props(equipment);
            props.name.en
        }
    };
    let frame_slice = match item_type {
        InventoryItem::Wand(_) => "empty", //"wand_frame",
        InventoryItem::Spell(_) => "spell_frame",
        InventoryItem::Equipment(_) => "empty",
    };
    let collider_width = match item_type {
        InventoryItem::Wand(_) => 16.0,
        _ => 8.0,
    };
    let swing = match item_type {
        InventoryItem::Spell(_) => 2.0,
        InventoryItem::Wand(_) => 0.0,
        InventoryItem::Equipment(_) => 0.0,
    };
    commands
        .spawn((
            Name::new(format!("dropped item {}", name)),
            StateScoped(GameState::InGame),
            DroppedItemEntity { item_type },
            EntityDepth,
            InheritedVisibility::default(),
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            GlobalTransform::default(),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            Collider::cuboid(collider_width, 8.0),
            CollisionGroups::new(
                DROPPED_ITEM_GROUP,
                ENTITY_GROUP | WALL_GROUP | PLAYER_INTERACTION_SENSOR_GROUP | DROPPED_ITEM_GROUP,
            ),
            Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
        ))
        .with_children(|mut parent| {
            spawn_interaction_marker(&mut parent, &assets, 14.0);

            parent
                .spawn((
                    SpellSprites { swing },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    GlobalTransform::default(),
                    InheritedVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        AseSpriteSlice {
                            aseprite: assets.atlas.clone(),
                            name: frame_slice.into(),
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));

                    parent.spawn((
                        AseSpriteSlice {
                            aseprite: assets.atlas.clone(),
                            name: icon.into(),
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0001),
                    ));
                });
        });
}

fn swing(mut query: Query<(&mut Transform, &SpellSprites)>, frame_count: Res<FrameCount>) {
    for (mut transform, sprite) in query.iter_mut() {
        transform.translation.y = (frame_count.0 as f32 * 0.05).sin() * sprite.swing;
    }
}

pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, swing.run_if(in_state(GameState::InGame)));
    }
}
