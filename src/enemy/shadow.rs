use crate::actor::basic::basic_animate;
use crate::actor::basic::spawn_basic_actor;
use crate::actor::basic::BasicActorSprite;
use crate::actor::collision_group_by_actor;
use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorSpriteGroup;
use crate::collision::SHADOW_GROUPS;
use crate::component::vertical::Vertical;
use crate::constant::*;
use crate::finder::Finder;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

const ENEMY_ATTACK_MARGIN: f32 = TILE_SIZE * 0.5;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

#[derive(Debug)]
enum State {
    Wait(u32),
    Hide(u32),
    Appear(u32),
    Attack(u32),
}

#[derive(Component, Debug)]
pub struct Shadow {
    state: State,
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            state: State::Wait(0),
        }
    }
}

pub fn default_shadow() -> Actor {
    Actor {
        extra: ActorExtra::Shadow,
        actor_group: ActorGroup::Enemy,
        life: 40,
        max_life: 40,
        golds: 10,
        ..default()
    }
}

pub fn spawn_shadow(
    mut commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    spawn_basic_actor(
        &mut commands,
        &registry,
        registry.assets.shadow.clone(),
        position,
        None,
        actor,
    )
}

fn transition(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Shadow>, &mut Actor, &mut Transform)>,
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
                State::Wait(count) if count < 60 => {
                    shadow.state = State::Wait(count + 1);
                }
                State::Wait(_) => {
                    shadow.state = State::Hide(0);
                }
                State::Hide(count) if count < 360 => {
                    if let Some(nearest) = nearest {
                        let diff = nearest.position - origin;
                        if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            shadow.state = State::Attack(0);
                        } else {
                            shadow.state = State::Hide(count + 1);
                        }
                    } else {
                        shadow.state = State::Hide(count + 1);
                    }
                }
                State::Hide(_) => {
                    shadow.state = State::Appear(0);
                }
                State::Appear(count) if count < 30 => {
                    shadow.state = State::Appear(count + 1);
                }
                State::Appear(_) => {
                    if let Some(nearest) = nearest {
                        let diff = nearest.position - origin;
                        if diff.length() < props.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            shadow.state = State::Attack(0);
                        } else {
                            shadow.state = State::Wait(0);
                        }
                    }
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
    mut query: Query<(Entity, Option<&mut Shadow>, &mut Actor, &mut Transform)>,
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

pub fn animate(
    query: Query<&Shadow>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<(&Parent, &mut AseSpriteAnimation), With<BasicActorSprite>>,
) {
    for (parent, mut animation) in sprite_query.iter_mut() {
        if let Ok(group) = group_query.get(parent.get()) {
            if let Ok(shadow) = query.get(group.get()) {
                match shadow.state {
                    State::Wait(_) => {
                        animation.animation.tag = Some("idle".to_string());
                        animation.animation.repeat = AnimationRepeat::Loop;
                    }
                    State::Hide(_) => {
                        animation.animation.tag = Some("hide".to_string());
                        animation.animation.repeat = AnimationRepeat::Count(1);
                    }
                    State::Appear(_) => {
                        animation.animation.tag = Some("appear".to_string());
                        animation.animation.repeat = AnimationRepeat::Count(1);
                    }
                    State::Attack(_) => {
                        animation.animation.tag = Some("attack".to_string());
                        animation.animation.repeat = AnimationRepeat::Count(1);
                    }
                }
            }
        }
    }
}

fn approach(
    registry: Registry,
    mut query: Query<(Entity, Option<&mut Shadow>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&registry, &lens.query());

    for (entity, shadow, mut actor, transform) in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());

        if let Some(shadow) = shadow {
            match shadow.state {
                State::Hide(count) if 60 < count => {
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
    mut query: Query<(Entity, Option<&mut Shadow>, &mut Actor, &mut Transform)>,
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

fn update_group_by_shadow(mut query: Query<(&Shadow, &Actor, &Vertical, &mut CollisionGroups)>) {
    for (shadow, actor, vertical, mut group) in query.iter_mut() {
        match shadow.state {
            State::Wait(_) => {
                *group = actor.actor_group.to_groups(vertical.v, actor.drowning);
            }
            State::Hide(_) => {
                *group = *SHADOW_GROUPS;
            }
            State::Appear(_) => {
                *group = actor.actor_group.to_groups(vertical.v, actor.drowning);
            }
            State::Attack(_) => {
                *group = actor.actor_group.to_groups(vertical.v, actor.drowning);
            }
        }
    }
}

pub struct ShadowPlugin;

impl Plugin for ShadowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                transition,
                animate.after(basic_animate),
                approach,
                attack,
                pointer,
                // Actor は　collision_group_by_actor　で CollisionGroups を毎フレーム選択しますが、
                // update_group_by_shadow はそれを上書きするため、は　collision_group_by_actor のあとに実行します
                update_group_by_shadow.after(collision_group_by_actor),
            )
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
