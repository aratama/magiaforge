use crate::ui::floating::{InventoryItemFloating, InventoryItemFloatingContent};
use crate::{
    asset::GameAssets, constant::MAX_ITEMS_IN_INVENTORY, controller::player::Player,
    entity::actor::Actor, inventory_item::InventoryItem, spell_props::spell_to_props,
    states::GameState, wand::wand_to_props,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AsepriteSlice, AsepriteSliceUiBundle};

use super::spell_information::SpellInformation;

#[derive(Component)]
struct InventoryItemSlot(usize);

pub fn spawn_inventory(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    // Make the height of the node fill its parent
                    height: Val::Percent(100.0),
                    // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                    // As the height is set explicitly, this means the width will adjust to match the height
                    aspect_ratio: Some(1.0),
                    // Use grid layout for this node
                    display: Display::Grid,
                    // Add 24px of padding around the grid
                    padding: UiRect::all(Val::Px(2.0)),
                    // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized columns
                    grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
                    // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized rows
                    grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
                    // Set a 12px gap/gutter between rows and columns
                    // row_gap: Val::Px(2.0),
                    // column_gap: Val::Px(2.0),
                    ..default()
                },
                // background_color: BackgroundColor(Color::hsla(0.0, 0.0, 0.5, 0.2)),
                ..default()
            },
        ))
        .with_children(|builder| {
            for i in 0..MAX_ITEMS_IN_INVENTORY {
                builder.spawn((
                    InventoryItemSlot(i),
                    StateScoped(GameState::MainMenu),
                    Interaction::default(),
                    ImageBundle {
                        style: Style {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        // background_color: BackgroundColor(Color::hsla(
                        //     0.0,
                        //     0.0,
                        //     0.5,
                        //     if i % 2 == 0 { 0.8 } else { 0.8 },
                        // )),
                        ..default()
                    },
                    AsepriteSliceUiBundle {
                        aseprite: assets.atlas.clone(),
                        slice: "empty".into(),
                        ..default()
                    },
                ));
            }
        });
}

fn update_inventory(
    query: Query<&Player>,
    mut slot_query: Query<(&InventoryItemSlot, &mut AsepriteSlice)>,
    floating_query: Query<&InventoryItemFloating>,
) {
    if let Ok(player) = query.get_single() {
        let floating = floating_query.single();
        for (slot, mut aseprite) in slot_query.iter_mut() {
            match floating.0 {
                Some(InventoryItemFloatingContent::FromInventory(index)) => {
                    if index == slot.0 {
                        *aseprite = "empty".into();
                        continue;
                    }
                }
                _ => {}
            }

            let item = &player.inventory[slot.0];
            let slice: &'static str = match item {
                None => "empty",
                Some(InventoryItem::Wand(wand)) => {
                    let props = wand_to_props(*wand);
                    props.slice
                }
                Some(InventoryItem::Spell(spell)) => {
                    let props = spell_to_props(*spell);
                    props.icon
                }
            };
            *aseprite = slice.into();
        }
    }
}

fn interaction(
    mut interaction_query: Query<
        (&InventoryItemSlot, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut floating_query: Query<&mut InventoryItemFloating>,
    mut player_query: Query<(&mut Player, &mut Actor)>,

    mut spell_info_query: Query<&mut SpellInformation>,
) {
    for (slot, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok((mut player, mut actor)) = player_query.get_single_mut() {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        None => {
                            *floating = InventoryItemFloating(Some(
                                InventoryItemFloatingContent::FromInventory(slot.0),
                            ));
                        }
                        Some(InventoryItemFloatingContent::FromInventory(index)) => {
                            match player.inventory[slot.0] {
                                None => {
                                    *floating = InventoryItemFloating(None);
                                    player.inventory[slot.0] = player.inventory[index];
                                    player.inventory[index] = None;
                                }
                                Some(_) => {
                                    let spell = player.inventory[slot.0];
                                    player.inventory[slot.0] = player.inventory[index];
                                    player.inventory[index] = spell;
                                }
                            }
                        }
                        Some(InventoryItemFloatingContent::FromWand {
                            wand_index,
                            spell_index,
                        }) => match actor.wands[wand_index] {
                            None => {
                                *floating = InventoryItemFloating(None);
                            }
                            Some(mut wand) => {
                                let spell = wand.slots[spell_index];
                                player.inventory[slot.0] =
                                    spell.and_then(|s| Some(InventoryItem::Spell(s)));
                                wand.slots[spell_index] = None;
                                actor.wands[wand_index] = Some(wand);
                                *floating = InventoryItemFloating(None);
                            }
                        },
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::hsla(0.0, 0.0, 0.5, 0.95).into();
                let mut spell_info = spell_info_query.single_mut();
                if let Ok((player, _)) = player_query.get_single() {
                    let item = player.inventory[slot.0];
                    *spell_info = SpellInformation(match item {
                        None => None,
                        Some(InventoryItem::Wand(_)) => None,
                        Some(InventoryItem::Spell(spell)) => Some(spell),
                    });
                } else {
                    *spell_info = SpellInformation(None);
                }
            }
            Interaction::None => {
                *color = get_background(slot.0);
            }
        }
    }
}

fn get_background(index: usize) -> BackgroundColor {
    let x = index % 8;
    let y = index / 8;
    Color::hsla(0.0, 0.0, if (x + y) % 2 == 0 { 0.2 } else { 0.24 }, 0.95).into()
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_inventory, interaction).run_if(in_state(GameState::InGame)),
        );
    }
}
