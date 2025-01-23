use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::collision::SENSOR_GROUPS;
use crate::constant::GameActors;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::QueryFilter;
use std::cmp::Ordering;

pub struct Finder {
    map: HashMap<Entity, (ActorGroup, Vec2, f32)>,
}

#[derive(Debug, Clone)]
pub struct FindResult {
    pub entity: Entity,
    pub position: Vec2,
    pub radius: f32,
}

impl Finder {
    pub fn new(constants: &GameActors, query: &Query<(Entity, &Actor, &Transform)>) -> Self {
        Self {
            map: query
                .iter()
                .map(|(e, a, t)| {
                    let props = a.to_props(&constants);
                    (e, (a.actor_group, t.translation.truncate(), props.radius))
                })
                .collect(),
        }
    }

    pub fn nearest_opponent(
        &self,
        rapier_context: &Query<&RapierContext, With<DefaultRapierContext>>,
        entity: Entity,
        range: f32,
    ) -> Option<FindResult> {
        let context: &RapierContext = rapier_context.single();

        if let Some((self_actor_group, origin, _)) = self.map.get(&entity) {
            // 指定した範囲にいる、自分以外で、かつ別のグループに所属するアクターの一覧を取得
            let mut enemies: Vec<FindResult> = Vec::new();
            context.intersections_with_shape(
                *origin,
                0.0,
                &Collider::ball(range),
                QueryFilter::from(*SENSOR_GROUPS),
                |e| {
                    if e != entity {
                        if let Some((e_g, e_t, e_r)) = self.map.get(&e) {
                            if *e_g != *self_actor_group && *e_g != ActorGroup::Entity {
                                enemies.push(FindResult {
                                    entity: e,
                                    position: *e_t,
                                    radius: *e_r,
                                });
                            }
                        }
                    }
                    true // 交差図形の検索を続ける
                },
            );

            // 最も近くにいる、別グループのアクターに対して接近または攻撃
            enemies.sort_by(compare_distance(*origin));
            enemies.first().cloned()
        } else {
            None
        }
    }
}

pub fn compare_distance(origin: Vec2) -> impl FnMut(&FindResult, &FindResult) -> Ordering {
    move |a, b| {
        let a_diff = a.position - origin;
        let b_diff = b.position - origin;
        a_diff.length().partial_cmp(&b_diff.length()).unwrap()
    }
}
