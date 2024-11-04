use bevy::prelude::*;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use std::f32::consts::PI;
use uuid::Uuid;

use crate::states::GameState;

/// ライフを持ち、弾丸のダメージの対象となるエンティティを表します
#[derive(Component)]
pub struct Actor {
    pub uuid: Uuid,

    /// 次の魔法を発射できるまでのクールタイム
    pub cooltime: i32,
    pub life: i32,
    pub max_life: i32,
    pub latest_damage: i32,

    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,

    pub intensity: f32,

    pub state: ActorState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActorState {
    Idle,
    Run,
}

fn update_actor_z(mut query: Query<(&Actor, &mut Sprite)>) {
    for (actor, mut sprite) in query.iter_mut() {
        // プレイヤーの向き
        let angle = actor.pointer.y.atan2(actor.pointer.x);
        if angle < -PI * 0.5 || PI * 0.5 < angle {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}

#[derive(Component)]
pub struct ActorLight {
    owner: Entity,
}

fn update_actor_light(
    mut commands: Commands,
    mut light_query: Query<(Entity, &ActorLight, &mut PointLight2d, &mut Transform)>,
    actor_query: Query<(Entity, &Actor, &Transform), Without<ActorLight>>,
) {
    for (actor_entity, actor, transform) in actor_query.iter() {
        if light_query
            .iter()
            .find(|(_, light, _, _)| light.owner == actor_entity)
            .is_none()
        {
            // SpriteBundle に PointLight2d を追加すると、画面外に出た時に Sprite が描画されなくなり、
            // ライトも描画されず不自然になるため、別で追加する
            // https://github.com/jgayfer/bevy_light_2d/issues/26
            commands.spawn((
                ActorLight {
                    owner: actor_entity,
                },
                PointLight2dBundle {
                    transform: transform.clone(),
                    point_light: PointLight2d {
                        radius: 150.0,
                        intensity: actor.intensity,
                        falloff: 10.0,
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }

    for (light_entity, light, mut point_light, mut light_transform) in light_query.iter_mut() {
        if let Ok((_, actor, actor_transform)) = actor_query.get(light.owner) {
            point_light.intensity = actor.intensity;
            light_transform.translation.x = actor_transform.translation.x;
            light_transform.translation.y = actor_transform.translation.y;
        } else {
            commands.entity(light_entity).despawn_recursive();
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_actor_z, update_actor_light).run_if(in_state(GameState::InGame)),
        );
    }
}
