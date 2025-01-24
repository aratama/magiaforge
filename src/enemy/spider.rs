use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::constant::TILE_SIZE;
use crate::enemy::basic::spawn_basic_enemy;
use crate::finder::Finder;
use crate::hud::life_bar::LifeBarResource;
use crate::level::entities::SpawnEntity;
use crate::level::entities::SpawnEntityEvent;
use crate::registry::*;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const ENEMY_ATTACK_MARGIN: f32 = TILE_SIZE * 0.5;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 25.0;

#[derive(Debug)]
enum State {
    Wait(u32),
    Approarch(u32),
    Attack(u32),
}

#[derive(Component, Debug)]
pub struct Spider {
    state: State,
}

impl Default for Spider {
    fn default() -> Self {
        Self {
            state: State::Wait(60),
        }
    }
}

pub fn default_spider() -> Actor {
    Actor {
        extra: ActorExtra::Spider,
        actor_group: ActorGroup::Enemy,
        life: 40,
        max_life: 40,
        golds: 6,
        ..default()
    }
}

pub fn spawn_spider(
    mut commands: &mut Commands,
    registry: &Registry,
    life_bar_locals: &Res<LifeBarResource>,
    position: Vec2,
    actor: Actor,
) -> Entity {
    spawn_basic_enemy(
        &mut commands,
        &registry,
        &life_bar_locals,
        registry.assets.spider.clone(),
        position,
        "spider",
        None,
        actor,
    )
}

fn transition(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Spider>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut spawn: EventWriter<SpawnEntityEvent>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());
    for (entity, spider, actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());

        if let Some(mut shadow) = spider {
            let origin = transform.translation.truncate();
            let nearest = finder.nearest_opponent(&rapier_context, entity, ENEMY_DETECTION_RANGE);
            match shadow.state {
                State::Wait(count) if 0 < count => {
                    shadow.state = State::Wait(count - 1);
                }
                State::Wait(_) => {
                    shadow.state = State::Approarch(120 + rand::random::<u32>() % 120);
                }
                State::Approarch(count) if 0 < count => {
                    if let Some(nearest) = nearest {
                        let diff = nearest.position - origin;
                        if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            shadow.state = State::Attack(30 + rand::random::<u32>() % 30);
                        } else {
                            shadow.state = State::Approarch(count - 1);
                        }
                    } else {
                        shadow.state = State::Approarch(count - 1);
                    }
                }
                State::Approarch(_) => {
                    spawn.send(SpawnEntityEvent {
                        position: origin,
                        entity: SpawnEntity::Web {
                            actor_group: actor.actor_group,
                        },
                    });
                    shadow.state = State::Wait(30 + rand::random::<u32>() % 30);
                }
                State::Attack(count) if 0 < count => {
                    shadow.state = State::Attack(count - 1);
                }
                State::Attack(_) => {
                    shadow.state = State::Wait(30 + rand::random::<u32>() % 30);
                }
            }
        }
    }
}

fn pointer(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Spider>, &mut Actor, &mut Transform)>,
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
                actor.pointer = diff.normalize_or_zero();
            }
        }
    }
}

fn approach(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Spider>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());

    for (entity, shadow, mut actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());

        if let Some(shadow) = shadow {
            match shadow.state {
                State::Approarch(..) => {
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
    mut query: Query<(Entity, Option<&mut Spider>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut actor_event: EventWriter<ActorEvent>,
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
                            actor_event.send(ActorEvent::Damaged {
                                actor: nearest.entity,
                                position: nearest.position,
                                damage: 4,
                                fire: false,
                                impulse: Vec2::ZERO,
                                stagger: 30,
                                metamorphose: None,
                                dispel: false,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (transition, approach, attack, pointer)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
