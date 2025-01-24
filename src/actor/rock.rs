use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::LifeBeingSprite;
use crate::collision::*;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::falling::Falling;
use crate::component::vertical::Vertical;
use crate::entity::impact::SpawnImpact;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct FallingRock;

#[derive(Default, Component, Reflect)]
struct FallenRock;

pub fn spawn_falling_rock(commands: &mut Commands, registry: &Registry, position: Vec2) {
    commands
        .spawn((
            Name::new("vertical rock"),
            StateScoped(GameState::InGame),
            FallingRock,
            EntityDepth::new(),
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "fallen_rock_shadow".to_string(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Vertical::new(0.0, -0.1),
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: "fallen_rock".to_string(),
                    ..default()
                },
                Transform::from_xyz(0.0, 800.0, 0.01),
            ));
        });
}

fn fall(
    mut commands: Commands,
    registry: Registry,
    child_query: Query<(&Parent, &Transform)>,
    parent_query: Query<(Entity, &Transform), (With<FallingRock>, Without<Vertical>)>,
    interlevel: Res<LevelSetup>,
    mut impact: EventWriter<SpawnImpact>,
) {
    for (parent, child_transform) in child_query.iter() {
        if let Ok((entity, parent_transform)) = parent_query.get(parent.get()) {
            if child_transform.translation.y <= 0.0 {
                let position = parent_transform.translation.truncate();
                commands.entity(entity).despawn_recursive();

                if let Some(ref level) = interlevel.chunk {
                    let tile = level.get_tile_by_coords(position);
                    if tile != Tile::Wall && tile != Tile::Blank && tile != Tile::PermanentWall {
                        impact.send(SpawnImpact {
                            owner: None,
                            position,
                            radius: 16.0,
                            impulse: 30000.0,
                        });
                        spawn_fallen_rock(&mut commands, &registry, position, default_rock());
                    }
                }
            }
        }
    }
}

pub fn default_rock() -> Actor {
    Actor {
        extra: ActorExtra::Rock,
        life: 200,
        max_life: 200,
        ..default()
    }
}

pub fn spawn_fallen_rock(
    commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    commands
        .spawn((
            Name::new("fallen rock"),
            StateScoped(GameState::InGame),
            actor,
            FallenRock,
            EntityDepth::new(),
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            Falling,
            (
                RigidBody::Dynamic,
                Damping {
                    linear_damping: 60.0,
                    angular_damping: 0.0,
                },
                LockedAxes::ROTATION_LOCKED,
                Collider::ball(16.0),
                ColliderMassProperties::Density(10.0),
                *ENTITY_GROUPS,
                ExternalImpulse::default(),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: "fallen_rock".to_string(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.01),
            ));
        })
        .id()
}

fn despawn(query: Query<(&Actor, &Transform), With<FallenRock>>, mut writer: EventWriter<SEEvent>) {
    for (breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();
            writer.send(SEEvent::pos(SE::Break, position));
        }
    }
}

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fall, despawn).in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<FallingRock>();
    }
}
