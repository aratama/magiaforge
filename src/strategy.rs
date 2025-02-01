use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorGroup;
use crate::collision::SENSOR_GROUPS;
use crate::registry::ActorPropsByType;
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

const APPROACH_MERGIN: f32 = 8.0;

#[cfg(feature = "debug")]
use bevy_egui::egui;
#[cfg(feature = "debug")]
use bevy_egui::EguiContext;

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

    #[allow(dead_code)]
    pub actor: Actor,
}

fn update(
    registry: Registry,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut query: Query<(Entity, &mut Actor, &Transform)>,
    navmesh: Query<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    let context: &RapierContext = rapier_context.single();

    let map: HashMap<Entity, (Actor, Vec2, f32)> = query
        .iter()
        .map(|(e, a, t)| {
            let props = registry.get_actor_props(&a.actor_type);
            (
                e,
                (a.clone(), t.translation.truncate(), props.collider.size()),
            )
        })
        .collect();

    for (entity, mut actor, actor_transform) in query.iter_mut() {
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

        let origin = actor_transform.translation.truncate();

        let self_actor_group = actor.actor_group;

        // 現在のアクションを実行する
        execute(
            context,
            &map,
            entity,
            self_actor_group,
            origin,
            &props,
            &mut actor,
            &navmesh,
            &navmeshes,
            next_action_index,
            action,
        );

        // アクションの match の下に置くと、continueで抜けたときにカウントアップされなくなってしまうことに注意
        actor.commander.count += 1;

        // 各アクションのタイムアウトを超えている場合は次のアクションへ移行
        let limit = match action {
            Action::Sleep => u32::MAX,
            Action::Wait { count, .. } => *count,
            Action::Approach { count, .. } => *count,
            Action::Fire { count, .. } => *count,
            Action::GoTo { count, .. } => *count,
            Action::Loop => u32::MAX,
            Action::End => u32::MAX,
        };
        if limit < actor.commander.count {
            actor.commander.count = 0;
            actor.commander.next_action += 1;
        }
    }
}

fn execute(
    context: &RapierContext,
    map: &HashMap<Entity, (Actor, Vec2, f32)>,
    entity: Entity,
    self_actor_group: ActorGroup,
    origin: Vec2,
    props: &ActorPropsByType,
    actor: &mut Actor,
    navmesh: &Query<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    navmeshes: &Res<Assets<NavMesh>>,
    next_action_index: usize,
    action: &Action,
) {
    match action {
        Action::Sleep => {
            // do nothing
            actor.fire_state = ActorFireState::Idle;
            actor.fire_state_secondary = ActorFireState::Idle;
            actor.move_direction = Vec2::ZERO;
            actor.navigation_path.clear();
        }
        Action::Wait { count: _, random } => {
            // このアクションを開始したときに、ランダムにcountを追加することで
            // アクションの完了時間をばらつかせます
            if actor.commander.count == 0 {
                actor.commander.count = rand::random::<u32>() % random;
            }

            actor.fire_state = ActorFireState::Idle;
            actor.fire_state_secondary = ActorFireState::Idle;
            actor.move_direction = Vec2::ZERO;
            actor.navigation_path.clear();
        }
        Action::Approach {
            far,
            near,
            count: _,
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
            let Some(nearest) = find_target(context, &map, entity, self_actor_group, origin, *far)
            else {
                return;
            };

            // 指定した距離まで近づいたら次のアクションへ移行
            if (origin - nearest.position).length() < *near {
                actor.move_direction = Vec2::ZERO;
                actor.commander.count = 0;
                actor.commander.next_action += 1;
                return;
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
                        actor.navigation_path = path.path.clone();

                        // ナビメッシュで次の目的地を選定
                        // APPROACH_MERGIN以下の近すぎるものは避ける
                        if let Some(first) = path
                            .path
                            .iter()
                            .filter(|p| APPROACH_MERGIN < (origin - **p).length())
                            .collect::<Vec<&Vec2>>()
                            .first()
                        {
                            **first
                        } else {
                            // ここに来ることはない？
                            warn!("first not found");
                            actor.commander.destination
                        }
                    } else {
                        // warn!("path not found, from {:?} to {:?}", from, to);
                        actor.commander.destination
                    }
                } else {
                    warn!("navmesh not found");
                    actor.commander.destination
                }
            } else {
                warn!("navmesh not built");
                actor.commander.destination
            };

            actor.commander.destination = destination;

            let diff = destination - origin;

            // 移動先は APPROACH_MERGIN 以上のものをフィルタしたので、
            // ここではAPPROACH_MERGIN以上は遠いはずで、ベクトルはゼロにはならないはず
            actor.move_direction = diff.normalize_or_zero();
        }
        Action::Fire {
            count: _,
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
                return;
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
                return;
            }

            // 相手に狙いを合わせる
            actor.pointer = nearest.position - origin;
            actor.move_direction = Vec2::ZERO;
            actor.fire_state = ActorFireState::Fire;
            actor.fire_state_secondary = ActorFireState::Idle;
            actor.navigation_path.clear();
        }
        Action::GoTo { destination, .. } => {
            actor.fire_state = ActorFireState::Idle;
            actor.fire_state_secondary = ActorFireState::Idle;
            actor.navigation_path.clear();

            let dest = match destination {
                Destination::HomePosition => actor.home_position,
            };
            let diff = dest - origin;
            if diff.length() < 8.0 {
                actor.move_direction = Vec2::ZERO;
                actor.commander.count = 0;
                actor.commander.next_action += 1;
                return;
            }

            actor.move_direction = diff.normalize_or_zero();
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
                return;
            };
            actor.commander.count = 0;
            actor.commander.next_action = index;
        }
    }
}

fn find_target(
    context: &RapierContext,
    map: &HashMap<Entity, (Actor, Vec2, f32)>,
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
                    if e_g.actor_group != self_actor_group && e_g.actor_group != ActorGroup::Entity
                    {
                        enemies.push(FindResult {
                            position: *e_t,
                            radius: *e_r,
                            actor: e_g.clone(),
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

#[cfg(feature = "debug")]
fn debug_draw(
    mut contexts_query: Query<&mut EguiContext>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    actor_query: Query<(&Actor, &Transform)>,
) {
    let (camera, camera_transform) = camera_query.single();
    let mut contexts = contexts_query.single_mut();
    let context = contexts.get_mut();

    egui::Area::new(egui::Id::new("overlay"))
        .fixed_pos([0.0, 0.0])
        .show(context, |ui| {
            let painter = ui.painter();
            for (actor, transform) in actor_query.iter() {
                if actor.navigation_path.is_empty() {
                    continue;
                }
                let mut points = Vec::new();
                let Ok(origin) = camera.world_to_viewport(&camera_transform, transform.translation)
                else {
                    return;
                };
                points.push(egui::pos2(origin.x, origin.y));
                for point in &actor.navigation_path {
                    let Ok(in_screen) =
                        camera.world_to_viewport(&camera_transform, point.extend(0.0))
                    else {
                        return;
                    };
                    points.push(egui::pos2(in_screen.x, in_screen.y));
                }
                painter.line(points, egui::Stroke::new(2.0, egui::Color32::RED));
            }
        });
}

pub struct StrategyPlugin;

impl Plugin for StrategyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update,).run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );

        #[cfg(feature = "debug")]
        app.add_systems(Update, debug_draw.run_if(in_state(GameState::InGame)));
    }
}
