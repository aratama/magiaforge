use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorState;
use crate::entity::life::Life;
use crate::entity::EntityChildrenAutoDepth;
use crate::inventory::Inventory;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_rapier2d::prelude::*;

const RABBIT_RADIUS: f32 = 5.0;

pub fn spawn_rabbit<T: Component, S: Component, U: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    sprite: &Handle<Aseprite>,
    position: Vec2,
    marker: T,
    sensor_marker: S,
    outer_sensor_marker: U,
) {
    commands
        .spawn((
            Name::new("rabbit"),
            marker,
            StateScoped(GameState::InGame),
            Actor {
                uuid: uuid::Uuid::new_v4(),
                pointer: Vec2::from_angle(0.0),
                point_light_radius: 0.0,
                radius: RABBIT_RADIUS,
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
                state: ActorState::default(),
                wait: 0,
            },
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
                Collider::ball(RABBIT_RADIUS),
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
                    ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | ENEMY_GROUP | DOOR_GROUP,
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
                    aseprite: sprite.clone(),
                    animation: "idle_d".into(),
                },
                EntityChildrenAutoDepth { offset: 0.0 },
            ));

            builder.spawn((
                RabbitInnerSensor,
                sensor_marker,
                Collider::ball(16.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(SENSOR_GROUP, WITCH_GROUP),
                Transform::default(), // RabbitSensor経由でフキダシの位置を取得するので、ここにGlobalTransformが必要
            ));

            builder.spawn((
                outer_sensor_marker,
                Collider::ball(32.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(SENSOR_GROUP, WITCH_GROUP),
            ));
        });
}

#[derive(Component)]
struct RabbitInnerSensor;

// fn squat(

//     inner_sensor_query: Query<&GlobalTransform, With<RabbitInnerSensor>>,
// ){

// }

pub struct RabbitPlugin;

impl Plugin for RabbitPlugin {
    fn build(&self, _app: &mut App) {}
}
