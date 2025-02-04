use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::actor::LifeBeingSprite;
use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::point_light::WithPointLight;
use crate::entity::fire::spawn_fire;
use crate::level::tile::Tile;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct Fireball;

#[derive(Default, Component, Reflect)]
struct FireballSprite;

pub fn spawn_fireball(
    commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    velocity: Vec2,
    actor_group: ActorGroup,
) {
    commands
        .spawn((
            Name::new("fireball"),
            StateScoped(GameState::InGame),
            Fireball,
            EntityDepth::new(),
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            WithPointLight {
                radius: 64.0,
                intensity: 1.0,
                falloff: 10.0,
                color: Color::hsl(42.0, 1.0, 0.71),
                animation_offset: rand::random::<u32>() % 1000,
                speed: 0.43,
                amplitude: 0.1,
            },
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                },
                Collider::ball(6.0),
                actor_group.to_bullet_group(),
                Velocity::linear(velocity),
                ExternalImpulse::default(),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                FireballSprite,
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: registry.assets.fireball.clone(),
                    animation: "default".into(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.01),
            ));
        });
}

fn spawn_fire_on_landed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    parent_query: Query<(Entity, &Actor, &Transform), With<Fireball>>,
    interlevel: Res<GameWorld>,
) {
    for (entity, vertical, transform) in parent_query.iter() {
        if vertical.just_landed {
            commands.entity(entity).despawn_recursive();

            let position = transform.translation.truncate();
            let tile = interlevel.get_tile_by_coords(position);
            if tile != Tile::new("Wall")
                && tile != Tile::new("Blank")
                && tile != Tile::new("Water")
                && tile != Tile::new("PermanentWall")
            {
                spawn_fire(&mut commands, &assets, position, None);
            }
        }
    }
}

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (spawn_fire_on_landed).in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Fireball>();
    }
}
