use super::impact::SpawnImpact;
use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::falling::Falling;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::*;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct FallingRock;

#[derive(Default, Component, Reflect)]
struct FallenRock;

pub fn spawn_falling_rock(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            Name::new("falling rock"),
            StateScoped(GameState::InGame),
            FallingRock,
            EntityDepth::new(),
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "fallen_rock_shadow".to_string(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Falling::new(0.0, -0.1),
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "fallen_rock".to_string(),
                    ..default()
                },
                Transform::from_xyz(0.0, 800.0, 0.01),
            ));
        });
}

fn fall(
    mut commands: Commands,
    assets: Res<GameAssets>,
    child_query: Query<(&Parent, &Transform)>,
    parent_query: Query<(Entity, &Transform), (With<FallingRock>, Without<Falling>)>,
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
                    if tile != Tile::Wall && tile != Tile::Blank {
                        impact.send(SpawnImpact {
                            owner: None,
                            position,
                            radius: 16.0,
                            impulse: 30000.0,
                        });
                        spawn_fallen_rock(&mut commands, &assets, position);
                    }
                }
            }
        }
    }
}

fn spawn_fallen_rock(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            Name::new("fallen rock"),
            StateScoped(GameState::InGame),
            Life::new(200),
            FallenRock,
            EntityDepth::new(),
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
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
        ))
        .with_children(|parent| {
            parent.spawn((
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "fallen_rock".to_string(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.01),
            ));
        });
}

fn despown(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform), With<FallenRock>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(SE::Break, position));
        }
    }
}

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fall, despown)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<FallingRock>();
    }
}
