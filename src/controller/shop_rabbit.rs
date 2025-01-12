use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::message::shop_rabbit;
use crate::message::too_few_golds;
use crate::message::SHOP_RABBIT;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::theater::Act;
use crate::theater::TheaterEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct ShopRabbit;

#[derive(Component)]
pub struct ShopRabbitSensor;

#[derive(Component)]
pub struct ShopRabbitOuterSensor;

fn collision_inner_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<&ShopRabbitSensor>,
    mut camera_query: Query<&mut GameCamera>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut speech_writer: EventWriter<TheaterEvent>,
    mut se: EventWriter<SEEvent>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Started(sensor_entity, player_entity) => {
                let mut camera = camera_query.single_mut();
                let mut actor = player_query.get_mut(player_entity).unwrap();
                let dept = actor.dept();
                if 0 < dept {
                    if actor.liquidate() {
                        camera.target = Some(sensor_entity);
                        se.send(SEEvent::new(SE::Register));
                        speech_writer.send(TheaterEvent::Play {
                            acts: vec![Act::Focus(sensor_entity), Act::Speech(shop_rabbit(dept))],
                        });
                    } else {
                        camera.target = Some(sensor_entity);
                        speech_writer.send(TheaterEvent::Play {
                            acts: vec![
                                Act::Focus(sensor_entity),
                                Act::Speech(too_few_golds(dept - actor.golds)),
                            ],
                        });
                    }
                } else {
                    camera.target = Some(sensor_entity);
                    speech_writer.send(TheaterEvent::Play {
                        acts: vec![
                            Act::Focus(sensor_entity),
                            Act::Speech(SHOP_RABBIT.to_string()),
                        ],
                    });
                }
            }
            IdentifiedCollisionEvent::Stopped(..) => {
                speech_writer.send(TheaterEvent::Quit);
            }
            _ => {}
        }
    }
}

fn collision_outer_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    mut camera_query: Query<&mut GameCamera>,
    sensor_query: Query<&ShopRabbitOuterSensor>,
    player_query: Query<&Actor, With<Player>>,
    mut speech_writer: EventWriter<TheaterEvent>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Stopped(..) => {
                let mut camera = camera_query.single_mut();
                camera.target = None;
                speech_writer.send(TheaterEvent::Quit);
            }
            _ => {}
        }
    }
}

pub struct ShopRabbitPlugin;

impl Plugin for ShopRabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (collision_inner_sensor, collision_outer_sensor).in_set(FixedUpdateGameActiveSet),
        );
    }
}
