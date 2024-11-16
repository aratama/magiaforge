use super::{
    floating::{InventoryItemFloating, InventoryItemFloatingContent},
    item_information::{SpellInformation, SpellInformationItem},
};
use crate::{
    asset::GameAssets,
    constant::MAX_SPELLS_IN_WAND,
    controller::player::Player,
    entity::actor::Actor,
    inventory_item::{spawn_inventory_item, InventoryItem},
    states::{GameMenuState, GameState},
    wand::Wand,
    wand_props::wand_to_props,
};
use bevy::{prelude::*, ui::Style};
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct WandSprite {
    wand_index: usize,
}

pub fn spawn_wand_sprite_in_list(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
) {
    parent.spawn((
        WandSprite { wand_index },
        Interaction::default(),
        ImageBundle {
            style: Style {
                width: Val::Px(64.),
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

fn update_wand_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSprite, &mut AsepriteSlice)>,
    floating_query: Query<&InventoryItemFloating>,
) {
    let floating = floating_query.single();

    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut aseprite) in sprite_query.iter_mut() {
            *aseprite = match floating {
                InventoryItemFloating(Some(InventoryItemFloatingContent::Wand(wand_index)))
                    if *wand_index == wand_sprite.wand_index =>
                {
                    AsepriteSlice::new("empty")
                }
                _ => match &actor.wands[wand_sprite.wand_index] {
                    Some(wand) => {
                        let props = wand_to_props(wand.wand_type);
                        AsepriteSlice::new(props.icon)
                    }
                    None => AsepriteSlice::new("empty"),
                },
            }
        }
    }
}

fn interact_wand_sprite(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut interaction_query: Query<(&WandSprite, &Interaction), Changed<Interaction>>,
    mut player_query: Query<(&mut Player, &mut Actor, &Transform)>,
    mut floating_query: Query<&mut InventoryItemFloating>,
    state: Res<State<GameMenuState>>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok((mut player, mut actor, player_position)) = player_query.get_single_mut()
                {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        Some(InventoryItemFloatingContent::InventoryItem(index)) => {
                            match player.inventory[index] {
                                Some(InventoryItem::Wand(wand)) => {
                                    let current = actor.wands[slot.wand_index].clone();

                                    let not_inserted = match current {
                                        Some(wand) => player.insert_wand_to_inventory(&wand),
                                        None => Vec::new(),
                                    };

                                    for item in not_inserted {
                                        spawn_inventory_item(
                                            &mut commands,
                                            &assets,
                                            player_position.translation.truncate(),
                                            item,
                                        );
                                    }

                                    actor.wands[slot.wand_index] = Some(Wand {
                                        wand_type: wand,
                                        slots: [None; MAX_SPELLS_IN_WAND],
                                        index: 0,
                                    });
                                    *floating = InventoryItemFloating(None);
                                    player.inventory[index] = None;
                                }
                                _ => {}
                            }
                        }
                        Some(InventoryItemFloatingContent::WandSpell { .. }) => {}
                        Some(InventoryItemFloatingContent::Wand(wand_index)) => {
                            if wand_index == slot.wand_index {
                                *floating = InventoryItemFloating(None);
                            } else {
                                let wand = actor.wands[slot.wand_index].clone();
                                actor.wands[slot.wand_index] = actor.wands[wand_index].clone();
                                actor.wands[wand_index] = wand;
                                *floating = InventoryItemFloating(None);
                            }
                        }
                        None => {
                            if let Some(_) = actor.wands[slot.wand_index] {
                                *floating = InventoryItemFloating(Some(
                                    InventoryItemFloatingContent::Wand(slot.wand_index),
                                ));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn update_wand_information(
    mut interaction_query: Query<(&WandSprite, &Interaction), Changed<Interaction>>,
    mut player_query: Query<&Actor>,
    mut spell_information_query: Query<&mut SpellInformation>,
    state: Res<State<GameMenuState>>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if let Ok(actor) = player_query.get_single_mut() {
                    if let Some(ref wand) = actor.wands[slot.wand_index] {
                        let mut info = spell_information_query.single_mut();
                        *info = SpellInformation(Some(SpellInformationItem::Wand(wand.wand_type)));
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct WandSpritePlugin;

impl Plugin for WandSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_wand_sprite,
                interact_wand_sprite,
                update_wand_information,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
