use crate::controller::player::Player;
use crate::entity::EntityDepth;
use crate::equipment::equipment_to_props;
use crate::inventory_item::InventoryItem;
use crate::se::{SECommand, SE};
use crate::spell_props::spell_to_props;
use crate::wand_props::wand_to_props;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct DroppedItemEntity {
    pub item_type: InventoryItem,
    pub price: u32,
}

#[derive(Component)]
struct SpellSprites {
    swing: f32,
    frame_count_offset: u32,
}

pub fn spawn_dropped_item(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    item_type: InventoryItem,
    price: u32,
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
            DroppedItemEntity { item_type, price },
            EntityDepth,
            InheritedVisibility::default(),
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            GlobalTransform::default(),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            Collider::cuboid(collider_width, 8.0),
            CollisionGroups::new(
                ENTITY_GROUP,
                ENTITY_GROUP
                    | WITCH_GROUP
                    | WITCH_BULLET_GROUP
                    | ENEMY_GROUP
                    | ENEMY_BULLET_GROUP
                    | WALL_GROUP,
            ),
            Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
            ActiveEvents::COLLISION_EVENTS,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpellSprites {
                        swing,
                        frame_count_offset: rand::random::<u32>() % 360,
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    GlobalTransform::default(),
                    InheritedVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2d(format!("{}", price)),
                        TextFont {
                            font: assets.dotgothic.clone(),
                            font_size: 24.0,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor(Color::WHITE),
                        Transform::from_xyz(0.0, 14.0, 1.0).with_scale(Vec3::new(0.3, 0.3, 1.0)),
                    ));

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
        transform.translation.y =
            ((sprite.frame_count_offset + frame_count.0) as f32 * 0.05).sin() * sprite.swing;
    }
}

fn collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    item_query: Query<&DroppedItemEntity>,
    mut player_query: Query<&mut Player>,
    mut global: EventWriter<SECommand>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _option) => {
                let _ = chat_start(
                    &mut commands,
                    a,
                    b,
                    &item_query,
                    &mut player_query,
                    &mut global,
                ) || chat_start(
                    &mut commands,
                    b,
                    a,
                    &item_query,
                    &mut player_query,
                    &mut global,
                );
            }
            CollisionEvent::Stopped(..) => {}
        }
    }
}

fn chat_start(
    commands: &mut Commands,
    a: &Entity,
    b: &Entity,
    item_query: &Query<&DroppedItemEntity>,
    player_query: &mut Query<&mut Player>,
    global: &mut EventWriter<SECommand>,
) -> bool {
    match (item_query.get(*a), player_query.get_mut(*b)) {
        (Ok(item), Ok(mut player)) => {
            if player.inventory.insert(item.item_type) {
                commands.entity(*a).despawn_recursive();
                global.send(SECommand::new(SE::PickUp));
                return true;
            }
        }
        _ => return false,
    }
    return false;
}

pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            collision
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.add_systems(Update, swing.run_if(in_state(GameState::InGame)));
    }
}
