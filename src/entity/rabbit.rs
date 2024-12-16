use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::actor::{ActorGroup, ActorState};
use crate::entity::life::Life;
use crate::entity::EntityChildrenAutoDepth;
use crate::speech_bubble::SpeechEvent;
use crate::states::GameState;
use crate::ui::interaction_marker::spawn_interaction_marker;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AseSpriteAnimation, AseSpriteSlice};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Rabbit;

#[derive(Component)]
struct RabbitSensor;

pub fn spawn_rabbit(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            Name::new("rabbit"),
            Rabbit,
            StateScoped(GameState::InGame),
            Actor {
                uuid: uuid::Uuid::new_v4(),
                spell_delay: 0,
                pointer: Vec2::from_angle(0.0),
                intensity: 0.5,
                move_direction: Vec2::ZERO,
                move_force: 0.0,
                fire_state: ActorFireState::Idle,
                current_wand: 0,
                effects: default(),
                actor_group: ActorGroup::Player,
                wands: [None, None, None, None],
            },
            ActorState::default(),
            Life {
                life: 100000,
                max_life: 100000,
                amplitude: 0.0,
            },
            Transform::from_translation(position.extend(0.0)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            // 以下はRapier2Dのコンポーネント
            (
                RigidBody::Dynamic,
                Velocity::default(),
                Collider::ball(5.0),
                GravityScale(0.0),
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 6.0,
                    angular_damping: 1.0,
                },
                ExternalForce::default(),
                ExternalImpulse::default(),
                CollisionGroups::new(
                    ENTITY_GROUP,
                    ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | ENEMY_GROUP,
                ),
            ),
        ))
        .with_children(|mut builder| {
            builder.spawn((
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "rabbit_shadow".into(),
                },
                Transform::from_xyz(0.0, 0.0, SHADOW_LAYER_Z),
            ));

            builder.spawn((
                AseSpriteAnimation {
                    aseprite: assets.rabbit.clone(),
                    animation: "idle_d".into(),
                },
                EntityChildrenAutoDepth { offset: 0.0 },
            ));

            builder.spawn((
                RabbitSensor,
                Collider::ball(16.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
}

fn chat(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<&RabbitSensor>,
    player_query: Query<&Player>,
    mut speech_writer: EventWriter<SpeechEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _option) => {
                let _ = chat_start(a, b, &sensor_query, &player_query, &mut speech_writer)
                    || chat_start(b, a, &sensor_query, &player_query, &mut speech_writer);
            }
            CollisionEvent::Stopped(a, b, _option) => {
                let _ = chat_end(a, b, &sensor_query, &player_query, &mut speech_writer)
                    || chat_end(b, a, &sensor_query, &player_query, &mut speech_writer);
            }
        }
    }
}

fn chat_start(
    a: &Entity,
    b: &Entity,
    sensor_query: &Query<&RabbitSensor>,
    player_query: &Query<&Player>,
    speech_writer: &mut EventWriter<SpeechEvent>,
) -> bool {
    if sensor_query.contains(*a) && player_query.contains(*b) {
        speech_writer.send(SpeechEvent::Speech(
            "やあ、君か。\nなにか買っていくかい？？".to_string(),
        ));
        return true;
    }
    return false;
}

fn chat_end(
    a: &Entity,
    b: &Entity,
    sensor_query: &Query<&RabbitSensor>,
    player_query: &Query<&Player>,
    speech_writer: &mut EventWriter<SpeechEvent>,
) -> bool {
    if sensor_query.contains(*a) && player_query.contains(*b) {
        speech_writer.send(SpeechEvent::Close);
        return true;
    }
    return false;
}

pub struct RabbitPlugin;

impl Plugin for RabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (chat)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
