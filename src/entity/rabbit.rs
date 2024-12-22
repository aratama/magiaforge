use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::actor::{ActorGroup, ActorState};
use crate::entity::life::Life;
use crate::entity::EntityChildrenAutoDepth;
use crate::inventory::Inventory;
use crate::language::Dict;
use crate::se::{SEEvent, SE};
use crate::speech_bubble::SpeechEvent;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AseSpriteAnimation, AseSpriteSlice};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct ShopRabbit;

#[derive(Component)]
pub struct ShopRabbitSensor;

#[derive(Component)]
struct RabbitOuterSensor;

pub fn spawn_rabbit<T: Component, S: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    marker: T,
    sensor_marker: S,
) {
    commands
        .spawn((
            Name::new("rabbit"),
            marker,
            StateScoped(GameState::InGame),
            Actor {
                uuid: uuid::Uuid::new_v4(),
                pointer: Vec2::from_angle(0.0),
                intensity: 0.5,
                move_direction: Vec2::ZERO,
                move_force: 0.0,
                fire_state: ActorFireState::Idle,
                fire_state_secondary: ActorFireState::Idle,
                current_wand: 0,
                effects: default(),
                actor_group: ActorGroup::Player,
                golds: 0,
                inventory: Inventory::new(),
                equipments: [None; MAX_ITEMS_IN_EQUIPMENT],
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
                    linear_damping: 1.0,
                    angular_damping: 1.0,
                },
                ExternalForce::default(),
                ExternalImpulse::default(),
                ActiveCollisionTypes::DYNAMIC_KINEMATIC,
                CollisionGroups::new(
                    RABBIT_GROUP,
                    ENTITY_GROUP
                        | WALL_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | ENEMY_GROUP
                        | ENEMY_BULLET_GROUP
                        | DOOR_GROUP,
                ),
            ),
        ))
        .with_children(|builder| {
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
                sensor_marker,
                Collider::ball(16.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(SENSOR_GROUP, WITCH_GROUP),
                Transform::default(), // RabbitSensor経由でフキダシの位置を取得するので、ここにGlobalTransformが必要
            ));

            builder.spawn((
                RabbitOuterSensor,
                Collider::ball(32.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(SENSOR_GROUP, WITCH_GROUP),
            ));
        });
}

fn collision_inner_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<&ShopRabbitSensor>,
    mut camera_query: Query<&mut GameCamera>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut speech_writer: EventWriter<SpeechEvent>,
    mut se: EventWriter<SEEvent>,
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
                    &mut se,
                ) || chat_start(
                    b,
                    a,
                    &mut camera_query,
                    &sensor_query,
                    &mut player_query,
                    &mut speech_writer,
                    &mut se,
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
    sensor_query: &Query<&ShopRabbitSensor>,
    player_query: &mut Query<&mut Actor, With<Player>>,
    speech_writer: &mut EventWriter<SpeechEvent>,
    se: &mut EventWriter<SEEvent>,
) -> bool {
    let mut camera = camera_query.single_mut();

    if let Ok(_) = sensor_query.get(*sensor_entity) {
        if let Ok(mut actor) = player_query.get_mut(*player_entity) {
            let dept = actor.dept();
            if 0 < dept {
                if actor.liquidate() {
                    camera.target = Some(*sensor_entity);
                    se.send(SEEvent::new(SE::Register));
                    speech_writer.send(SpeechEvent::Speech {
                        speaker: *sensor_entity,
                        text: Dict {
                            ja: format!("合計{}ゴールドのお買い上げ！\nありがとう", dept)
                                .to_string(),
                            en: format!("Your total is {} Golds\nThank you", dept).to_string(),
                        },
                    });
                } else {
                    camera.target = Some(*sensor_entity);
                    speech_writer.send(SpeechEvent::Speech {
                        speaker: *sensor_entity,
                        text: Dict {
                            ja: format!(
                                "おいおい\n{}ゴールド足りないよ\n買わない商品は\n戻しておいてね",
                                dept - actor.golds
                            )
                            .to_string(),
                            en: format!(
                            "Hey, hey!\nYou are {} Golds short!\nPut it back that you woun't buy",
                            dept - actor.golds
                        )
                            .to_string(),
                        },
                    });
                }
            } else {
                camera.target = Some(*sensor_entity);
                speech_writer.send(SpeechEvent::Speech {
                    speaker: *sensor_entity,
                    text:  Dict {
                        ja: "やあ\nなにか買っていくかい？\n欲しい商品があったら\n持ってきて".to_string(),
                        en: "Hello\nIs there anything you want?\nIf you have something you want\nbring it here".to_string(),
                    },
                });
            }
            return true;
        }
    }
    return false;
}

fn chat_end(
    a: &Entity,
    b: &Entity,
    sensor_query: &Query<&ShopRabbitSensor>,
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

pub struct RabbitPlugin;

impl Plugin for RabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (collision_inner_sensor, collision_outer_sensor)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
