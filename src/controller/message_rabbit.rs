use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::constant::GameSenarios;
use crate::constant::SenarioType;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::witch::Witch;
use crate::physics::identify;
use crate::physics::identify_item;
use crate::physics::IdentifiedCollisionEvent;
use crate::physics::IdentifiedCollisionItem;
use crate::set::FixedUpdateInGameSet;
use crate::theater::Act;
use crate::theater::TheaterEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MessageRabbit {
    pub senario: SenarioType,
}

/// 呪文一覧のウサギは特別な処理があるので、区別できるようにするマーカー
#[derive(Component)]
pub struct SpellListRabbit;

#[derive(Component)]
pub struct MessageRabbitInnerSensor;

#[derive(Component)]
pub struct MessageRabbitOuterSensor;

fn collision_inner_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    rabbit_query: Query<&MessageRabbit>,
    sensor_query: Query<&Parent, With<MessageRabbitInnerSensor>>,
    mut camera_query: Query<&mut GameCamera>,
    player_query: Query<&Actor, (With<Player>, With<Witch>)>,
    mut speech_writer: EventWriter<TheaterEvent>,

    assets: Res<GameAssets>,
    ron: Res<Assets<GameSenarios>>,
) {
    let senarios = ron.get(assets.senario.id()).unwrap();

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

                let event = rabbit.senario.to_acts(&senarios);
                let mut messages = event.clone();
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
            (collision_inner_sensor, collision_outer_sensor).in_set(FixedUpdateInGameSet),
        );
    }
}
