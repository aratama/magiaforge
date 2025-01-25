use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::interpreter::Cmd;
use crate::interpreter::InterpreterEvent;
use crate::physics::identify;
use crate::physics::identify_item;
use crate::physics::IdentifiedCollisionEvent;
use crate::physics::IdentifiedCollisionItem;
use crate::registry::Registry;
use crate::set::FixedUpdateInGameSet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MessageRabbit {
    pub senario: String,
}

impl MessageRabbit {
    pub fn new(senario: &str) -> Self {
        Self {
            senario: senario.to_string(),
        }
    }
}

/// 呪文一覧のウサギは特別な処理があるので、区別できるようにするマーカー
#[derive(Component)]
pub struct SpellListRabbit;

#[derive(Component)]
pub struct MessageRabbitInnerSensor;

#[derive(Component)]
pub struct MessageRabbitOuterSensor;

fn collision_inner_sensor(
    registry: Registry,
    mut collision_events: EventReader<CollisionEvent>,
    rabbit_query: Query<&MessageRabbit>,
    sensor_query: Query<&Parent, With<MessageRabbitInnerSensor>>,
    mut camera_query: Query<&mut GameCamera>,
    player_query: Query<&Actor, (With<Player>, With<Witch>)>,
    mut speech_writer: EventWriter<InterpreterEvent>,
) {
    for collision_event in collision_events.read() {
        match identify_item(collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionItem::Started(parent, _, _, _) => {
                let mut camera = camera_query.single_mut();

                if camera.target.is_some() {
                    continue;
                }

                let rabbit_entity = parent.get();
                let rabbit = rabbit_query.get(rabbit_entity).unwrap();
                camera.target = Some(rabbit_entity);

                let event = registry.get_senario(&rabbit.senario);
                let mut messages = event.clone();
                messages.insert(0, Cmd::Focus(rabbit_entity));
                messages.push(Cmd::Close);
                speech_writer.send(InterpreterEvent::Play { commands: messages });
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
    mut speech_writer: EventWriter<InterpreterEvent>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Stopped(..) => {
                let mut camera = camera_query.single_mut();
                camera.target = None;
                speech_writer.send(InterpreterEvent::Quit);
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
            (collision_inner_sensor, collision_outer_sensor).in_set(FixedUpdateInGameSet),
        );
    }
}
