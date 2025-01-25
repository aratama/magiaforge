use crate::actor::basic::spawn_basic_actor;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::constant::*;
use crate::finder::Finder;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::random::randomize_velocity;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const ENEMY_ATTACK_MARGIN: f32 = TILE_SIZE * 6.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 20.0;

#[derive(Debug)]
enum State {
    Wait(u32),
    Approarch(u32),
    Attack(u32),
}

#[derive(Component, Debug)]
pub struct Salamander {
    state: State,
}

impl Default for Salamander {
    fn default() -> Self {
        Self {
            state: State::Wait(60),
        }
    }
}

pub fn default_salamander() -> Actor {
    Actor {
        extra: ActorExtra::Salamander,
        actor_group: ActorGroup::Enemy,
        wands: Wand::single(Some(Spell::new("Fireball"))),
        life: 100,
        max_life: 100,
        golds: 10,
        ..default()
    }
}

pub fn spawn_salamander(
    mut commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    spawn_basic_actor(
        &mut commands,
        &registry,
        registry.assets.salamander.clone(),
        position,
        None,
        actor,
    )
}

fn transition(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());
    for (entity, shadow, actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());

        if let Some(mut shadow) = shadow {
            let origin = transform.translation.truncate();
            let nearest = finder.nearest_opponent(&rapier_context, entity, ENEMY_DETECTION_RANGE);
            match shadow.state {
                State::Wait(count) if count < 30 => {
                    shadow.state = State::Wait(count + 1);
                }
                State::Wait(_) => {
                    shadow.state = State::Approarch(0);
                }
                State::Approarch(count) if count < 240 => {
                    if let Some(nearest) = nearest {
                        let diff = nearest.position - origin;
                        if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            shadow.state = State::Attack(0);
                        } else {
                            shadow.state = State::Approarch(count + 1);
                        }
                    } else {
                        shadow.state = State::Approarch(count + 1);
                    }
                }
                State::Approarch(_) => {
                    shadow.state = State::Wait(0);
                }
                State::Attack(count) if count < 60 => {
                    shadow.state = State::Attack(count + 1);
                }
                State::Attack(_) => {
                    shadow.state = State::Wait(0);
                }
            }
        }
    }
}

fn pointer(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());
    for (entity, shadow, mut actor, transform) in query.iter_mut() {
        if let Some(_) = shadow {
            let origin = transform.translation.truncate();
            let nearest = finder.nearest_opponent(&rapier_context, entity, ENEMY_DETECTION_RANGE);
            if let Some(nearest) = nearest {
                let diff = nearest.position - origin;
                actor.pointer = diff; // 火球を当てるためにここでは正規化しない
            }
        }
    }
}

fn approach(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());

    for (entity, shadow, mut actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());
        if let Some(shadow) = shadow {
            match shadow.state {
                State::Approarch(count) if 60 < count => {
                    let origin = transform.translation.truncate();
                    if let Some(nearest) =
                        finder.nearest_opponent(&rapier_context, entity, ENEMY_DETECTION_RANGE)
                    {
                        let diff = nearest.position - origin;
                        if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            actor.move_direction = Vec2::ZERO;
                        } else if diff.length() < ENEMY_DETECTION_RANGE {
                            actor.move_direction = diff.normalize_or_zero();
                        }
                    } else {
                        actor.move_direction = Vec2::ZERO;
                    }
                }

                _ => {
                    actor.move_direction = Vec2::ZERO;
                }
            }
        }
    }
}

fn attack(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut spawn: EventWriter<SpawnEvent>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());
    for (entity, shadow, actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());

        if let Some(shadow) = shadow {
            match shadow.state {
                // 攻撃モーションを開始してから実際にダメージが発生するまで20フレームあり、
                // そのあいだに範囲外に逃げればダメージは受けない
                State::Attack(count) if count == 20 => {
                    let origin = transform.translation.truncate();
                    if let Some(nearest) =
                        finder.nearest_opponent(&rapier_context, entity, ENEMY_DETECTION_RANGE)
                    {
                        let diff = nearest.position - origin;
                        if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            let actor_position = transform.translation.truncate();
                            let position = actor_position + actor.pointer.normalize_or_zero() * 8.0;
                            let velocity = randomize_velocity(actor.pointer * 1.2, 0.5, 0.5);
                            spawn.send(SpawnEvent {
                                position,
                                spawn: Spawn::Fireball {
                                    velocity,
                                    actor_group: actor.actor_group,
                                },
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct SalamanderPlugin;

impl Plugin for SalamanderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (transition, approach, attack, pointer)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
