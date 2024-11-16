use bevy::{
    prelude::*,
    ui::{Display, Style},
};
use bevy_aseprite_ultra::prelude::*;

use crate::{asset::GameAssets, constant::MAX_SPELLS_IN_WAND};

use super::wand_list::slot_color;

#[derive(Component)]
pub struct EquipmentContainer;

#[derive(Component, Debug, Clone)]
struct EquipmentSprite {
    index: usize,
}

pub fn spawn_equipment_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            EquipmentContainer,
            NodeBundle {
                style: Style {
                    width: Val::Px(64. + 32. * MAX_SPELLS_IN_WAND as f32),
                    height: Val::Px(32.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                border_color: Color::hsla(0.0, 0.0, 1.0, 0.0).into(),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            for index in 0..MAX_SPELLS_IN_WAND {
                spawn_equipment_slot(&mut parent, &assets, index);
            }
        });
}

fn spawn_equipment_slot(parent: &mut ChildBuilder, assets: &Res<GameAssets>, index: usize) {
    parent.spawn((
        EquipmentSprite { index },
        Interaction::default(),
        ImageBundle {
            background_color: slot_color(5, index).into(),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(64.0 + 32. * (index as f32)),
                width: Val::Px(32.),
                height: Val::Px(32.),
                ..default()
            },
            ..default()
        },
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "empty".into(),
            ..default()
        },
    ));
}

pub struct EquipmentListPlugin;

impl Plugin for EquipmentListPlugin {
    fn build(&self, _app: &mut App) {}
}
