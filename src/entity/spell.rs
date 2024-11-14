use crate::entity::EntityDepth;
use crate::spell::SpellType;
use crate::spell_props::spell_to_props;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Component)]
pub struct SpellEntity {
    pub spell: SpellType,
    pub interaction_marker: bool,
}

#[derive(Component)]
struct SpellSprites;

#[derive(Component)]
struct InteractionMarker;

pub fn spawn_spell_entity(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    x: f32,
    y: f32,
    spell: SpellType,
) {
    let tx = x;
    let ty = y;
    let props = spell_to_props(spell);
    commands
        .spawn((
            Name::new(format!("spell {}", props.name)),
            StateScoped(GameState::InGame),
            SpellEntity {
                spell,
                interaction_marker: false,
            },
            EntityDepth,
            InheritedVisibility::default(),
            Transform::from_translation(Vec3::new(
                tx + (random::<f32>() - 0.5) * 16.0,
                ty + (random::<f32>() - 0.5) * 16.0,
                0.0,
            )),
            GlobalTransform::default(),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Fixed,
            Sensor,
            Collider::cuboid(8.0, 8.0),
            CollisionGroups::new(
                ENTITY_GROUP,
                ENTITY_GROUP | WALL_GROUP | PLAYER_INTERACTION_SENSOR_GROUP,
            ),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpellSprites,
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    GlobalTransform::default(),
                    InheritedVisibility::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.atlas.clone(),
                            slice: "spell_frame".into(),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.atlas.clone(),
                            slice: props.icon.into(),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        InteractionMarker,
                        StateScoped(GameState::InGame),
                        AsepriteSliceBundle {
                            aseprite: assets.atlas.clone(),
                            transform: Transform::from_xyz(0.0, 14.0, 0.0),
                            slice: "interactive".into(),
                            // visibilityはinteraction_sensor側で制御しています
                            visibility: Visibility::Hidden,
                            sprite: Sprite {
                                color: Color::hsla(0.0, 0.0, 1.0, 0.2),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn swing(mut query: Query<&mut Transform, With<SpellSprites>>, frame_count: Res<FrameCount>) {
    for mut transform in query.iter_mut() {
        transform.translation.y = (frame_count.0 as f32 * 0.05).sin() * 2.0;
    }
}

fn update_interactive_marker_visibility(
    spell: Query<&SpellEntity>,
    sprites: Query<&Parent, With<SpellSprites>>,
    mut marker_query: Query<(&Parent, &mut Visibility), With<InteractionMarker>>,
) {
    for (parent, mut visibility) in marker_query.iter_mut() {
        if let Ok(parent) = sprites.get(parent.get()) {
            if let Ok(spell) = spell.get(parent.get()) {
                *visibility = if spell.interaction_marker {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (swing, update_interactive_marker_visibility).run_if(in_state(GameState::InGame)),
        );
    }
}
