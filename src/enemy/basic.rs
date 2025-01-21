use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::actor::ActorSpriteGroup;
use crate::actor::ActorTypes;
use crate::asset::GameAssets;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::life::Life;
use crate::component::vertical::Vertical;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGold;
use crate::controller::servant::Servant;
use crate::entity::bullet::HomingTarget;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct BasicEnemy;

#[derive(Component, Debug)]
pub struct BasicEnemySprite;

pub fn spawn_basic_enemy(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
    actor_type: ActorTypes,
    name: &str,
    spell: Option<SpellType>,
    golds: u32,
    actor_group: ActorGroup,
    master: Option<Entity>,
    max_life: u32,
    radius: f32,
    auto_levitation: bool,
) -> Entity {
    let mut builder = commands.spawn((
        BasicEnemy,
        Name::new(name.to_string()),
        StateScoped(GameState::InGame),
        DespawnWithGold { golds },
        Actor {
            actor_type,
            radius,
            actor_group,
            golds,
            wands: Wand::single(spell),
            auto_levitation,
            ..default()
        },
        Life::new(max_life),
        HomingTarget,
        Transform::from_translation(position.extend(0.0)),
        Vertical::default(),
        Visibility::default(),
        Counter::default(),
        (
            RigidBody::Dynamic,
            Collider::ball(radius),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveEvents::COLLISION_EVENTS,
            actor_group.to_groups(0.0, 0),
        ),
    ));

    builder.with_children(|mut parent| {
        parent.spawn((
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "chicken_shadow".into(),
            },
            Transform::from_xyz(0.0, 0.0, SHADOW_LAYER_Z),
        ));

        parent.spawn(ActorSpriteGroup).with_child((
            BasicEnemySprite,
            CounterAnimated,
            AseSpriteAnimation {
                aseprite,
                animation: Animation::default().with_tag("idle"),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        spawn_life_bar(&mut parent, &life_bar_locals);
    });

    if let Some(owner) = master {
        builder.insert(Servant { master: owner });
    }

    builder.id()
}

fn animate(
    query: Query<&Actor, With<BasicEnemy>>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<(&Parent, &mut AseSpriteAnimation), With<BasicEnemySprite>>,
) {
    for (parent, mut animation) in sprite_query.iter_mut() {
        if let Ok(group) = group_query.get(parent.get()) {
            if let Ok(actor) = query.get(group.get()) {
                animation.animation.repeat = AnimationRepeat::Loop;
                animation.animation.tag = Some(
                    if 0 < actor.frozen {
                        "frozen"
                    } else if 0 < actor.staggered {
                        "staggered"
                    } else {
                        "idle"
                    }
                    .to_string(),
                );
            }
        }
    }
}

fn flip(
    actor_query: Query<&Actor, With<BasicEnemy>>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<
        (&Parent, &mut Sprite),
        (With<BasicEnemySprite>, Without<ActorSpriteGroup>),
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
            (animate, flip).in_set(FixedUpdateGameActiveSet),
        );
    }
}
