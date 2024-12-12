use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::prelude::*;

use crate::{
    asset::GameAssets,
    camera::GameCamera,
    command::GameCommand,
    constant::{ENEMY_GROUP, ENTITY_GROUP, PAINT_LAYER_Z, WITCH_GROUP},
    controller::player::Player,
    enemy::huge_slime::{HugeSlime, HugeSlimeSprite},
    entity::damege::spawn_damage_number,
    states::GameState,
};

use super::life::Life;

#[derive(Component)]
struct Impact {
    lifetime: u32,
}

/// 一定の範囲にダメージと吹き飛ばし効果を与える衝撃波です
/// 1フレームだけ当たり判定があり、すぐに消えます
pub fn spawn_impact(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    player_query: &mut Query<(&mut Life, &Transform, &mut ExternalImpulse), With<Player>>,
    life_query: &mut Query<
        (&mut Life, &Transform, Option<&mut ExternalImpulse>),
        (
            Without<GameCamera>,
            Without<HugeSlimeSprite>,
            Without<HugeSlime>,
            Without<Player>,
        ),
    >,
    rapier_context: &Query<&RapierContext, With<DefaultRapierContext>>,
    writer: &mut EventWriter<GameCommand>,
    position: Vec2,
    radius: f32,
    impulse: f32,
) {
    let context: &RapierContext = rapier_context.single();

    let mut entities: Vec<Entity> = Vec::new();

    context.intersections_with_shape(
        position,
        0.0,
        &Collider::ball(radius),
        QueryFilter {
            groups: Some(CollisionGroups::new(
                ENEMY_GROUP,
                WITCH_GROUP | ENEMY_GROUP | ENTITY_GROUP,
            )),
            ..default()
        },
        |entity| {
            entities.push(entity);
            true // 交差図形の検索を続ける
        },
    );

    info!("lifes {:?}", life_query.iter().count());

    for entity in entities {
        if let Ok((mut life, life_transform, mut external_impulse)) = player_query.get_mut(entity) {
            info!("hit player {:?}", life.life);

            let damage = 10;
            let p = life_transform.translation.truncate();
            life.life = (life.life - damage).max(0);
            spawn_damage_number(&mut commands, 10, p);
            writer.send(GameCommand::SEDamage(Some(p)));
            external_impulse.impulse = (p - position).normalize() * impulse;
        } else if let Ok((mut life, life_transform, mut external_impulse)) =
            life_query.get_mut(entity)
        {
            info!("hit something {:?}", life.life);

            let damage = 10;
            let p = life_transform.translation.truncate();
            life.life = (life.life - damage).max(0);
            spawn_damage_number(&mut commands, 10, p);
            writer.send(GameCommand::SEDamage(Some(p)));
            if let Some(ref mut ex) = external_impulse {
                ex.impulse = (p - position).normalize() * impulse;
            }
        } else {
            warn!("HugeSlime hits {:?} but not found life", entity);
        }
    }

    commands.spawn((
        Impact { lifetime: 60 },
        AseSpriteAnimation {
            aseprite: assets.impact.clone(),
            animation: "idle".into(),
        },
        Transform::from_translation(position.extend(PAINT_LAYER_Z)),
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
        app.add_systems(
            FixedUpdate,
            update_impact
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
