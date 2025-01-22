use crate::actor::ActorEvent;
use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::collision::SENSOR_GROUPS;
use crate::component::life::Life;
use crate::constant::*;
use crate::entity::fire::Fire;
use crate::level::map::index_to_position;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;

pub const EXPLOSION_COUNT: u32 = 10;

#[derive(Default, Component, Reflect)]
pub struct ExplosionPointLight {
    pub lifetime: u32,
}

#[derive(Event)]
pub struct SpawnExplosion {
    pub position: Vec2,
    pub radius: f32,
    pub impulse: f32,
    pub damage: u32,
}

fn spawn_explosion(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut se: EventWriter<SEEvent>,
    mut reader: EventReader<SpawnExplosion>,
    mut camera_query: Query<&mut GameCamera>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut life_query: Query<&Transform, With<Life>>,
    fire_query: Query<(Entity, &Transform), (With<Fire>, Without<Life>)>,
    mut damage_writer: EventWriter<ActorEvent>,
    mut level: ResMut<LevelSetup>,
) {
    let context: &RapierContext = rapier_context.single();

    for SpawnExplosion {
        position,
        radius,
        impulse,
        damage,
    } in reader.read()
    {
        context.intersections_with_shape(
            *position,
            0.0,
            &Collider::ball(*radius),
            QueryFilter {
                groups: Some(*SENSOR_GROUPS),
                ..default()
            },
            |entity| {
                if let Ok(life_transform) = life_query.get_mut(entity) {
                    let p = life_transform.translation.truncate();
                    let distance = p.distance(*position);

                    let damage = (*damage as f32 * 0.1
                        + (1.0 - distance / radius) * *damage as f32 * 0.9)
                        as u32;
                    damage_writer.send(ActorEvent::Damaged {
                        actor: entity,
                        damage,
                        position: p,
                        fire: false,
                        impulse: (p - position).normalize_or_zero() * impulse,
                        stagger: 120,
                        metamorphose: None,
                    });
                }

                true // 交差図形の検索を続ける
            },
        );

        commands.spawn((
            StateScoped(GameState::InGame),
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "scorch_mark".into(),
            },
            Transform::from_translation(position.extend(SCORCH_MARK_LAYER_Z)),
        ));

        commands.spawn((
            ExplosionPointLight {
                lifetime: EXPLOSION_COUNT,
            },
            StateScoped(GameState::InGame),
            PointLight2d {
                intensity: 20.0,
                radius: 200.0,
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
        ));

        se.send(SEEvent::pos(SE::Bakuhatsu, *position));

        let mut camera = camera_query.single_mut();
        camera.vibration = 12.0;

        // 付近の炎を消火
        for (entity, transform) in fire_query.iter() {
            let distance = transform.translation.truncate().distance(*position);
            if distance < TILE_SIZE * 5.0 {
                commands.entity(entity).despawn();
            }
        }

        // 付近の壁を破壊、または氷床を水に変更
        if let Some(ref mut chunk) = level.chunk {
            let range = 5;
            for dy in -range..(range + 1) {
                for dx in -range..(range + 1) {
                    let x = (position.x / TILE_SIZE) as i32 + dx;
                    let y = (position.y / -TILE_SIZE) as i32 + dy;
                    let distance = index_to_position((x, y)).distance(*position);
                    if distance < TILE_SIZE * 5.0 {
                        match chunk.get_tile(x, y) {
                            Tile::Wall => {
                                chunk.set_tile(x, y, Tile::StoneTile);
                            }
                            Tile::Ice => {
                                chunk.set_tile(x, y, Tile::Water);
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
    }
}

fn update_pointlight(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ExplosionPointLight, &mut PointLight2d)>,
) {
    for (entity, mut explosion, mut light) in query.iter_mut() {
        light.intensity = 20.0 * explosion.lifetime as f32 / EXPLOSION_COUNT as f32;
        if explosion.lifetime <= 0 {
            commands.entity(entity).despawn();
        } else {
            explosion.lifetime -= 1;
        }
    }
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosion>();
        app.add_systems(
            FixedUpdate,
            (spawn_explosion, update_pointlight).in_set(FixedUpdateGameActiveSet),
        );
    }
}
