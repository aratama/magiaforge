use super::counter::CounterAnimated;
use super::falling::Falling;
use super::fire::spawn_fire;
use super::point_light::WithPointLight;
use crate::asset::GameAssets;
use crate::constant::ENTITY_GROUP;
use crate::constant::RABBIT_GROUP;
use crate::constant::WALL_GROUP;
use crate::entity::life::LifeBeingSprite;
use crate::entity::EntityDepth;
use crate::level::tile::Tile;
use crate::page::in_game::Interlevel;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct Fireball;

pub fn spawn_fireball(commands: &mut Commands, assets: &Res<GameAssets>, from: Vec2, to: Vec2) {
    commands
        .spawn((
            Name::new("fireball"),
            StateScoped(GameState::InGame),
            Fireball,
            EntityDepth,
            Visibility::default(),
            Transform::from_translation(from.extend(0.0)),
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
                CollisionGroups::new(ENTITY_GROUP, ENTITY_GROUP | WALL_GROUP | RABBIT_GROUP),
                Velocity::linear((to - from).normalize_or_zero() * 200.0),
                ExternalImpulse::default(),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Falling::new(2.0, -0.1),
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.fireball.clone(),
                    animation: "default".into(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.01),
            ));
        });
}

fn fall(
    mut commands: Commands,
    assets: Res<GameAssets>,
    child_query: Query<(&Parent, &Falling)>,
    parent_query: Query<(Entity, &Transform), With<Fireball>>,
    interlevel: Res<Interlevel>,
) {
    for (parent, falling) in child_query.iter() {
        if falling.just_landed {
            if let Ok((entity, parent_transform)) = parent_query.get(parent.get()) {
                commands.entity(entity).despawn_recursive();
                if let Some(ref level) = interlevel.chunk {
                    let position = parent_transform.translation.truncate();
                    let tile = level.get_tile_by_coords(position);
                    if tile != Tile::Wall && tile != Tile::Blank {
                        spawn_fire(&mut commands, &assets, position);
                    }
                }
            }
        }
    }
}

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fall)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Fireball>();
    }
}
