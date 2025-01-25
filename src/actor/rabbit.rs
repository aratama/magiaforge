use super::ActorSpriteGroup;
use super::ActorType;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::collision::RABBIT_GROUPS;
use crate::collision::SENSOR_GROUPS;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::ChildEntityDepth;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::message_rabbit::SpellListRabbit;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::registry::Registry;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_rapier2d::prelude::*;

pub fn default_rabbit(aseprite: &String, senario: &String) -> Actor {
    Actor {
        extra: ActorExtra::Rabbit {
            aseprite: aseprite.clone(),
            senario: senario.clone(),
        },
        actor_group: ActorGroup::Friend,
        fire_resistance: true,
        life: 100000,
        max_life: 100000,
        ..default()
    }
}

pub fn spawn_rabbit(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
    senario: &String,
) -> Entity {
    let props = registry.get_actor_props(ActorType::Rabbit);
    let mut entity = commands.spawn((
        Name::new("rabbit"),
        actor,
        Transform::from_translation(position.extend(0.0)),
        // 以下はRapier2Dのコンポーネント
        (
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(props.radius),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveCollisionTypes::DYNAMIC_KINEMATIC,
            *RABBIT_GROUPS,
        ),
    ));

    entity.insert(MessageRabbit::new(senario));

    match senario.as_str() {
        "ShopRabbit" => {
            entity.insert(ShopRabbit);
        }
        "SpellListRabbit" => {
            entity.insert(SpellListRabbit);
        }
        _ => {}
    };

    entity.with_children(|builder| {
        builder.spawn((
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "rabbit_shadow".into(),
            },
            Transform::from_xyz(0.0, 0.0, SHADOW_LAYER_Z),
        ));

        builder.spawn(ActorSpriteGroup).with_children(|builder| {
            builder.spawn((
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: aseprite.clone(),
                    animation: "idle_d".into(),
                },
                ChildEntityDepth { offset: 0.0 },
            ));
        });

        match senario.as_str() {
            "ShopRabbit" => {
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

        match senario.as_str() {
            "ShopRabbit" => {
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
