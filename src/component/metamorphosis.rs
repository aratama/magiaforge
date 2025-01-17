use super::{counter::Counter, life::Life};
use crate::{
    asset::GameAssets,
    controller::player::Player,
    enemy::{
        chicken::spawn_chiken, eyeball::spawn_eyeball, salamander::spawn_salamander,
        sandbug::spawn_sandbag, shadow::spawn_shadow, slime::spawn_slime, spider::spawn_spider,
    },
    entity::{
        actor::{Actor, ActorGroup},
        bullet_particle::SpawnParticle,
    },
    hud::life_bar::LifeBarResource,
    level::entities::{SpawnEnemyType, SpawnEntity, SpawnWitch, SpawnWitchType},
    se::{SEEvent, SE},
    set::FixedUpdateGameActiveSet,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// 魔女が変化したあとのモンスターであることを表します
/// 変化残り時間がゼロになったら、このエンティティを削除して、その位置に魔女をスポーンします
#[derive(Component)]
#[require(Counter)]
pub struct Metamorphosis {
    pub witch: SpawnWitch,
}

pub fn cast_metamorphosis(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_resource: &Res<LifeBarResource>,
    se: &mut EventWriter<SEEvent>,
    spawn: &mut EventWriter<SpawnEntity>,
    actor: &Actor,
    actor_life: &Life,
    actor_entity: &Entity,
    actor_transform: &Transform,
    mut rng: &mut ThreadRng,
) {
    commands.entity(*actor_entity).despawn_recursive();
    let position = actor_transform.translation.truncate();
    let actor_group = ActorGroup::Player;
    let enemy_type = *[
        SpawnEnemyType::Slime,
        SpawnEnemyType::Eyeball,
        SpawnEnemyType::Shadow,
        SpawnEnemyType::Spider,
        SpawnEnemyType::Salamander,
        SpawnEnemyType::Chiken,
        SpawnEnemyType::Sandbag,
    ]
    .choose(&mut rng)
    .unwrap();

    let entity = match enemy_type {
        SpawnEnemyType::Slime => spawn_slime(
            &mut commands,
            &assets,
            position,
            &life_bar_resource,
            0,
            actor_group,
            None,
        ),
        SpawnEnemyType::Eyeball => spawn_eyeball(
            &mut commands,
            &assets,
            position,
            &life_bar_resource,
            actor_group,
            8,
        ),
        SpawnEnemyType::Shadow => spawn_shadow(
            &mut commands,
            &assets,
            &life_bar_resource,
            actor_group,
            position,
        ),
        SpawnEnemyType::Spider => spawn_spider(
            &mut commands,
            &assets,
            &life_bar_resource,
            actor_group,
            position,
        ),
        SpawnEnemyType::Salamander => spawn_salamander(
            &mut commands,
            &assets,
            &life_bar_resource,
            actor_group,
            position,
        ),
        SpawnEnemyType::Chiken => {
            spawn_chiken(&mut commands, &assets, &life_bar_resource, position)
        }
        SpawnEnemyType::Sandbag => {
            spawn_sandbag(&mut commands, &assets, &life_bar_resource, position)
        }
    };

    let mut builder = commands.entity(entity);
    builder.insert(Player::new("".to_string(), false));
    builder.insert(Metamorphosis {
        witch: SpawnWitch {
            wands: actor.wands.clone(),
            inventory: actor.inventory.clone(),
            witch_type: SpawnWitchType::Player,
            getting_up: false,
            name: "".to_string(),
            life: actor_life.life,
            max_life: actor_life.max_life,
            golds: actor.golds,
        },
    });
    se.send(SEEvent::pos(SE::Kyushu2Short, position));
    spawn.send(SpawnEntity::Particle {
        position,
        spawn: metamorphosis_effect(),
    });
}

fn revert(
    mut commands: Commands,
    mut query: Query<(Entity, &Metamorphosis, &Counter, &Transform)>,
    mut se: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEntity>,
) {
    for (entity, metamorphosis, counter, transform) in query.iter_mut() {
        if 60 * 10 <= counter.count {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            spawn.send(SpawnEntity::SpawnWitch {
                position,
                witch: metamorphosis.witch.clone(),
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
