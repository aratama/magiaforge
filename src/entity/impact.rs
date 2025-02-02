use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::camera::GameCamera;
use crate::collision::SENSOR_GROUPS;
use crate::component::counter::CounterAnimated;
use crate::constant::PAINT_LAYER_Z;
use crate::constant::TILE_SIZE;
use crate::level::chunk::index_to_position;
use crate::level::tile::Tile;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::DROP;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Impact {
    lifetime: u32,
}

/// 一定の範囲にダメージと吹き飛ばし効果を与える衝撃波です
/// 1フレームだけ当たり判定があり、すぐに消えます
#[derive(Event, Clone, Copy, Debug)]
pub struct SpawnImpact {
    pub owner: Option<Entity>,
    pub position: Vec2,
    pub radius: f32,
    pub impulse: f32,
}

fn read_impact_event(
    mut commands: Commands,
    registry: Registry,
    mut level: ResMut<GameWorld>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut writer: EventWriter<SEEvent>,
    mut reader: EventReader<SpawnImpact>,
    mut life_query: Query<&Transform, With<Actor>>,
    mut camera_query: Query<(&mut GameCamera, &Transform), Without<Actor>>,
    mut damage_writer: EventWriter<ActorEvent>,
) {
    let context: &RapierContext = rapier_context.single();

    for event in reader.read() {
        spawn_impact(
            &mut commands,
            &registry,
            &mut level,
            &mut writer,
            &mut life_query,
            &mut camera_query,
            &mut damage_writer,
            &context,
            *event,
        );
    }
}

pub fn spawn_impact(
    commands: &mut Commands,
    registry: &Registry,
    level: &mut ResMut<GameWorld>,
    writer: &mut EventWriter<SEEvent>,
    life_query: &mut Query<&Transform, With<Actor>>,
    camera_query: &mut Query<(&mut GameCamera, &Transform), Without<Actor>>,
    damage_writer: &mut EventWriter<ActorEvent>,
    context: &RapierContext,
    impact: SpawnImpact,
) {
    writer.send(SEEvent::pos(DROP, impact.position));
    let (mut camera, camera_transform) = camera_query.single_mut();
    camera.vibrate(&camera_transform, impact.position, 20.0);

    let mut entities: Vec<Entity> = Vec::new();

    context.intersections_with_shape(
        impact.position,
        0.0,
        &Collider::ball(impact.radius),
        QueryFilter {
            groups: Some(*SENSOR_GROUPS),
            ..default()
        },
        |entity| {
            match impact.owner {
                Some(owner) if entity == owner => {
                    // 衝撃波の衝突先が詠唱者自身なら無視
                }
                _ => {
                    entities.push(entity);
                }
            }

            true // 交差図形の検索を続ける
        },
    );

    // 付近のエンティティにダメージ
    for entity in entities {
        if let Ok(life_transform) = life_query.get_mut(entity) {
            let p = life_transform.translation.truncate();
            damage_writer.send(ActorEvent::Damaged {
                actor: entity,
                damage: 10,
                position: p,
                fire: false,
                impulse: (p - impact.position).normalize_or_zero() * impact.impulse,
                stagger: 60,
                metamorphose: None,
                dispel: false,
            });
        }
    }

    // 付近の氷床を破壊
    let range = 5;
    for dy in -range..(range + 1) {
        for dx in -range..(range + 1) {
            let x = (impact.position.x / TILE_SIZE) as i32 + dx;
            let y = (impact.position.y / -TILE_SIZE) as i32 + dy;
            let distance = index_to_position((x, y)).distance(impact.position);
            if distance < TILE_SIZE * 5.0 {
                match level.get_tile(x, y).0.as_str() {
                    "Ice" => {
                        level.set_tile(x, y, Tile::new("Water"));
                    }
                    _ => {}
                };
            }
        }
    }

    commands.spawn((
        Name::new("impact"),
        StateScoped(GameState::InGame),
        Impact { lifetime: 60 },
        CounterAnimated,
        AseSpriteAnimation {
            aseprite: registry.assets.impact.clone(),
            animation: "idle".into(),
        },
        Transform::from_translation(impact.position.extend(PAINT_LAYER_Z)),
    ));
}

fn update_impact(mut commands: Commands, mut query: Query<(Entity, &mut Impact)>) {
    for (entity, mut impact) in query.iter_mut() {
        impact.lifetime -= 1;
        if impact.lifetime <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct ImpactPlugin;

impl Plugin for ImpactPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnImpact>();

        app.add_systems(
            FixedUpdate,
            (read_impact_event, update_impact).in_set(FixedUpdateGameActiveSet),
        );
    }
}
