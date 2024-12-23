use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::constant::ENEMY_GROUP;
use crate::constant::ENTITY_GROUP;
use crate::constant::PAINT_LAYER_Z;
use crate::constant::WITCH_GROUP;
use crate::entity::actor::ActorEvent;
use crate::entity::life::Life;
use crate::se::SEEvent;
use crate::se::SE;
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
#[derive(Event)]
pub struct SpawnImpact {
    pub owner: Entity,
    pub position: Vec2,
    pub radius: f32,
    pub impulse: f32,
}

fn read_impact_event(
    mut commands: Commands,
    assets: Res<GameAssets>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut writer: EventWriter<SEEvent>,
    mut reader: EventReader<SpawnImpact>,
    mut life_query: Query<(&mut Life, &Transform, Option<&mut ExternalImpulse>)>,
    mut camera_query: Query<(&mut GameCamera, &Transform), Without<Life>>,
    mut damage_writer: EventWriter<ActorEvent>,
) {
    let context: &RapierContext = rapier_context.single();

    for SpawnImpact {
        owner,
        position,
        radius,
        impulse,
    } in reader.read()
    {
        writer.send(SEEvent::pos(SE::Drop, *position));
        let (mut camera, camera_transform) = camera_query.single_mut();
        let distance = camera_transform.translation.truncate().distance(*position);
        let max_range = 320.0; // 振動が起きる最大距離
        camera.vibration = (20.0 * (max_range - distance).max(0.0) / max_range).min(10.0);

        let mut entities: Vec<Entity> = Vec::new();

        context.intersections_with_shape(
            *position,
            0.0,
            &Collider::ball(*radius),
            QueryFilter {
                groups: Some(CollisionGroups::new(
                    ENEMY_GROUP,
                    WITCH_GROUP | ENEMY_GROUP | ENTITY_GROUP,
                )),
                ..default()
            },
            |entity| {
                if entity != *owner {
                    entities.push(entity);
                }
                true // 交差図形の検索を続ける
            },
        );

        for entity in entities {
            if let Ok((mut life, life_transform, mut external_impulse)) = life_query.get_mut(entity)
            {
                let damage = 10;
                let p = life_transform.translation.truncate();
                life.life = (life.life - damage).max(0);
                damage_writer.send(ActorEvent::Damaged {
                    actor: entity,
                    damage: 10,
                    position: p,
                });
                writer.send(SEEvent::pos(SE::Damage, p));
                if let Some(ref mut ex) = external_impulse {
                    ex.impulse = (p - position).normalize_or_zero() * impulse;
                }
            }
        }

        commands.spawn((
            Name::new("impact"),
            Impact { lifetime: 60 },
            AseSpriteAnimation {
                aseprite: assets.impact.clone(),
                animation: "idle".into(),
            },
            Transform::from_translation(position.extend(PAINT_LAYER_Z)),
        ));
    }
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
            (read_impact_event, update_impact)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
