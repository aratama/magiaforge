use crate::asset::GameAssets;
use crate::component::counter::Counter;
use crate::component::life::Life;
use crate::controller::player::Player;
use crate::enemy::chicken::spawn_chiken;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::salamander::spawn_salamander;
use crate::enemy::sandbug::spawn_sandbag;
use crate::enemy::shadow::spawn_shadow;
use crate::enemy::slime::spawn_slime;
use crate::enemy::spider::spawn_spider;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::bullet_particle::SpawnParticle;
use crate::hud::life_bar::LifeBarResource;
use crate::level::entities::SpawnEnemyType;
use crate::level::entities::SpawnEntity;
use crate::level::entities::SpawnWitch;
use crate::level::entities::SpawnWitchType;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
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
    player: &Option<&Player>,
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

    let discovered_spells = player
        .map(|p| p.discovered_spells.clone())
        .unwrap_or_default();
    let mut builder = commands.entity(entity);
    builder.insert(Player::new(
        player.map(|p| p.name.clone()).unwrap_or_default(),
        false,
        &discovered_spells,
    ));
    builder.insert(Metamorphosis {
        witch: SpawnWitch {
            wands: actor.wands.clone(),
            inventory: actor.inventory.clone(),
            witch_type: SpawnWitchType::Player,
            wand: actor.current_wand,
            getting_up: false,
            name: "".to_string(),
            life: actor_life.life,
            max_life: actor_life.max_life,
            golds: actor.golds,
            discovered_spells,
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
