use std::f32::consts::PI;

use bevy::prelude::*;
use uuid::Uuid;

use crate::constant::{ENTITY_LAYER_Z, Z_ORDER_SCALE};

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
}

fn update_actor_z(mut query: Query<(&Actor, &mut Sprite, &mut Transform)>) {
    for (actor, mut sprite, mut transform) in query.iter_mut() {
        transform.translation.z = get_entity_z(transform.translation.y);

        // プレイヤーの向き
        let angle = actor.pointer.y.atan2(actor.pointer.x);
        if angle < -PI * 0.5 || PI * 0.5 < angle {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}

pub fn get_entity_z(y: f32) -> f32 {
    ENTITY_LAYER_Z - y * Z_ORDER_SCALE
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_actor_z);
    }
}
