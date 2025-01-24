use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorFireState;
use crate::actor::ActorGroup;
use crate::constant::*;
use crate::enemy::basic::spawn_basic_enemy;
use crate::finder::Finder;
use crate::hud::life_bar::LifeBarResource;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct EyeballControl;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_RANGE: f32 = TILE_SIZE * 8.0;

pub fn default_eyeball() -> Actor {
    Actor {
        extra: ActorExtra::Eyeball,
        actor_group: ActorGroup::Enemy,
        wands: Wand::single(Some(SpellType::PurpleBolt)),
        life: 25,
        max_life: 25,
        ..default()
    }
}

pub fn spawn_eyeball(
    mut commands: &mut Commands,
    registry: &Registry,
    life_bar_locals: &Res<LifeBarResource>,
    position: Vec2,
    actor: Actor,
) -> Entity {
    let actor_group = actor.actor_group;
    spawn_basic_enemy(
        &mut commands,
        &registry,
        life_bar_locals,
        match actor_group {
            ActorGroup::Friend => registry.assets.eyeball_friend.clone(),
            ActorGroup::Enemy => registry.assets.eyeball.clone(),
            ActorGroup::Neutral => registry.assets.eyeball_friend.clone(),
            ActorGroup::Entity => registry.assets.eyeball_friend.clone(),
        },
        position,
        "eyeball",
        None,
        actor,
    )
}

fn control_eyeball(
    registry: Registry,
    mut query: Query<(
        Entity,
        Option<&mut EyeballControl>,
        &mut Actor,
        &mut Transform,
    )>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());

    // 各アイボールの行動を選択します
    for (eyeball_entity, eyeball_optional, mut eyeball_actor, eyeball_transform) in query.iter_mut()
    {
        if let Some(_) = eyeball_optional {
            eyeball_actor.move_direction = Vec2::ZERO;
            eyeball_actor.fire_state = ActorFireState::Idle;

            // 最も近くにいる、別グループのアクターに対して接近または攻撃
            let origin = eyeball_transform.translation.truncate();
            if let Some(nearest) =
                finder.nearest_opponent(&rapier_context, eyeball_entity, ENEMY_DETECTION_RANGE)
            {
                let diff = nearest.position - origin;
                if diff.length() < ENEMY_ATTACK_RANGE {
                    eyeball_actor.move_direction = Vec2::ZERO;
                    eyeball_actor.pointer = diff;
                    eyeball_actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    eyeball_actor.move_direction = diff.normalize_or_zero();
                    eyeball_actor.fire_state = ActorFireState::Idle;
                }
            }
        }
    }
}

pub struct EyeballControlPlugin;

impl Plugin for EyeballControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (control_eyeball).in_set(FixedUpdateGameActiveSet),
        );
    }
}
