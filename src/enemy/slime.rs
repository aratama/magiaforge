use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorFireState;
use crate::actor::ActorGroup;
use crate::constant::*;
use crate::finder::Finder;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use vleue_navigator::prelude::ManagedNavMesh;
use vleue_navigator::prelude::NavMeshStatus;
use vleue_navigator::NavMesh;

#[derive(Component, Debug)]
pub struct SlimeControl {
    pub wait: u32,
}

impl Default for SlimeControl {
    fn default() -> Self {
        Self { wait: 5 }
    }
}

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 20.0;

const ENEMY_ATTACK_MARGIN: f32 = TILE_SIZE * 0.5;

pub fn default_slime() -> Actor {
    Actor {
        extra: ActorExtra::Slime,
        actor_group: ActorGroup::Enemy,
        wands: Wand::single(Some(Spell::new("SlimeCharge"))),
        life: 15,
        max_life: 15,
        ..default()
    }
}

pub fn spawn_slime(
    mut commands: &mut Commands,
    registry: &Registry,
    actor: Actor,
    position: Vec2,
    owner: Option<Entity>,
) -> Entity {
    let actor_group = actor.actor_group;
    spawn_basic_actor(
        &mut commands,
        &registry,
        match actor_group {
            ActorGroup::Friend => registry.assets.friend_slime.clone(),
            ActorGroup::Enemy => registry.assets.slime.clone(),
            ActorGroup::Neutral => registry.assets.friend_slime.clone(),
            ActorGroup::Entity => registry.assets.friend_slime.clone(),
        },
        position,
        owner,
        actor,
    )
}

/// 1マス以上5マス以内にプレイヤーがいたら追いかけます
/// また、プレイヤーを狙います
fn control_slime(
    registry: Registry,
    mut query: Query<(
        Entity,
        Option<&mut SlimeControl>,
        &mut Actor,
        &mut Transform,
    )>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,

    navmesh: Query<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());

    // 各スライムの行動を選択します
    for (slime_entity, slime_optional, mut slime_actor, slime_transform) in query.iter_mut() {
        let props = registry.get_actor_props(slime_actor.to_type());

        if let Some(mut slime) = slime_optional {
            slime_actor.move_direction = Vec2::ZERO;
            slime_actor.fire_state = ActorFireState::Idle;

            if 0 < slime.wait {
                slime.wait -= 1;
                continue;
            }

            // 最も近くにいる、別グループのアクターに対して接近または攻撃
            let origin = slime_transform.translation.truncate();

            if let Some(nearest) =
                finder.nearest_opponent(&rapier_context, slime_entity, ENEMY_DETECTION_RANGE)
            {
                let diff = nearest.position - origin;
                if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                    slime_actor.move_direction = Vec2::ZERO;
                    slime_actor.pointer = diff;
                    slime_actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    let (navmesh_handle, status) = navmesh.single();
                    let navmesh = navmeshes.get(navmesh_handle);

                    if *status != NavMeshStatus::Built {
                        return;
                    }
                    let Some(navmesh) = navmesh else {
                        warn!("navmesh not found");
                        return;
                    };

                    let from = origin;
                    let to = nearest.position;

                    let Some(path) = navmesh.path(
                        // エンティティの位置そのものを使うと、壁際に近づいたときに agent_radius のマージンに埋もれて
                        // 到達不可能になってしまうので、タイルの中心を使います
                        from, to,
                    ) else {
                        // warn!("path not found");
                        continue;
                    };

                    // for p in path.path.iter() {
                    //     info!("{:?} {:?}", p.x / TILE_SIZE, p.y / TILE_SIZE);
                    // }

                    let Some(first) = path.path.first() else {
                        warn!("first not found");
                        continue;
                    };

                    slime_actor.move_direction = (*first - origin).normalize_or_zero();
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
            (control_slime).in_set(FixedUpdateGameActiveSet),
        );
    }
}
