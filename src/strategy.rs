use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorGroup;
use crate::collision::SENSOR_GROUPS;
use crate::registry::Registry;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::QueryFilter;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use vleue_navigator::prelude::ManagedNavMesh;
use vleue_navigator::prelude::NavMeshStatus;
use vleue_navigator::NavMesh;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Strategy {
    min_life: u32,
    actions: Vec<Action>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Action {
    /// 敵が一定の範囲内に近づく、敵から攻撃を受けるなどするまで待機します
    Sleep,

    /// 指定したフレーム数待機します
    Wait { count: u32, random: u32 },

    /// farの範囲内にいる最も近い敵を探し、接近します
    /// 指定した時間を経過するか、nearまで近づくと次のアクションに移ります
    Approach {
        count: u32,
        random: u32,
        far: f32,
        near: f32,
    },

    /// 攻撃可能範囲内に敵がいる場合、敵を攻撃します。
    /// rangeは相手と自分の距離で、相手と自分の大きさを考慮するので、0のときは密着状態です
    Fire { count: u32, random: u32, range: f32 },

    GoTo {
        destination: Destination,
        count: u32,
    },

    /// ループを開始します
    Loop,

    /// ループを終了します
    End,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Destination {
    HomePosition,
}

#[derive(Default, Component, Debug, Deserialize, Clone, Reflect)]
pub struct Commander {
    pub next_action: usize,

    /// 現在のアクションを実行している時間
    pub count: u32,

    /// End を実行したときに戻るインデックス
    pub loop_start_indices: Vec<usize>,

    /// 最後に移動先になっていた座標
    /// 物体が少しでも動くとナビメッシュが再構築され、そのあいだ敵の動きが止まってしまうので
    /// その場合は最後の目的地への移動を続ける
    pub destination: Vec2,
}

#[derive(Debug, Clone)]
pub struct FindResult {
    pub position: Vec2,
    pub radius: f32,
}

fn update(
    registry: Registry,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut query: Query<(Entity, &mut Actor, &Transform)>,
    navmesh: Query<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    let context: &RapierContext = rapier_context.single();

    let map: HashMap<Entity, (ActorGroup, Vec2, f32)> = query
        .iter()
        .map(|(e, a, t)| {
            let props = registry.get_actor_props(&a.actor_type);
            (
                e,
                (
                    a.actor_group,
                    t.translation.truncate(),
                    props.collider.size(),
                ),
            )
        })
        .collect();

    for (entity, mut actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(&actor.actor_type);
        let strategies = &props.strategies;

        if strategies.is_empty() {
            continue;
        }

        let Some(selected) = strategies
            .iter()
            .find(|(_, str)| actor.life <= str.min_life)
            .map(|(name, _)| name)
            .cloned()
        else {
            warn!("No strategy found for {:?}", actor);
            continue;
        };

        let Some(strategy) = strategies.get(&selected) else {
            warn!("Strategy {} not found", selected);
            continue;
        };

        if strategy.actions.len() <= actor.commander.next_action {
            actor.commander.next_action = 0;
        }

        let next_action_index = actor.commander.next_action;

        let action = strategy.actions.get(next_action_index).unwrap();

        let origin = transform.translation.truncate();

        let self_actor_group = actor.actor_group;

        // 現在のアクションを実行する
        match action {
            Action::Sleep => {
                // do nothing
            }
            Action::Wait { count, random } => {
                // このアクションを開始したときに、ランダムにcountを追加することで
                // アクションの完了時間をばらつかせます
                if actor.commander.count == 0 {
                    actor.commander.count = rand::random::<u32>() % random;
                }

                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;
                actor.move_direction = Vec2::ZERO;

                if *count < actor.commander.count {
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                }
            }
            Action::Approach {
                far,
                near,
                count,
                random,
            } => {
                if actor.commander.count == 0 {
                    actor.commander.count = rand::random::<u32>() % random;
                }

                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;
                actor.move_direction = Vec2::ZERO;

                // 指定した範囲にいる、自分以外で、かつ別のグループに所属するアクターの一覧を取得
                // アクターは多数になるので一旦 Rapier でクエリを送っていますが、
                // 単に線形に探してもいいかも？
                let Some(nearest) =
                    find_target(context, &map, entity, self_actor_group, origin, *far)
                else {
                    continue;
                };

                // 指定した距離まで近づいたら次のアクションへ移行
                if (origin - nearest.position).length() < *near {
                    actor.move_direction = Vec2::ZERO;
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                    continue;
                }

                // ナビメッシュでルートを検索
                let (navmesh_handle, status) = navmesh.single();
                let navmesh = navmeshes.get(navmesh_handle);

                let destination = if *status == NavMeshStatus::Built {
                    if let Some(navmesh) = navmesh {
                        let from = origin;
                        let to = nearest.position;

                        if let Some(path) = navmesh.path(
                            // エンティティの位置そのものを使うと、壁際に近づいたときに agent_radius のマージンに埋もれて
                            // 到達不可能になってしまうので、タイルの中心を使います
                            from, to,
                        ) {
                            // for p in path.path.iter() {
                            //     info!("{:?} {:?}", p.x / TILE_SIZE, p.y / TILE_SIZE);
                            // }
                            if let Some(first) = path.path.first() {
                                *first
                            } else {
                                // ここに来ることはない？
                                warn!("first not found");
                                actor.commander.destination
                            }
                        } else {
                            // warn!("path not found");
                            actor.commander.destination
                        }
                    } else {
                        warn!("navmesh not found");
                        actor.commander.destination
                    }
                } else {
                    actor.commander.destination
                };

                actor.commander.destination = destination;

                actor.move_direction = (destination - origin).normalize_or_zero();

                // 一定時間経過したら次のアクションへ移行
                if *count < actor.commander.count {
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                }
            }
            Action::Fire {
                count,
                random,
                range,
            } => {
                if actor.commander.count == 0 {
                    actor.commander.count = rand::random::<u32>() % random;
                }

                // 最も近い相手を検索
                // このとき相手の大きさがわからないので正確な検索範囲はわからないが、
                // 十分に大きい距離を検索する
                let Some(nearest) = find_target(
                    context,
                    &map,
                    entity,
                    self_actor_group,
                    origin,
                    props.collider.size() + *range + 48.0,
                ) else {
                    continue;
                };

                let max_range = props.collider.size() + nearest.radius + range;

                // 指定した距離から遠ければ中止
                let diff = nearest.position - origin;
                if max_range < diff.length() {
                    actor.move_direction = Vec2::ZERO;
                    actor.fire_state = ActorFireState::Idle;
                    actor.fire_state_secondary = ActorFireState::Idle;
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                    continue;
                }

                // 相手に狙いを合わせる
                actor.pointer = nearest.position - origin;
                actor.move_direction = Vec2::ZERO;
                actor.fire_state = ActorFireState::Fire;
                actor.fire_state_secondary = ActorFireState::Idle;

                if *count < actor.commander.count {
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                }
            }
            Action::GoTo { destination, count } => {
                actor.fire_state = ActorFireState::Idle;
                actor.fire_state_secondary = ActorFireState::Idle;

                let dest = match destination {
                    Destination::HomePosition => actor.home_position,
                };
                let diff = dest - origin;
                if diff.length() < 8.0 {
                    actor.move_direction = Vec2::ZERO;
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                    continue;
                }

                actor.move_direction = diff.normalize_or_zero();

                if *count < actor.commander.count {
                    actor.commander.count = 0;
                    actor.commander.next_action += 1;
                }
            }
            Action::Loop => {
                actor.commander.loop_start_indices.push(next_action_index);
                actor.commander.count = 0;
                actor.commander.next_action += 1;
            }
            Action::End => {
                let Some(index) = actor.commander.loop_start_indices.pop() else {
                    // ストラテジーの再読み込み後はインデックスが壊れることがあるのに注意
                    warn!("Loop stack is empty");
                    actor.commander.loop_start_indices.clear();
                    actor.commander.next_action = 0;
                    actor.commander.count = 0;
                    continue;
                };
                actor.commander.count = 0;
                actor.commander.next_action = index;
            }
        }

        actor.commander.count += 1;
    }
}

fn find_target(
    context: &RapierContext,
    map: &HashMap<Entity, (ActorGroup, Vec2, f32)>,
    entity: Entity,
    self_actor_group: ActorGroup,
    origin: Vec2,
    range: f32,
) -> Option<FindResult> {
    let mut enemies: Vec<FindResult> = Vec::new();
    context.intersections_with_shape(
        origin,
        0.0,
        &Collider::ball(range),
        QueryFilter::from(*SENSOR_GROUPS),
        |e| {
            if e != entity {
                if let Some((e_g, e_t, e_r)) = map.get(&e) {
                    if *e_g != self_actor_group && *e_g != ActorGroup::Entity {
                        enemies.push(FindResult {
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
    enemies.sort_by(compare_distance(origin));
    enemies.first().cloned()
}

pub fn compare_distance(origin: Vec2) -> impl FnMut(&FindResult, &FindResult) -> Ordering {
    move |a, b| {
        let a_diff = a.position - origin;
        let b_diff = b.position - origin;
        a_diff.length().partial_cmp(&b_diff.length()).unwrap()
    }
}

pub struct StrategyPlugin;

impl Plugin for StrategyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update,).run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
    }
}
