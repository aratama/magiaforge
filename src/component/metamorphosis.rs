use super::life::Life;
use crate::actor::get_default_actor;
use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::asset::GameAssets;
use crate::entity::bullet_particle::SpawnParticle;
use crate::hud::life_bar::LifeBarResource;
use crate::level::entities::spawn_actor;
use crate::level::entities::SpawnEntity;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::TimeState;
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// 魔女が変化したあとのモンスターであることを表します
/// 変化残り時間がゼロになったら、このエンティティを削除して、その位置に魔女をスポーンします
/// このコンポーネントには Actor を含むため、 Actor 内に状態異常として含めることはできません
#[derive(Component, Clone)]
pub struct Metamorphosis {
    pub count: u32,
    pub original_actor: Actor,
    pub original_life: Life,
}

pub fn random_actor_type(mut rng: &mut ThreadRng, except: ActorType) -> ActorType {
    *[
        ActorType::Slime,
        ActorType::EyeBall,
        ActorType::Shadow,
        ActorType::Spider,
        ActorType::Salamander,
        ActorType::Chicken,
        ActorType::Sandbag,
        ActorType::Lantern,
        ActorType::Chest,
        ActorType::BookShelf,
    ]
    .iter()
    .filter(|a| **a != except)
    .collect::<Vec<&ActorType>>()
    .choose(&mut rng)
    .unwrap()
    .to_owned()
}

/// ActorGroupは基本的には変化しませんが、変化先が物体の場合は ENtityGroup になり、
/// 変化元が EntityGroup の場合は ActorGroup がランダムに割り当てられます
///
pub fn cast_metamorphosis(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_resource: &Res<LifeBarResource>,
    se: &mut EventWriter<SEEvent>,
    spawn: &mut EventWriter<SpawnEntity>,

    original_actor_entity: &Entity,
    original_actor: Actor,
    original_life: Life,
    original_morph: &Option<&Metamorphosis>,

    position: Vec2,
    morphed_type: ActorType,
) -> Entity {
    let (mut dest_actor, dest_life) = get_default_actor(morphed_type);

    dest_actor.actor_group = match (original_actor.actor_group, dest_actor.actor_group) {
        (_, ActorGroup::Entity) => ActorGroup::Entity,
        (ActorGroup::Entity, _) => {
            if rand::random::<bool>() {
                ActorGroup::Player
            } else {
                ActorGroup::Enemy
            }
        }
        _ => original_actor.actor_group,
    };

    commands.entity(*original_actor_entity).despawn_recursive();
    // info!("despawn {} {}", file!(), line!());

    let entity = spawn_actor(
        &mut commands,
        &assets,
        &life_bar_resource,
        position,
        dest_actor,
        dest_life,
    );

    let mut builder = commands.entity(entity);

    builder.insert(
        original_morph
            .map(|r| Metamorphosis {
                count: 60 * 10,
                original_actor: r.original_actor.clone(),
                original_life: r.original_life.clone(),
            })
            .unwrap_or(Metamorphosis {
                count: 60 * 10,
                original_actor,
                original_life,
            }),
    );
    se.send(SEEvent::pos(SE::Kyushu2Short, position));
    spawn.send(SpawnEntity::Particle {
        position,
        spawn: metamorphosis_effect(),
    });

    entity
}

fn revert(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Metamorphosis, &Transform)>,
    mut se: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEntity>,
    time: Res<State<TimeState>>,
) {
    if *time == TimeState::Inactive {
        return;
    }
    for (entity, mut metamorphosis, transform) in query.iter_mut() {
        if 0 < metamorphosis.count {
            metamorphosis.count -= 1;
            continue;
        }
        if metamorphosis.count == 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
            spawn.send(SpawnEntity::Actor {
                position,
                life: metamorphosis.original_life.clone(),
                actor: metamorphosis.original_actor.clone(),
            });
            spawn.send(SpawnEntity::Particle {
                position,
                spawn: metamorphosis_effect(),
            });
            se.send(SEEvent::pos(SE::Kyushu2Short, position));
        }
    }
}

pub fn metamorphosis_effect() -> SpawnParticle {
    SpawnParticle {
        scale: 4.0,
        count: 50,
        velocity_base: 0.1,
        velocity_random: 0.8,
        lifetime_base: 15,
        lifetime_random: 20,
    }
}

pub struct MetamorphosisPlugin;

impl Plugin for MetamorphosisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, revert.in_set(FixedUpdateGameActiveSet));
    }
}
