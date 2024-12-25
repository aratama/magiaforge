use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::physics::{identify, identify_item, IdentifiedCollisionEvent, IdentifiedCollisionItem};
use crate::theater::{Act, TheaterEvent};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MessageRabbit {
    pub messages: Vec<Act>,
}

#[derive(Component)]
pub struct MessageRabbitInnerSensor;

#[derive(Component)]
pub struct MessageRabbitOuterSensor;

fn collision_inner_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    rabbit_query: Query<&MessageRabbit>,
    sensor_query: Query<&Parent, With<MessageRabbitInnerSensor>>,
    mut camera_query: Query<&mut GameCamera>,
    player_query: Query<&Actor, With<Player>>,
    mut speech_writer: EventWriter<TheaterEvent>,
) {
    for collision_event in collision_events.read() {
        match identify_item(collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionItem::Started(parent, _, sensor_entity, _) => {
                let mut camera = camera_query.single_mut();
                let rabbit_entity = parent.get();
                let rabbit = rabbit_query.get(rabbit_entity).unwrap();
                camera.target = Some(sensor_entity);

                let mut messages = rabbit.messages.clone();
                messages.insert(0, Act::Focus(rabbit_entity));
                messages.push(Act::Close);
                speech_writer.send(TheaterEvent::Play { acts: messages });
            }
            _ => {}
        }
    }
}

fn collision_outer_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    mut camera_query: Query<&mut GameCamera>,
    sensor_query: Query<&MessageRabbitOuterSensor>,
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

pub struct MessageRabbitPlugin;

impl Plugin for MessageRabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (collision_inner_sensor, collision_outer_sensor)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
