use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::language::Dict;
use crate::speech_bubble::SpeechEvent;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct TrainingRabbit;

#[derive(Component)]
pub struct TrainingRabbitSensor;

#[derive(Component)]
struct RabbitOuterSensor;

fn collision_inner_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<&TrainingRabbitSensor>,
    mut camera_query: Query<&mut GameCamera>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut speech_writer: EventWriter<SpeechEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _option) => {
                let _ = chat_start(
                    a,
                    b,
                    &mut camera_query,
                    &sensor_query,
                    &mut player_query,
                    &mut speech_writer,
                ) || chat_start(
                    b,
                    a,
                    &mut camera_query,
                    &sensor_query,
                    &mut player_query,
                    &mut speech_writer,
                );
            }
            CollisionEvent::Stopped(a, b, _option) => {
                let _ = chat_end(a, b, &sensor_query, &player_query, &mut speech_writer)
                    || chat_end(b, a, &sensor_query, &player_query, &mut speech_writer);
            }
        }
    }
}

fn chat_start(
    sensor_entity: &Entity,
    player_entity: &Entity,
    camera_query: &mut Query<&mut GameCamera>,
    sensor_query: &Query<&TrainingRabbitSensor>,
    player_query: &mut Query<&mut Actor, With<Player>>,
    speech_writer: &mut EventWriter<SpeechEvent>,
) -> bool {
    let mut camera = camera_query.single_mut();
    if let Ok(_) = sensor_query.get(*sensor_entity) {
        if let Ok(_) = player_query.get_mut(*player_entity) {
            camera.target = Some(*sensor_entity);
            speech_writer.send(SpeechEvent::Speech {
                speaker: *sensor_entity,
                text: Dict {
                    ja: "キミも強くなりたいのかい？\nここで練習していくといい".to_string(),
                    en: "Do you want to be strong too?\nIt's good to practice here".to_string(),
                },
            });
            return true;
        }
    }
    return false;
}

fn chat_end(
    a: &Entity,
    b: &Entity,
    sensor_query: &Query<&TrainingRabbitSensor>,
    player_query: &Query<&mut Actor, With<Player>>,
    speech_writer: &mut EventWriter<SpeechEvent>,
) -> bool {
    if sensor_query.contains(*a) && player_query.contains(*b) {
        speech_writer.send(SpeechEvent::Close);
        return true;
    }
    return false;
}

fn collision_outer_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    mut camera_query: Query<&mut GameCamera>,
    sensor_query: Query<&RabbitOuterSensor>,
    player_query: Query<&Actor, With<Player>>,
    mut speech_writer: EventWriter<SpeechEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(..) => {}
            CollisionEvent::Stopped(a, b, _option) => {
                let _ = out_sensor(
                    a,
                    b,
                    &mut camera_query,
                    &sensor_query,
                    &player_query,
                    &mut speech_writer,
                ) || out_sensor(
                    b,
                    a,
                    &mut camera_query,
                    &sensor_query,
                    &player_query,
                    &mut speech_writer,
                );
            }
        }
    }
}

fn out_sensor(
    a: &Entity,
    b: &Entity,
    camera_query: &mut Query<&mut GameCamera>,
    sensor_query: &Query<&RabbitOuterSensor>,
    player_query: &Query<&Actor, With<Player>>,
    _speech_writer: &mut EventWriter<SpeechEvent>,
) -> bool {
    if sensor_query.contains(*a) {
        if let Ok(_) = player_query.get(*b) {
            let mut camera = camera_query.single_mut();
            camera.target = None;
        }
    }
    return false;
}

pub struct TrainingRabbitPlugin;

impl Plugin for TrainingRabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (collision_inner_sensor, collision_outer_sensor)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
