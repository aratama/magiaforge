use crate::{
    asset::GameAssets, constant::MAX_SPELLS_IN_WAND, controller::player::Player,
    equipment::equipment_to_props, inventory_item::InventoryItem, states::GameState,
};
use crate::{
    states::GameMenuState,
    ui::{
        floating::{Floating, FloatingContent},
        wand_list::slot_color,
    },
};
use bevy::{prelude::*, ui::Display};
use bevy_aseprite_ultra::prelude::*;

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
            BorderColor(Color::hsla(0.0, 0.0, 1.0, 0.0)),
            Node {
                width: Val::Px(64. + 32. * MAX_SPELLS_IN_WAND as f32),
                height: Val::Px(32.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(1.)),
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
        BackgroundColor(slot_color(5, index)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(64.0 + 32. * (index as f32)),
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "empty".into(),
        },
    ));
}

fn update_equipment_sprite(
    mut sprite_query: Query<(&EquipmentSprite, &mut AseUiSlice)>,
    player_query: Query<&Player>,
    floating_query: Query<&mut Floating>,
) {
    if let Ok(player) = player_query.get_single() {
        for (sprite, mut slice) in sprite_query.iter_mut() {
            let float = floating_query.single();
            let picked = match float.0 {
                Some(FloatingContent::Equipment(index)) => index == sprite.index,
                _ => false,
            };
            if picked {
                slice.name = "empty".into();
            } else if let Some(equipment) = player.equipments[sprite.index] {
                let props = equipment_to_props(equipment);
                slice.name = props.icon.into();
            } else {
                slice.name = "empty".into();
            }
        }
    }
}

fn interact(
    sprite_query: Query<(&EquipmentSprite, &Interaction), Changed<Interaction>>,
    mut player_query: Query<&mut Player>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
) {
    if *state == GameMenuState::WandEditOpen {
        if let Ok(mut player) = player_query.get_single_mut() {
            for (sprite, interaction) in sprite_query.iter() {
                let mut floating = floating_query.single_mut();
                match interaction {
                    Interaction::Pressed => match floating.0 {
                        None => {
                            *floating = Floating(Some(FloatingContent::Equipment(sprite.index)));
                        }
                        Some(FloatingContent::Inventory(index)) => {
                            match player.inventory.get(index) {
                                Some(InventoryItem::Equipment(equipment)) => {
                                    player.equipments[sprite.index] = Some(equipment);
                                    player.inventory.set(index, None);
                                    *floating = Floating(None);
                                }
                                _ => {}
                            }
                        }
                        Some(FloatingContent::Wand(_)) => {}
                        Some(FloatingContent::WandSpell { .. }) => {}
                        Some(FloatingContent::Equipment(index)) => {
                            player.equipments.swap(index, sprite.index);
                            *floating = Floating(None);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

pub struct EquipmentListPlugin;

impl Plugin for EquipmentListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_equipment_sprite, interact).run_if(in_state(GameState::InGame)),
        );
    }
}
