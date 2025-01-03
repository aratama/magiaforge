use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::falling::Falling;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGold;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorState;
use crate::entity::bullet::HomingTarget;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::Inventory;
use crate::set::GameSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::states::TimeState;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

#[derive(Debug)]
enum ShadowState {
    Wait(u32),
    Hide(u32),
    Appear(u32),
    Attack(u32),
}

#[derive(Component, Debug)]
pub struct Shadow {
    state: ShadowState,
}

#[derive(Component, Debug)]
pub struct ShadowSprite;

pub fn spawn_shadow(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_locals: &Res<LifeBarResource>,
    position: Vec2,
) -> Entity {
    let radius = 8.0;
    let golds = 10;
    let spell = Some(SpellType::Fireball);
    let actor_group = ActorGroup::Enemy;
    let mut builder = commands.spawn((
        Name::new("shadow"),
        StateScoped(GameState::InGame),
        DespawnWithGold { golds },
        Shadow {
            state: ShadowState::Wait(60),
        },
        Actor {
            uuid: Uuid::new_v4(),
            pointer: Vec2::ZERO,
            point_light_radius: 0.0,
            radius,
            move_direction: Vec2::ZERO,
            move_force: 100000.0,
            fire_state: ActorFireState::Idle,
            fire_state_secondary: ActorFireState::Idle,
            current_wand: 0,
            effects: default(),
            actor_group,
            golds,
            inventory: Inventory::new(),
            equipments: [None; MAX_ITEMS_IN_EQUIPMENT],
            wands: Wand::single(spell),
            state: ActorState::default(),
            wait: 0,
        },
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
            actor_group.to_groups(),
        ),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "chicken_shadow".into(),
        },
    ));

    builder.with_children(|mut parent| {
        parent.spawn((
            ShadowSprite,
            Falling::new(0.0, -0.1),
            LifeBeingSprite,
            CounterAnimated,
            AseSpriteAnimation {
                aseprite: assets.shadow.clone(),
                animation: Animation::tag("idle"),
            },
        ));

        spawn_life_bar(&mut parent, &life_bar_locals);
    });

    builder.id()
}

fn transition(mut query: Query<&mut Shadow>) {
    for mut shadow in query.iter_mut() {
        // info!("shadow {:?}", shadow.state);
        match shadow.state {
            ShadowState::Wait(ref mut count) if *count < 180 => {
                *count += 1;
            }
            ShadowState::Wait(_) => {
                shadow.state = ShadowState::Hide(0);
            }
            ShadowState::Hide(ref mut count) if *count < 360 => {
                *count += 1;
            }
            ShadowState::Hide(_) => {
                shadow.state = ShadowState::Appear(0);
            }
            ShadowState::Appear(ref mut count) if *count < 60 => {
                *count += 1;
            }
            ShadowState::Appear(_) => {
                shadow.state = ShadowState::Attack(0);
            }
            ShadowState::Attack(ref mut count) if *count < 60 => {
                *count += 1;
            }
            ShadowState::Attack(_) => {
                shadow.state = ShadowState::Wait(0);
            }
        }
    }
}

fn animate(
    query: Query<&Shadow>,
    mut sprite_query: Query<
        (&Parent, &mut AseSpriteAnimation, &mut AnimationState),
        With<ShadowSprite>,
    >,
) {
    for (parent, mut animation, mut animation_state) in sprite_query.iter_mut() {
        let shadow = query.get(parent.get()).unwrap();
        match shadow.state {
            ShadowState::Wait(count) if count == 0 => {
                animation.animation.tag = Some("idle".to_string());
                animation.animation.repeat = AnimationRepeat::Loop;
                animation_state.current_frame = 0;
            }
            ShadowState::Hide(count) if count == 0 => {
                animation.animation.tag = Some("hide".to_string());
                animation.animation.repeat = AnimationRepeat::Count(1);
                animation_state.current_frame = 2;
            }
            ShadowState::Appear(count) if count == 0 => {
                animation.animation.tag = Some("appear".to_string());
                animation.animation.repeat = AnimationRepeat::Count(1);
                animation_state.current_frame = 7;
            }
            ShadowState::Attack(count) if count == 0 => {
                animation.animation.tag = Some("attack".to_string());
                animation.animation.repeat = AnimationRepeat::Count(1);
                animation_state.current_frame = 11;
            }
            _ => {}
        }
    }
}

fn group(mut query: Query<(&Shadow, &Actor, &mut CollisionGroups)>) {
    for (shadow, actor, mut group) in query.iter_mut() {
        match shadow.state {
            ShadowState::Wait(count) if count == 0 => {
                *group = actor.actor_group.to_groups();
            }
            ShadowState::Hide(count) if count == 0 => {
                *group = *SHADOW_GROUPS;
            }
            ShadowState::Appear(count) if count == 0 => {
                *group = actor.actor_group.to_groups();
            }
            ShadowState::Attack(count) if count == 0 => {
                *group = actor.actor_group.to_groups();
            }
            _ => {}
        }
    }
}

pub struct ShadowPlugin;

impl Plugin for ShadowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (transition, animate, group)
                .chain()
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
