use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::life::Life;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGold;
use crate::controller::servant::Servant;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorProps;
use crate::entity::actor::ActorSpriteGroup;
use crate::entity::bullet::HomingTarget;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::Inventory;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

#[derive(Component, Debug)]
pub struct BasicEnemy;

#[derive(Component, Debug)]
pub struct BasicEnemySprite;

pub fn spawn_basic_enemy<T: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
    marker: T,
    name: &str,
    spell: Option<SpellType>,
    move_force: f32,
    golds: u32,
    actor_group: ActorGroup,
    master: Option<Entity>,
    max_life: i32,
    radius: f32,
) -> Entity {
    let mut builder = commands.spawn((
        BasicEnemy,
        Name::new(name.to_string()),
        StateScoped(GameState::InGame),
        DespawnWithGold { golds },
        marker,
        Actor::new(ActorProps {
            uuid: Uuid::new_v4(),
            angle: 0.0,
            point_light_radius: 0.0,
            radius,
            move_force: move_force,
            current_wand: 0,
            actor_group,
            golds,
            inventory: Inventory::new(),
            wands: Wand::single(spell),
            fire_resistance: false,
        }),
        Life::new(max_life),
        HomingTarget,
        Transform::from_translation(position.extend(0.0)),
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
    mut sprite_query: Query<(&Parent, &mut AseSpriteAnimation), With<BasicEnemySprite>>,
) {
    for (parent, mut animation) in sprite_query.iter_mut() {
        if let Ok(slime) = query.get(parent.get()) {
            if 0 < slime.frozen {
                animation.animation.tag = Some("frozen".to_string());
                animation.animation.repeat = AnimationRepeat::Loop;
            } else {
                animation.animation.tag = Some("idle".to_string());
                animation.animation.repeat = AnimationRepeat::Loop;
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
