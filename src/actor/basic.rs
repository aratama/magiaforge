use crate::actor::Actor;
use crate::actor::ActorSpriteGroup;
use crate::actor::ActorState;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::constant::*;
use crate::controller::servant::Servant;
use crate::registry::ActorCollider;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use super::witch::WitchWandSprite;
use super::ActorType;

#[derive(Component, Debug)]
pub struct BasicActor;

#[derive(Component, Debug)]
pub struct BasicActorSprite;

/// 敵モンスターの共通の構造です
/// スプライトには idle, run, staggered, frozen のアニメーションが必要です
pub fn spawn_basic_actor(
    commands: &mut Commands,
    registry: &Registry,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    master: Option<Entity>,
    actor: Actor,
) -> Entity {
    let actor_type = actor.to_type();
    let actor_group = actor.actor_group;
    let props = registry.get_actor_props(actor_type);
    let mut builder = commands.spawn((
        BasicActor,
        Name::new(format!("{:?}", actor.to_type())),
        actor,
        Transform::from_translation(position.extend(0.0)),
        Counter::default(),
        match props.collider {
            ActorCollider::Ball(radius) => Collider::ball(radius),
            ActorCollider::Cuboid(width, height) => Collider::cuboid(width, height),
        },
        actor_group.to_groups(0.0, 0),
    ));

    builder.with_children(|parent| {
        parent.spawn((
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "chicken_shadow".into(),
            },
            Transform::from_xyz(0.0, 0.0, SHADOW_LAYER_Z),
        ));

        parent.spawn(ActorSpriteGroup).with_children(|parent| {
            parent.spawn((
                BasicActorSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite,
                    animation: Animation::default().with_tag(props.animations.idle_r.clone()),
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));

            if actor_type == ActorType::Witch {
                parent.spawn((
                    WitchWandSprite,
                    AseSpriteSlice {
                        aseprite: registry.assets.atlas.clone(),
                        name: "wand_cypress".into(),
                    },
                    Transform::from_xyz(0.0, 4.0, -0.0001),
                ));
            }
        });
    });

    if actor_type == ActorType::Witch {
        builder.insert((
            // 足音
            // footsteps.rsで音量を調整
            AudioPlayer::new(registry.assets.taiikukan.clone()),
            PlaybackSettings {
                volume: Volume::new(0.0),
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            },
        ));
    }

    if let Some(owner) = master {
        builder.insert(Servant { master: owner });
    }

    builder.id()
}

/// frozen, staggerd, run, idle のみからなる基本的なアニメーションを実装します
/// これ以外の表現が必要な場合は各アクターで個別に実装して上書きします
pub fn basic_animate(
    query: Query<&Actor, With<BasicActor>>,
    registry: Registry,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<
        (&Parent, &mut Sprite, &mut AseSpriteAnimation),
        With<BasicActorSprite>,
    >,
) {
    for (parent, mut sprite, mut animation) in sprite_query.iter_mut() {
        if let Ok(group) = group_query.get(parent.get()) {
            if let Ok(actor) = query.get(group.get()) {
                let props = registry.get_actor_props(actor.to_type());

                let angle = actor.pointer.to_angle();
                let pi = std::f32::consts::PI;

                animation.animation.repeat = AnimationRepeat::Loop;
                animation.animation.tag = Some(if 0 < actor.frozen {
                    props.animations.frozen.clone()
                } else if 0 < actor.drown {
                    props.animations.drown.clone()
                } else if 0 < actor.staggered {
                    props.animations.staggered.clone()
                } else {
                    match actor.state {
                        ActorState::Idle => {
                            if angle < pi * -0.75 || pi * 0.75 < angle {
                                props.animations.idle_r.clone()
                            } else if pi * 0.25 < angle && angle < pi * 0.75 {
                                props.animations.idle_u.clone()
                            } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                                props.animations.idle_d.clone()
                            } else {
                                props.animations.idle_r.clone()
                            }
                        }
                        ActorState::Run => {
                            if angle < pi * -0.75 || pi * 0.75 < angle {
                                props.animations.run_r.clone()
                            } else if pi * 0.25 < angle && angle < pi * 0.75 {
                                props.animations.run_u.clone()
                            } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                                props.animations.run_d.clone()
                            } else {
                                props.animations.run_r.clone()
                            }
                        }
                    }
                });

                sprite.flip_x = match actor.state {
                    ActorState::Idle => {
                        if angle < pi * -0.75 || pi * 0.75 < angle {
                            true
                        } else if pi * 0.25 < angle && angle < pi * 0.75 {
                            false
                        } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                            false
                        } else {
                            false
                        }
                    }
                    ActorState::Run => {
                        if angle < pi * -0.75 || pi * 0.75 < angle {
                            true
                        } else if pi * 0.25 < angle && angle < pi * 0.75 {
                            false
                        } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                            false
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

fn flip(
    actor_query: Query<&Actor, With<BasicActor>>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<
        (&Parent, &mut Sprite),
        (With<BasicActorSprite>, Without<ActorSpriteGroup>),
    >,
) {
    for (parent, mut sprite) in sprite_query.iter_mut() {
        let parent = group_query.get(parent.get()).unwrap();
        let chicken = actor_query.get(parent.get()).unwrap();
        sprite.flip_x = chicken.pointer.x < 0.0;
    }
}

pub struct BasicEnemyPlugin;

impl Plugin for BasicEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (basic_animate, flip).in_set(FixedUpdateGameActiveSet),
        );
    }
}
