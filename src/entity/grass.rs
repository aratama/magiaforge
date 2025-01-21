use crate::asset::GameAssets;
use crate::collision::*;
use crate::component::counter::Counter;
use crate::component::entity_depth::EntityDepth;
use crate::constant::TILE_HALF;
use crate::constant::Z_ORDER_SCALE;
use crate::controller::player::Player;
use crate::actor::Actor;
use crate::entity::fire::Burnable;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;

#[derive(Default, Component, Reflect)]
pub struct Grasses {
    sway: f32,
}

#[derive(Default, Component, Reflect)]
pub struct SpriteGroup;

pub fn spawn_grasses(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            Name::new("grasses"),
            Grasses { sway: 0.0 },
            Counter::default(),
            StateScoped(GameState::InGame),
            EntityDepth::new(),
            Burnable {
                life: 30 * 10 + rand::random::<u32>() % 30,
            },
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            (
                Sensor,
                Collider::cuboid(TILE_HALF, TILE_HALF),
                // 草はいかなる弾丸も妨げないので SENSOR_GROUP に属します
                *SENSOR_GROUPS,
                ActiveEvents::COLLISION_EVENTS,
            ),
        ))
        .with_children(|builder| {
            builder
                .spawn((SpriteGroup, Transform::from_xyz(0.0, -8.0, 0.0)))
                .with_children(|builder| {
                    builder.spawn((
                        Name::new("grass2"),
                        Transform::from_xyz(0.0, 12.0, Z_ORDER_SCALE * 12.0),
                        AseSpriteSlice {
                            aseprite: assets.atlas.clone(),
                            name: format!("grass_{}", rand::random::<u32>() % 3).into(),
                        },
                    ));
                    builder.spawn((
                        Name::new("grass1"),
                        Transform::from_xyz(0.0, 8.0, Z_ORDER_SCALE * 8.0),
                        AseSpriteSlice {
                            aseprite: assets.atlas.clone(),
                            name: format!("grass_{}", rand::random::<u32>() % 3).into(),
                        },
                    ));
                    builder.spawn((
                        Name::new("grass0"),
                        Transform::from_xyz(0.0, 4.0, Z_ORDER_SCALE * 4.0),
                        AseSpriteSlice {
                            aseprite: assets.atlas.clone(),
                            name: format!("grass_{}", rand::random::<u32>() % 3).into(),
                        },
                    ));
                });
        });
}

fn burnout(mut commands: Commands, query: Query<(Entity, &Burnable), With<Grasses>>) {
    for (entity, burnable) in query.iter() {
        if burnable.life <= 0 {
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
        }
    }
}

fn collision_outer_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    mut sensor_query: Query<&mut Grasses>,
    player_query: Query<&Actor, With<Player>>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Started(sensor, _) => {
                let mut grasses = sensor_query.get_mut(sensor).unwrap();
                grasses.sway = 0.5;
            }
            IdentifiedCollisionEvent::Stopped(sensor, _) => {
                let mut grasses = sensor_query.get_mut(sensor).unwrap();
                grasses.sway = 0.5;
            }
            _ => {}
        }
    }
}

fn sway(
    mut query: Query<(&Counter, &mut Grasses)>,
    mut group_query: Query<(&Parent, &mut Transform), With<SpriteGroup>>,
) {
    for (parent, mut transform) in group_query.iter_mut() {
        if let Ok((counter, mut grasses)) = query.get_mut(parent.get()) {
            transform.rotation =
                Quat::from_rotation_z((counter.count as f32 * 0.1).sin() * grasses.sway);
            grasses.sway *= 0.95;
        }
    }
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (burnout, collision_outer_sensor, sway).in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Grasses>();
    }
}
