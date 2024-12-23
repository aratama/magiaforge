use crate::asset::GameAssets;
use crate::constant::*;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorGroup;
use crate::hud::life_bar::LifeBarResource;
use crate::physics::compare_distance;
use crate::set::GameSet;
use crate::spell::SpellType;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug)]
pub struct SlimeControl {
    wait: u32,
}

const ENEMY_MOVE_FORCE: f32 = 100000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_RANGE: f32 = TILE_SIZE * 1.0;

pub fn spawn_slime(
    mut commands: &mut Commands,
    aseprite: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
    initial_wait: u32,
    gold: u32,
    group: ActorGroup,
    owner: Option<Entity>,
) {
    spawn_basic_enemy(
        &mut commands,
        match group {
            ActorGroup::Player => aseprite.friend_slime.clone(),
            ActorGroup::Enemy => aseprite.slime.clone(),
        },
        position,
        life_bar_locals,
        SlimeControl { wait: initial_wait },
        "slime",
        SpellType::SlimeCharge,
        ENEMY_MOVE_FORCE,
        gold,
        group,
        owner,
        15,
    );
}

/// 1マス以上5マス以内にプレイヤーがいたら追いかけます
/// また、プレイヤーを狙います
fn control_slime(
    mut actor_query: Query<(
        Entity,
        Option<&mut SlimeControl>,
        &mut Actor,
        &mut Transform,
    )>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let context: &RapierContext = rapier_context.single();

    // 多対多の参照になるので、HashMapでキャッシュしておく
    let map: HashMap<Entity, (ActorGroup, Vec2)> = actor_query
        .iter()
        .map(|(e, _, a, t)| (e, (a.actor_group, t.translation.truncate())))
        .collect();

    // 各スライムの行動を選択します
    for (slime_entity, slime_optional, mut slime_actor, slime_transform) in actor_query.iter_mut() {
        if let Some(mut slime) = slime_optional {
            slime_actor.move_direction = Vec2::ZERO;
            slime_actor.fire_state = ActorFireState::Idle;

            if 0 < slime.wait {
                slime.wait -= 1;
                continue;
            }

            // 指定した範囲にいる、自分以外で、かつ別のグループに所属するアクターの一覧を取得
            let mut enemies: Vec<Vec2> = Vec::new();
            context.intersections_with_shape(
                slime_transform.translation.truncate(),
                0.0,
                &Collider::ball(ENEMY_DETECTION_RANGE),
                QueryFilter {
                    groups: Some(CollisionGroups::new(ENEMY_GROUP, WITCH_GROUP | ENEMY_GROUP)),
                    ..default()
                },
                |e| {
                    if e != slime_entity {
                        if let Some((e_g, e_t)) = map.get(&e) {
                            if *e_g != slime_actor.actor_group {
                                enemies.push(*e_t);
                            }
                        }
                    }
                    true // 交差図形の検索を続ける
                },
            );

            // 最も近くにいる、別グループのアクターに対して接近または攻撃
            let origin = slime_transform.translation.truncate();
            enemies.sort_by(compare_distance(origin));
            if let Some(nearest) = enemies.first() {
                let diff = nearest - origin;
                if diff.length() < ENEMY_ATTACK_RANGE {
                    slime_actor.move_direction = Vec2::ZERO;
                    slime_actor.pointer = diff;
                    slime_actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    slime_actor.move_direction = diff.normalize_or_zero();
                    slime_actor.fire_state = ActorFireState::Idle;
                }
            }
        }
    }
}

pub struct SlimeControlPlugin;

impl Plugin for SlimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (control_slime)
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
