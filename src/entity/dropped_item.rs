use crate::entity::EntityDepth;
use crate::inventory_item::InventoryItem;
use crate::spell::SpellType;
use crate::spell_props::spell_to_props;
use crate::wand::WandType;
use crate::wand_props::wand_to_props;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

pub enum DroppedItemType {
    Spell(SpellType),
    Wand(WandType),
    Lantern,
}

impl DroppedItemType {
    pub fn to_inventory_item(&self) -> InventoryItem {
        match self {
            DroppedItemType::Spell(spell) => InventoryItem::Spell(*spell),
            DroppedItemType::Lantern => InventoryItem::Lantern,
            DroppedItemType::Wand(wand) => InventoryItem::Wand(*wand),
        }
    }
}

#[derive(Component)]
pub struct DroppedItemEntity {
    pub item_type: DroppedItemType,
    pub interaction_marker: bool,
}

#[derive(Component)]
struct SpellSprites {
    swing: f32,
}

#[derive(Component)]
struct InteractionMarker;

pub fn spawn_dropped_item(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    x: f32,
    y: f32,
    item_type: DroppedItemType,
) {
    let tx = x;
    let ty = y;
    let icon = match item_type {
        DroppedItemType::Spell(spell) => {
            let props = spell_to_props(spell);
            props.icon
        }
        DroppedItemType::Wand(wand) => {
            let props = wand_to_props(wand);
            props.icon
        }
        DroppedItemType::Lantern => "lantern",
    };
    let name = match item_type {
        DroppedItemType::Spell(spell) => {
            let props = spell_to_props(spell);
            props.name.en
        }
        DroppedItemType::Wand(wand) => {
            let props = wand_to_props(wand);
            props.name.en
        }
        DroppedItemType::Lantern => "lantern",
    };
    let frame_slice = match item_type {
        DroppedItemType::Wand(_) => "empty", //"wand_frame",
        DroppedItemType::Spell(_) => "spell_frame",
        DroppedItemType::Lantern => "empty",
    };
    let collider_width = match item_type {
        DroppedItemType::Wand(_) => 16.0,
        _ => 8.0,
    };
    let swing = match item_type {
        DroppedItemType::Spell(_) => 2.0,
        DroppedItemType::Wand(_) => 0.0,
        DroppedItemType::Lantern => 0.0,
    };
    commands
        .spawn((
            Name::new(format!("dropped item {}", name)),
            StateScoped(GameState::InGame),
            DroppedItemEntity {
                item_type,
                interaction_marker: false,
            },
            EntityDepth,
            InheritedVisibility::default(),
            Transform::from_translation(Vec3::new(
                tx + (random::<f32>() - 0.5) * 16.0,
                ty + (random::<f32>() - 0.5) * 16.0,
                0.0,
            )),
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
        .with_children(|parent| {
            parent
                .spawn((
                    SpellSprites { swing },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    GlobalTransform::default(),
                    InheritedVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.atlas.clone(),
                            slice: frame_slice.into(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.atlas.clone(),
                            slice: icon.into(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0001),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        InteractionMarker,
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.atlas.clone(),
                            transform: Transform::from_xyz(0.0, 14.0, 0.0002),
                            slice: "interactive".into(),
                            visibility: Visibility::Hidden,
                            sprite: Sprite {
                                color: Color::hsla(0.0, 0.0, 1.0, 0.2),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn swing(mut query: Query<(&mut Transform, &SpellSprites)>, frame_count: Res<FrameCount>) {
    for (mut transform, sprite) in query.iter_mut() {
        transform.translation.y = (frame_count.0 as f32 * 0.05).sin() * sprite.swing;
    }
}

fn update_interactive_marker_visibility(
    spell: Query<&DroppedItemEntity>,
    sprites: Query<&Parent, With<SpellSprites>>,
    mut marker_query: Query<(&Parent, &mut Visibility), With<InteractionMarker>>,
) {
    for (parent, mut visibility) in marker_query.iter_mut() {
        if let Ok(parent) = sprites.get(parent.get()) {
            if let Ok(spell) = spell.get(parent.get()) {
                *visibility = if spell.interaction_marker {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (swing, update_interactive_marker_visibility).run_if(in_state(GameState::InGame)),
        );
    }
}
