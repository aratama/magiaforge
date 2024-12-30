use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::life::Life;
use crate::entity::EntityDepth;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use super::counter::Counter;

#[derive(Component)]
pub struct DroppedItemEntity {
    item: InventoryItem,
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
    item: InventoryItem,
) {
    let item_type = item.item_type;
    let icon = match item_type {
        InventoryItemType::Spell(spell) => spell.to_props().icon,
        InventoryItemType::Equipment(equipment) => equipment.to_props().icon,
    };
    let name = match item_type {
        InventoryItemType::Spell(spell) => spell.to_props().name.en,
        InventoryItemType::Equipment(equipment) => equipment.to_props().name.en,
    };
    let frame_slice = match item_type {
        InventoryItemType::Spell(_) if 0 < item.price => "spell_frame_yellow",
        InventoryItemType::Spell(_) => "spell_frame",
        InventoryItemType::Equipment(_) => "empty",
    };
    let collider_width = match item_type {
        _ => 8.0,
    };
    let swing = match item_type {
        InventoryItemType::Spell(_) => 2.0,
        InventoryItemType::Equipment(_) => 0.0,
    };
    commands
        .spawn((
            Name::new(format!("dropped item {}", name)),
            StateScoped(GameState::InGame),
            DroppedItemEntity { item },
            EntityDepth,
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            GlobalTransform::default(),
            Visibility::default(),
            Life {
                life: 300,
                max_life: 300,
                amplitude: 0.0,
            },
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 1.0,
                    angular_damping: 1.0,
                },
                Collider::cuboid(collider_width, 8.0),
                CollisionGroups::new(
                    DROPPED_ITEM_GROUP,
                    DROPPED_ITEM_GROUP
                        | ENTITY_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | WALL_GROUP
                        | DOOR_GROUP
                        | RABBIT_GROUP,
                ),
                ActiveEvents::COLLISION_EVENTS,
                ExternalForce::default(),
                ExternalImpulse::default(),
            ),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Counter::new(),
                    SpellSprites {
                        swing,
                        frame_count_offset: rand::random::<u32>() % 360,
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    GlobalTransform::default(),
                    Visibility::default(),
                ))
                .with_children(|parent| {
                    if 0 < item.price {
                        parent.spawn((
                            // Text2dはVisibilityを必須としているため、その親にもVisibility::default(),を設定しないと警告が出る
                            Text2d(format!("{}", item.price)),
                            TextFont {
                                font: assets.noto_sans_jp.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Transform::from_xyz(0.0, 14.0, 1.0)
                                .with_scale(Vec3::new(0.3, 0.3, 1.0)),
                        ));
                    }

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

fn swing(mut query: Query<(&mut Transform, &SpellSprites, &Counter)>) {
    for (mut transform, sprite, counter) in query.iter_mut() {
        transform.translation.y =
            ((sprite.frame_count_offset + counter.count) as f32 * 0.05).sin() * sprite.swing;
    }
}

fn collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    item_query: Query<&DroppedItemEntity>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut global: EventWriter<SEEvent>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &item_query, &player_query) {
            IdentifiedCollisionEvent::Started(item_entity, player_entity) => {
                let item = item_query.get(item_entity).unwrap();
                let mut actor = player_query.get_mut(player_entity).unwrap();
                if actor.inventory.insert(item.item) {
                    commands.entity(item_entity).despawn_recursive();
                    global.send(SEEvent::new(SE::PickUp));
                }
            }
            _ => {}
        }
    }
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
