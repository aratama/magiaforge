use crate::entity::EntityDepth;
use crate::spell::SpellType;
use crate::spell_props::spell_to_props;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Default, Component)]
pub struct SpellEntity;

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
            SpellEntity,
            EntityDepth,
            AsepriteSliceBundle {
                aseprite: assets.atlas.clone(),
                slice: "spell_frame".into(),
                transform: Transform::from_translation(Vec3::new(
                    tx + (random::<f32>() - 0.5) * 16.0,
                    ty + (random::<f32>() - 0.5) * 16.0,
                    0.0,
                )),
                ..default()
            },
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Fixed,
            Sensor,
            Collider::cuboid(16.0, 16.0),
            CollisionGroups::new(ENTITY_GROUP, ENTITY_GROUP | WALL_GROUP),
        ))
        .with_children(|parent| {
            parent.spawn((
                StateScoped(GameState::InGame),
                AsepriteSliceBundle {
                    aseprite: assets.atlas.clone(),
                    slice: props.icon.into(),
                    ..default()
                },
            ));
        });
}

pub struct SpellEntityPlugin;

impl Plugin for SpellEntityPlugin {
    fn build(&self, _app: &mut App) {}
}
