use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::vertical::Vertical;
use crate::component::flip::Flip;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGold;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorProps;
use crate::entity::bullet::HomingTarget;
use crate::finder::Finder;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::Inventory;
use crate::level::entities::SpawnEntity;
use crate::random::randomize_velocity;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

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

#[derive(Component, Debug)]
pub struct ChildSprite;

pub fn spawn_salamander(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_locals: &Res<LifeBarResource>,
    actor_group: ActorGroup,
    position: Vec2,
) -> Entity {
    let radius = 8.0;
    let golds = 10;
    let spell = Some(SpellType::Fireball);
    let mut builder = commands.spawn((
        Name::new("salamander"),
        StateScoped(GameState::InGame),
        DespawnWithGold { golds },
        Actor::new(ActorProps {
            uuid: Uuid::new_v4(),
            angle: 0.0,
            point_light_radius: 0.0,
            radius,
            move_force: 100000.0,
            current_wand: 0,
            actor_group,
            golds,
            inventory: Inventory::new(),
            wands: Wand::single(spell),
            fire_resistance: true,
        }),
        EntityDepth::new(),
        Life::new(100),
        HomingTarget,
        Transform::from_translation(position.extend(SHADOW_LAYER_Z)),
        GlobalTransform::default(),
        Visibility::default(),
        (
            RigidBody::Dynamic,
            Collider::ball(radius),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveEvents::COLLISION_EVENTS,
            actor_group.to_groups(0.0, 0),
        ),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "chicken_shadow".into(),
        },
    ));

    builder.with_children(|mut parent| {
        parent.spawn((
            ChildSprite,
            Vertical::new(0.0, -0.1),
            Flip,
            LifeBeingSprite,
            CounterAnimated,
            AseSpriteAnimation {
                aseprite: assets.salamander.clone(),
                animation: Animation::tag("idle"),
            },
        ));

        spawn_life_bar(&mut parent, &life_bar_locals);
    });

    builder.id()
}

fn transition(
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&lens.query());
    for (entity, shadow, actor, transform) in query.iter_mut() {
        if let Some(mut shadow) = shadow {
            let origin = transform.translation.truncate();
            let nearest = finder.nearest(&rapier_context, entity, ENEMY_DETECTION_RANGE);
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
                        if diff.length() < actor.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
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
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&lens.query());
    for (entity, shadow, mut actor, transform) in query.iter_mut() {
        if let Some(_) = shadow {
            let origin = transform.translation.truncate();
            let nearest = finder.nearest(&rapier_context, entity, ENEMY_DETECTION_RANGE);
            if let Some(nearest) = nearest {
                let diff = nearest.position - origin;
                actor.pointer = diff; // 火球を当てるためにここでは正規化しない
            }
        }
    }
}

fn animate(
    query: Query<&Salamander>,
    mut sprite_query: Query<(&Parent, &mut AseSpriteAnimation), With<ChildSprite>>,
) {
    for (parent, mut animation) in sprite_query.iter_mut() {
        if let Ok(shadow) = query.get(parent.get()) {
            match shadow.state {
                State::Wait(count) if count == 0 => {
                    animation.animation.tag = Some("idle".to_string());
                    animation.animation.repeat = AnimationRepeat::Loop;
                }
                State::Approarch(count) if count == 0 => {
                    animation.animation.tag = Some("run".to_string());
                    animation.animation.repeat = AnimationRepeat::Loop;
                }
                State::Attack(count) if count == 0 => {
                    animation.animation.tag = Some("idle".to_string());
                    animation.animation.repeat = AnimationRepeat::Loop;
                }
                _ => {}
            }
        }
    }
}

fn approach(
    mut query: Query<(Entity, Option<&mut Salamander>, &mut Actor, &mut Transform)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&lens.query());

    for (entity, shadow, mut actor, transform) in query.iter_mut() {
        if let Some(shadow) = shadow {
            match shadow.state {
                State::Approarch(count) if 60 < count => {
                    let origin = transform.translation.truncate();
                    if let Some(nearest) =
                        finder.nearest(&rapier_context, entity, ENEMY_DETECTION_RANGE)
                    {
                        let diff = nearest.position - origin;
                        if diff.length() < actor.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            actor.move_direction = Vec2::ZERO;
                            actor.move_force = 0.0;
                        } else if diff.length() < ENEMY_DETECTION_RANGE {
                            actor.move_direction = diff.normalize_or_zero();
                            actor.move_force = 100000.0;
                        }
                    } else {
                        actor.move_direction = Vec2::ZERO;
                        actor.move_force = 0.0;
                    }
                }

                _ => {
                    actor.move_direction = Vec2::ZERO;
                    actor.move_force = 0.0;
                }
            }
        }
    }
}

fn attack(
    mut query: Query<(
        Entity,
        Option<&mut Salamander>,
        &mut Actor,
        &mut Transform,
        &mut Life,
    )>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut spawn: EventWriter<SpawnEntity>,
) {
    let mut lens = query.transmute_lens_filtered::<(Entity, &Actor, &Transform), ()>();
    let finder = Finder::new(&lens.query());
    for (entity, shadow, actor, transform, _) in query.iter_mut() {
        if let Some(shadow) = shadow {
            match shadow.state {
                // 攻撃モーションを開始してから実際にダメージが発生するまで20フレームあり、
                // そのあいだに範囲外に逃げればダメージは受けない
                State::Attack(count) if count == 20 => {
                    let origin = transform.translation.truncate();
                    if let Some(nearest) =
                        finder.nearest(&rapier_context, entity, ENEMY_DETECTION_RANGE)
                    {
                        let diff = nearest.position - origin;
                        if diff.length() < actor.radius + nearest.radius + ENEMY_ATTACK_MARGIN {
                            let actor_position = transform.translation.truncate();
                            let position = actor_position + actor.pointer.normalize_or_zero() * 8.0;
                            let velocity = randomize_velocity(actor.pointer * 1.2, 0.5, 0.5);
                            spawn.send(SpawnEntity::Fireball {
                                position,
                                velocity,
                                actor_group: actor.actor_group,
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
            (transition, animate, approach, attack, pointer)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
