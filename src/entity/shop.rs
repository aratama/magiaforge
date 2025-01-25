use crate::actor::Actor;
use crate::collision::ENTITY_GROUPS;
use crate::collision::HIDDEN_WALL_GROUPS;
use crate::collision::SENSOR_GROUPS;
use crate::component::entity_depth::EntityDepth;
use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::controller::player::Player;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::interpreter::Cmd;
use crate::interpreter::InterpreterEvent;
use crate::message::PAY_FIRST;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::registry::Registry;
use crate::se::SEEvent;

use crate::se::BUS;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct ShopDoorSensor {
    open: bool,
}

#[derive(Component)]
struct ShopDoor {
    sign: f32,
    state: f32,
}

pub fn spawn_shop_door(commands: &mut Commands, registry: &Registry, position: Vec2) {
    commands
        .spawn((
            // ドアを開くセンサー
            ShopDoorSensor { open: false },
            StateScoped(GameState::InGame),
            Sensor,
            Collider::cuboid(TILE_SIZE * 2.0, TILE_SIZE * 3.5),
            Transform::from_translation(Vec3::new(position.x + TILE_HALF, position.y, 0.0)),
            EntityDepth::new(),
            ActiveEvents::COLLISION_EVENTS,
            *SENSOR_GROUPS,
            Visibility::default(),
        ))
        .with_children(|builder| {
            // 商品が外に出ないようにする壁
            builder.spawn((
                RigidBody::Fixed,
                Collider::cuboid(TILE_SIZE * 1.0, TILE_SIZE * 1.5),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ActiveEvents::COLLISION_EVENTS,
                *HIDDEN_WALL_GROUPS,
            ));

            // 左側のドア
            builder.spawn((
                ShopDoor {
                    sign: -1.0,
                    state: 0.0,
                },
                RigidBody::KinematicPositionBased,
                Collider::cuboid(8.0, 10.0),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                LockedAxes::ROTATION_LOCKED,
                ActiveCollisionTypes::DYNAMIC_KINEMATIC,
                *ENTITY_GROUPS,
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: "door_left".into(),
                },
            ));

            // 右側のドア
            builder.spawn((
                ShopDoor {
                    sign: 1.0,
                    state: 0.0,
                },
                RigidBody::KinematicPositionBased,
                Collider::cuboid(8.0, 10.0),
                Transform::from_translation(Vec3::new(TILE_SIZE, 0.0, 0.0)),
                LockedAxes::ROTATION_LOCKED,
                ActiveCollisionTypes::DYNAMIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
                *ENTITY_GROUPS,
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: "door_right".into(),
                },
            ));
        });
}

fn sensor(
    mut collision_events: EventReader<CollisionEvent>,
    shop_rabbit_query: Query<Entity, With<ShopRabbit>>,
    mut sensor_query: Query<(&mut ShopDoorSensor, &Transform), Without<ShopRabbit>>,
    player_query: Query<&Actor, (With<Player>, Without<ShopRabbit>)>,
    mut speech_writer: EventWriter<InterpreterEvent>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Started(sensor_entity, player_entity) => {
                let (mut sensor, sensor_transform) = sensor_query.get_mut(sensor_entity).unwrap();
                let actor = player_query.get(player_entity).unwrap();
                if 0 < actor.dept() {
                    if let Ok(shop_rabbit_entity) = shop_rabbit_query.get_single() {
                        sensor.open = false;
                        speech_writer.send(InterpreterEvent::Play {
                            commands: vec![
                                Cmd::Focus(shop_rabbit_entity),
                                Cmd::Speech(PAY_FIRST.to_string()),
                            ],
                        });
                    }
                } else {
                    sensor.open = true;
                    se_writer.send(SEEvent::pos(BUS, sensor_transform.translation.truncate()));
                }
            }
            IdentifiedCollisionEvent::Stopped(sensor_entity, _) => {
                let (mut sensor, _) = sensor_query.get_mut(sensor_entity).unwrap();
                sensor.open = false;
            }
            _ => {}
        }
    }
}

fn update_door_position(
    sensor_query: Query<&ShopDoorSensor>,
    mut door_query: Query<(&Parent, &mut ShopDoor, &mut Transform)>,
) {
    for (parent, mut door, mut transform) in door_query.iter_mut() {
        let sensor = sensor_query.get(parent.get()).unwrap();
        let delta = if sensor.open { 0.1 } else { -0.1 };
        let offset = if door.sign == -1.0 { 0.0 } else { TILE_SIZE } - TILE_HALF;
        door.state = (door.state + delta).max(0.0).min(1.0);
        transform.translation.x = offset + door.sign * door.state * TILE_SIZE;
    }
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (sensor, update_door_position).in_set(FixedUpdateGameActiveSet),
        );
    }
}
