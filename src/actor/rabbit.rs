use super::ActorGroup;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::collision::RABBIT_GROUPS;
use crate::collision::SENSOR_GROUPS;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::ChildEntityDepth;
use crate::component::falling::Falling;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::registry::Registry;
use crate::registry::SenarioType;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

const RABBIT_RADIUS: f32 = 5.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Deserialize)]
pub enum RabbitType {
    Guide,
    Training,
    Shop,
    Singleplay,
    MultiPlay,
    Reading,
    SpellList,
}

pub fn default_rabbit(rabbit_type: RabbitType) -> Actor {
    Actor {
        extra: ActorExtra::Rabbit { rabbit_type },
        actor_group: ActorGroup::Friend,
        fire_resistance: true,
        life: 100000,
        max_life: 100000,
        ..default()
    }
}

pub fn spawn_rabbit(
    commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
    rabbit_type: RabbitType,
) -> Entity {
    let mut entity = commands.spawn((
        Name::new("rabbit"),
        StateScoped(GameState::InGame),
        actor,
        Transform::from_translation(position.extend(0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        Falling,
        // 以下はRapier2Dのコンポーネント
        (
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(RABBIT_RADIUS),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveCollisionTypes::DYNAMIC_KINEMATIC,
            *RABBIT_GROUPS,
        ),
    ));

    match rabbit_type {
        RabbitType::Shop => {
            entity.insert(ShopRabbit);
        }
        RabbitType::Training => {
            entity.insert(MessageRabbit {
                senario: SenarioType::TrainingRabbit,
            });
        }
        RabbitType::Singleplay => {
            entity.insert(MessageRabbit {
                senario: SenarioType::SingleplayRabbit,
            });
        }
        RabbitType::Guide => {
            entity.insert(MessageRabbit {
                senario: SenarioType::HelloRabbit,
            });
        }
        RabbitType::MultiPlay => {
            entity.insert(MessageRabbit {
                senario: SenarioType::MultiplayerRabbit,
            });
        }
        RabbitType::Reading => {
            entity.insert(MessageRabbit {
                senario: SenarioType::ReserchRabbit,
            });
        }
        RabbitType::SpellList => {
            entity.insert(MessageRabbit {
                senario: SenarioType::SpellListRabbit,
            });
        }
    };

    entity.with_children(|builder| {
        builder.spawn((
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "rabbit_shadow".into(),
            },
            Transform::from_xyz(0.0, 0.0, SHADOW_LAYER_Z),
        ));

        builder.spawn((
            CounterAnimated,
            AseSpriteAnimation {
                aseprite: match rabbit_type {
                    RabbitType::Shop => registry.assets.rabbit_yellow.clone(),
                    RabbitType::Training => registry.assets.rabbit_red.clone(),
                    RabbitType::Singleplay => registry.assets.rabbit_white.clone(),
                    RabbitType::Guide => registry.assets.rabbit_blue.clone(),
                    RabbitType::MultiPlay => registry.assets.rabbit_black.clone(),
                    RabbitType::Reading => registry.assets.rabbit_green.clone(),
                    RabbitType::SpellList => registry.assets.rabbit_blue.clone(),
                }
                .clone(),
                animation: "idle_d".into(),
            },
            ChildEntityDepth { offset: 0.0 },
        ));

        match rabbit_type {
            RabbitType::Shop => {
                builder.spawn((
                    ShopRabbitSensor,
                    Collider::ball(16.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    *SENSOR_GROUPS,
                    Transform::default(), // RabbitSensor経由でフキダシの位置を取得するので、ここにGlobalTransformが必要
                ));
            }
            _ => {
                builder.spawn((
                    MessageRabbitInnerSensor,
                    Collider::ball(16.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    *SENSOR_GROUPS,
                    Transform::default(), // RabbitSensor経由でフキダシの位置を取得するので、ここにGlobalTransformが必要
                ));
            }
        };

        match rabbit_type {
            RabbitType::Shop => {
                builder.spawn((
                    ShopRabbitOuterSensor,
                    Collider::ball(32.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    *SENSOR_GROUPS,
                ));
            }
            _ => {
                builder.spawn((
                    MessageRabbitOuterSensor,
                    Collider::ball(32.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    *SENSOR_GROUPS,
                ));
            }
        };
    });

    entity.id()
}

pub struct RabbitPlugin;

impl Plugin for RabbitPlugin {
    fn build(&self, _app: &mut App) {}
}
