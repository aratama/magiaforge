use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorFireState;
use crate::actor::ActorGroup;
use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::constant::*;
use crate::enemy::basic::spawn_basic_enemy;
use crate::finder::Finder;
use crate::hud::life_bar::LifeBarResource;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct SlimeControl {
    pub wait: u32,
}

impl Default for SlimeControl {
    fn default() -> Self {
        Self { wait: 5 }
    }
}

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_MARGIN: f32 = TILE_SIZE * 0.5;

pub fn default_slime() -> (Actor, Life) {
    (
        Actor {
            extra: ActorExtra::Slime,
            actor_group: ActorGroup::Enemy,
            wands: Wand::single(Some(SpellType::SlimeCharge)),
            ..default()
        },
        Life::new(15),
    )
}

pub fn spawn_slime(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_locals: &Res<LifeBarResource>,
    actor: Actor,
    life: Life,
    position: Vec2,
    owner: Option<Entity>,
) -> Entity {
    let actor_group = actor.actor_group;
    spawn_basic_enemy(
        &mut commands,
        &assets,
        life_bar_locals,
        match actor_group {
            ActorGroup::Friend => assets.friend_slime.clone(),
            ActorGroup::Enemy => assets.slime.clone(),
            ActorGroup::Neutral => assets.friend_slime.clone(),
            ActorGroup::Entity => assets.friend_slime.clone(),
        },
        position,
        "slime",
        owner,
        actor,
        life,
    )
}

/// 1マス以上5マス以内にプレイヤーがいたら追いかけます
/// また、プレイヤーを狙います
fn control_slime(
    mut query: Query<(
        Entity,
        Option<&mut SlimeControl>,
        &mut Actor,
        &mut Transform,
    )>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&lens.query());

    // 各スライムの行動を選択します
    for (slime_entity, slime_optional, mut slime_actor, slime_transform) in query.iter_mut() {
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
                if diff.length() < slime_actor.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
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

fn blood(
    mut commands: Commands,
    assets: Res<GameAssets>,
    query: Query<(&Life, &Transform), With<SlimeControl>>,
) {
    for (life, transform) in query.iter() {
        if life.life <= 0 {
            let position = transform.translation.truncate();
            commands.spawn((
                StateScoped(GameState::InGame),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: format!("slime_blood_{}", rand::random::<u8>() % 3),
                },
                Transform::from_translation(position.extend(BLOOD_LAYER_Z))
                    .with_scale(Vec3::new(2.0, 2.0, 1.0)),
            ));
        }
    }
}

pub struct SlimeControlPlugin;

impl Plugin for SlimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (control_slime, blood).in_set(FixedUpdateGameActiveSet),
        );
    }
}
