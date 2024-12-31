use crate::{
    constant::{SENSOR_GROUP, WITCH_GROUP},
    se::{SEEvent, SE},
    states::GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{DefaultRapierContext, RapierContext},
    prelude::{Collider, CollisionGroups, QueryFilter},
};

use super::{
    actor::{Actor, ActorEvent},
    firebaall::Fire,
};

/// 木箱やトーチなどの破壊可能なオブジェクトを表すコンポーネントです
/// 弾丸は Breakable コンポーネントを持つエンティティに対してダメージを与えます
/// ただし、ライフがゼロになってもこのコンポーネント自身は自動でdespownしません
#[derive(Default, Component, Reflect)]
pub struct Life {
    /// 破壊可能なオブジェクトのライフ
    pub life: i32,

    pub max_life: i32,

    /// ダメージを受けた時の振動の幅
    pub amplitude: f32,

    /// 次に炎でダメージを受けるまでの間隔
    pub fire_damage_wait: u32,
}

impl Life {
    pub fn new(life: i32) -> Self {
        Self {
            life,
            max_life: life,
            amplitude: 0.0,
            fire_damage_wait: 0,
        }
    }

    pub fn damage(&mut self, damage: u32) {
        if 0 < self.life {
            self.life = (self.life - damage as i32).max(0);
            self.amplitude = 6.0;
        }
    }
}

/// ダメージを受けた時に振動するスプライト
#[derive(Default, Component, Reflect)]
pub struct LifeBeingSprite;

fn vibrate_breakabke_sprite(
    time: Res<Time>,
    mut breakable_query: Query<(&mut Life, &Children)>,
    mut breakable_sprite_query: Query<&mut Transform, With<LifeBeingSprite>>,
) {
    for (mut breakable, children) in breakable_query.iter_mut() {
        for child in children {
            if let Ok(mut transform) = breakable_sprite_query.get_mut(*child) {
                transform.translation.x = (time.elapsed_secs() * 56.0).sin() * breakable.amplitude;
            }
            breakable.amplitude *= 0.9;
        }
    }
}

fn fire_damage(
    mut actor_query: Query<(&mut Life, &Transform)>,
    fire_query: Query<&mut Transform, (With<Fire>, Without<Life>)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut actor_event: EventWriter<ActorEvent>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for (mut life, actor_transform) in actor_query.iter_mut() {
        if life.fire_damage_wait <= 0 {
            let mut entities = Vec::<Entity>::new();
            let context = rapier_context.single();
            context.intersections_with_shape(
                actor_transform.translation.truncate(),
                0.0,
                &Collider::ball(12.0),
                QueryFilter {
                    groups: Some(CollisionGroups::new(WITCH_GROUP, SENSOR_GROUP)),
                    ..default()
                },
                |entity| {
                    if fire_query.contains(entity) {
                        if life.fire_damage_wait <= 0 {
                            entities.push(entity);

                            // 一度炎ダメージを受けたらそれ以上他の炎からダメージを受けることはないため、
                            // 探索を打ち切る
                            return false;
                        }
                    }
                    true // 交差図形の検索を続ける
                },
            );

            for entity in entities {
                let damage = 4;
                life.damage(damage);
                life.fire_damage_wait = 60;
                let position = actor_transform.translation.truncate();
                actor_event.send(ActorEvent::Damaged {
                    actor: entity,
                    damage: damage as u32,
                    position,
                });
                se_writer.send(SEEvent::pos(SE::Damage, position));
            }
        }

        if 0 < life.fire_damage_wait {
            life.fire_damage_wait -= 1;
        }
    }
}

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (vibrate_breakabke_sprite, fire_damage).run_if(in_state(GameState::InGame)),
        );
    }
}
