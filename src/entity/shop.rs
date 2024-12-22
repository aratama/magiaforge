use crate::{
    asset::GameAssets,
    constant::{
        DOOR_GROUP, ENEMY_BULLET_GROUP, ENEMY_GROUP, ENTITY_GROUP, RABBIT_GROUP, SENSOR_GROUP,
        TILE_HALF, TILE_SIZE, WITCH_BULLET_GROUP, WITCH_GROUP,
    },
    controller::player::Player,
    language::Dict,
    se::{SEEvent, SE},
    speech_bubble::SpeechEvent,
    states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_rapier2d::prelude::*;

use super::{actor::Actor, EntityDepth};

#[derive(Component)]
struct ShopDoorSensor {
    open: bool,
}

#[derive(Component)]
struct ShopDoor {
    sign: f32,
    state: f32,
}

pub fn spawn_shop_door(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            // ドアを開くセンサー
            ShopDoorSensor { open: false },
            StateScoped(GameState::InGame),
            Sensor,
            Collider::cuboid(TILE_SIZE * 2.0, TILE_SIZE * 3.5),
            Transform::from_translation(Vec3::new(position.x + TILE_HALF, position.y, 0.0)),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(SENSOR_GROUP, WITCH_GROUP),
        ))
        .with_children(|builder| {
            // 商品が外に出ないようにする壁
            builder.spawn((
                RigidBody::Fixed,
                Collider::cuboid(TILE_SIZE * 1.0, TILE_SIZE * 1.5),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(DOOR_GROUP, ENTITY_GROUP | RABBIT_GROUP),
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
                EntityDepth,
                LockedAxes::ROTATION_LOCKED,
                ActiveCollisionTypes::DYNAMIC_KINEMATIC,
                CollisionGroups::new(
                    ENTITY_GROUP,
                    ENTITY_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | ENEMY_GROUP
                        | ENEMY_BULLET_GROUP,
                ),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
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
                EntityDepth,
                LockedAxes::ROTATION_LOCKED,
                ActiveCollisionTypes::DYNAMIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
                CollisionGroups::new(
                    ENTITY_GROUP,
                    ENTITY_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | ENEMY_GROUP
                        | ENEMY_BULLET_GROUP,
                ),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "door_right".into(),
                },
            ));
        });
}

fn sensor(
    mut collision_events: EventReader<CollisionEvent>,
    mut door_query: Query<(&mut ShopDoorSensor, &Transform)>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut speech_writer: EventWriter<SpeechEvent>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, ..) => {
                let _ = enter(
                    a,
                    b,
                    &mut door_query,
                    &mut player_query,
                    &mut speech_writer,
                    &mut se_writer,
                ) || enter(
                    b,
                    a,
                    &mut door_query,
                    &mut player_query,
                    &mut speech_writer,
                    &mut se_writer,
                );
            }
            CollisionEvent::Stopped(a, b, ..) => {
                let _ = exit(a, b, &mut door_query, &mut player_query, &mut speech_writer)
                    || exit(b, a, &mut door_query, &mut player_query, &mut speech_writer);
            }
        }
    }
}

fn enter(
    a: &Entity,
    b: &Entity,
    sensor_query: &mut Query<(&mut ShopDoorSensor, &Transform)>,
    player_query: &mut Query<&mut Actor, With<Player>>,
    speech_writer: &mut EventWriter<SpeechEvent>,
    se_writer: &mut EventWriter<SEEvent>,
) -> bool {
    if let Ok((mut sensor, sensor_transform)) = sensor_query.get_mut(*a) {
        if let Ok(actor) = player_query.get(*b) {
            info!("enter");
            if 0 < actor.dept() {
                sensor.open = false;
                speech_writer.send(SpeechEvent::Speech(Dict {
                    ja: "おいおい、代金を払ってから行ってくれ".to_string(),
                    en: "Hey Hey, pay first before you go".to_string(),
                }));
            } else {
                sensor.open = true;
                se_writer.send(SEEvent::pos(
                    SE::Bus,
                    sensor_transform.translation.truncate(),
                ));
            }
            return true;
        }
    }
    return false;
}

fn exit(
    a: &Entity,
    b: &Entity,
    sensor_query: &mut Query<(&mut ShopDoorSensor, &Transform)>,
    player_query: &mut Query<&mut Actor, With<Player>>,
    speech_writer: &mut EventWriter<SpeechEvent>,
) -> bool {
    if let Ok((mut sensor, _)) = sensor_query.get_mut(*a) {
        if let Ok(_) = player_query.get(*b) {
            sensor.open = false;
            speech_writer.send(SpeechEvent::Close);
            return true;
        }
    }
    return false;
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
            (sensor, update_door_position)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
