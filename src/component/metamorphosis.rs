use crate::actor::get_default_actor;
use crate::actor::spawn_actor;
use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::entity::bullet_particle::SpawnParticle;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::KYUSHU2_SHORT;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::TimeState;
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// 魔女が変化したあとのモンスターであることを表します
/// 変化残り時間がゼロになったら、このエンティティを削除して、その位置に魔女をスポーンします
/// このコンポーネントには Actor を含むため、 Actor 内に状態異常として含めることはできません
#[derive(Component, Clone)]
pub struct Metamorphosed {
    /// 変身の残り時間
    /// 0 になったら変身が解除されます
    pub count: u32,
    pub original_actor: Actor,
}

pub fn random_actor_type(mut rng: &mut ThreadRng, except: &ActorType) -> ActorType {
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
    .filter(|a| **a != *except)
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
    asset_server: &Res<AssetServer>,
    registry: &Registry,
    se: &mut EventWriter<SEEvent>,
    spawn: &mut EventWriter<SpawnEvent>,

    original_actor_entity: &Entity,
    original_actor: Actor,
    original_morph: &Option<&Metamorphosed>,

    position: Vec2,
    morphed_type: &ActorType,
) -> Entity {
    let original_actor = original_morph
        .map(|r| r.original_actor.clone())
        .unwrap_or(original_actor);

    let mut dest_actor = get_default_actor(&registry, &morphed_type);

    dest_actor.fire_state = original_actor.fire_state;
    dest_actor.fire_state_secondary = original_actor.fire_state_secondary;
    dest_actor.move_direction = original_actor.move_direction;
    dest_actor.actor_group = match (original_actor.actor_group, dest_actor.actor_group) {
        (_, ActorGroup::Entity) => ActorGroup::Entity,
        (ActorGroup::Entity, _) => {
            if rand::random::<bool>() {
                ActorGroup::Friend
            } else {
                ActorGroup::Enemy
            }
        }
        _ => original_actor.actor_group,
    };

    dest_actor.life = dest_actor.life
        * (original_actor.life as f32 / original_actor.max_life as f32).ceil() as u32;

    commands.entity(*original_actor_entity).despawn_recursive();

    let entity = spawn_actor(
        &mut commands,
        &asset_server,
        &registry,
        position,
        dest_actor,
    );

    let mut builder = commands.entity(entity);

    builder.insert(Metamorphosed {
        count: 60 * 10,
        original_actor,
    });
    se.send(SEEvent::pos(KYUSHU2_SHORT, position));
    spawn.send(SpawnEvent {
        position,
        spawn: Spawn::Particle {
            particle: metamorphosis_effect(),
        },
    });

    entity
}

fn revert(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Metamorphosed, &Transform, &Actor)>,
    mut se: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEvent>,
    time: Res<State<TimeState>>,
) {
    if *time == TimeState::Inactive {
        return;
    }
    for (entity, mut metamorphosis, transform, actor) in query.iter_mut() {
        if 0 < metamorphosis.count {
            metamorphosis.count -= 1;
            continue;
        }
        if metamorphosis.count == 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();

            // 変身を詠唱直後なので original_actor は fire_state が Fire のままになっており、
            // このまま変身が解除されるとまた詠唱を行って変身してしまう場合があることに注意
            // fire_state は上書きする
            let mut original_actor = metamorphosis.original_actor.clone();
            original_actor.fire_state = actor.fire_state;
            original_actor.fire_state_secondary = actor.fire_state_secondary;
            original_actor.move_direction = actor.move_direction;
            original_actor.life =
                original_actor.life * (actor.life as f32 / actor.max_life as f32).ceil() as u32;

            spawn.send(SpawnEvent {
                position,
                spawn: Spawn::Respawn {
                    actor: original_actor,
                    player_controlled: false,
                },
            });
            spawn.send(SpawnEvent {
                position,
                spawn: Spawn::Particle {
                    particle: metamorphosis_effect(),
                },
            });
            se.send(SEEvent::pos(KYUSHU2_SHORT, position));
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
