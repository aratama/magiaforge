use crate::actor::Actor;
use crate::collision::*;
use crate::component::counter::Counter;
use crate::component::entity_depth::EntityDepth;
use crate::controller::player::Player;
use crate::inventory::InventoryItem;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::PICK_UP;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::new_spell::spawn_new_spell;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

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
    registry: &Registry,
    position: Vec2,
    item: &InventoryItem,
) {
    let item_type = &item.spell;
    let props = registry.get_spell_props(&item_type);
    let icon = props.icon.clone();
    let name = props.name.en.clone();
    let frame_slice = "spell_frame";
    let collider_width = 8.0;
    let swing = 2.0;
    commands
        .spawn((
            Name::new(format!("dropped item {}", name)),
            StateScoped(GameState::InGame),
            DroppedItemEntity { item: item.clone() },
            EntityDepth::new(),
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            GlobalTransform::default(),
            Visibility::default(),
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 1.0,
                    angular_damping: 1.0,
                },
                Collider::cuboid(collider_width, 8.0),
                *DROPPED_ITEM_GROUPS,
                ActiveEvents::COLLISION_EVENTS,
                ExternalForce::default(),
                ExternalImpulse::default(),
            ),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Counter::up(0),
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
                                font: registry.assets.noto_sans_jp.clone(),
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
                            aseprite: registry.assets.atlas.clone(),
                            name: frame_slice.into(),
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0001),
                    ));

                    parent.spawn((
                        AseSpriteSlice {
                            aseprite: registry.assets.atlas.clone(),
                            name: icon.into(),
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                });
        });
}

fn swing(mut query: Query<(&mut Transform, &SpellSprites, &Counter)>) {
    for (mut transform, sprite, counter) in query.iter_mut() {
        transform.translation.y =
            ((sprite.frame_count_offset as i32 + counter.count) as f32 * 0.05).sin() * sprite.swing;
    }
}

fn pickup_dropped_item(
    mut commands: Commands,
    registry: Registry,
    mut collision_events: EventReader<CollisionEvent>,
    item_query: Query<&DroppedItemEntity>,
    mut player_query: Query<(&mut Actor, &Player)>,
    mut se: EventWriter<SEEvent>,
    mut time: ResMut<NextState<TimeState>>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &item_query, &player_query) {
            IdentifiedCollisionEvent::Started(item_entity, player_entity) => {
                info!("pickup_dropped_item ");

                let item = item_query.get(item_entity).unwrap();
                let (mut actor, player) = player_query.get_mut(player_entity).unwrap();
                if actor.inventory.insert(item.item.clone()) {
                    commands.entity(item_entity).despawn_recursive();

                    se.send(SEEvent::new(PICK_UP));

                    let InventoryItem {
                        spell,
                        price: _,
                    } = &item.item;

                    if !player.discovered_spells.contains(&spell) {
                        spawn_new_spell(
                            &mut commands,
                            &registry,
                            &mut time,
                            spell.clone(),
                            &mut se,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, app: &mut App) {
        // swing は FixedUpdate ではなく Update でもいいですが、
        // 分けるのも面倒な割に意味がないので FixedUpdate に含めています
        app.add_systems(
            FixedUpdate,
            (pickup_dropped_item, swing).in_set(FixedUpdateGameActiveSet),
        );
    }
}
