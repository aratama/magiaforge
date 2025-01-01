use super::counter::Counter;
use super::counter::CounterAnimated;
use super::point_light::WithPointLight;
use crate::asset::GameAssets;
use crate::constant::ENEMY_GROUP;
use crate::constant::ENTITY_GROUP;
use crate::constant::RABBIT_GROUP;
use crate::constant::SENSOR_GROUP;
use crate::constant::WITCH_GROUP;
use crate::entity::EntityDepth;
use crate::se::SEEvent;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
pub struct Fire;

pub fn spawn_fire(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands.spawn((
        Name::new("fire"),
        StateScoped(GameState::InGame),
        Fire,
        Counter::down(1800),
        EntityDepth,
        Visibility::default(),
        Transform::from_translation(position.extend(0.0)),
        CounterAnimated,
        AseSpriteAnimation {
            aseprite: assets.fire.clone(),
            animation: "default".into(),
            ..default()
        },
        WithPointLight {
            radius: 128.0,
            intensity: 1.0,
            falloff: 10.0,
            color: Color::hsl(42.0, 1.0, 0.71),
            animation_offset: rand::random::<u32>() % 1000,
            speed: 0.43,
            amplitude: 0.1,
        },
        (
            Sensor,
            Collider::ball(8.0),
            CollisionGroups::new(
                SENSOR_GROUP,
                ENTITY_GROUP | WITCH_GROUP | ENEMY_GROUP | RABBIT_GROUP,
            ),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ),
    ));
}

fn despown(
    mut commands: Commands,
    query: Query<(Entity, &Counter, &Transform), With<Fire>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, counter, transform) in query.iter() {
        if counter.count <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (despown)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Fire>();
    }
}
