use super::floating::{InventoryItemFloating, InventoryItemFloatingContent};
use crate::{
    asset::GameAssets,
    constant::{MAX_SPELLS_IN_WAND, MAX_WANDS},
    controller::player::Player,
    entity::actor::Actor,
    inventory_item::InventoryItem,
    spell_props::spell_to_props,
    states::{GameMenuState, GameState},
};
use bevy::{
    prelude::*,
    ui::{Display, Style},
};
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct WandList;

#[derive(Component)]
pub struct WandSlot {
    wand_index: usize,
}

#[derive(Component)]
pub struct WandSprite {
    wand_index: usize,
}

#[derive(Component, Debug, Clone)]
struct WandSpellSprite {
    wand_index: usize,
    spell_index: usize,
}

pub fn spawn_wand_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            WandList,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            for wand_index in 0..MAX_WANDS {
                spawn_wand_and_spell_slot(parent, &assets, wand_index);
            }
        });
}

fn spawn_wand_and_spell_slot(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
) {
    parent
        .spawn((
            WandSlot { wand_index },
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(1.),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                border_color: Color::hsla(0.0, 0.0, 1.0, 0.0).into(),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            parent.spawn((
                WandSprite { wand_index },
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

            for spell_index in 0..MAX_SPELLS_IN_WAND {
                spawn_wand_spell_slot(&mut parent, &assets, wand_index, spell_index);
            }
        });
}

fn spawn_wand_spell_slot(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
    spell_index: usize,
) {
    parent.spawn((
        WandSpellSprite {
            wand_index,
            spell_index,
        },
        Interaction::default(),
        ImageBundle {
            style: Style {
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

fn update_wand_slot_visibility(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSlot, &mut BorderColor)>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut border) in sprite_query.iter_mut() {
            if wand_sprite.wand_index == actor.current_wand {
                *border = Color::hsla(0.0, 0.0, 1.0, 0.2).into();
            } else {
                *border = Color::hsla(0.0, 0.0, 1.0, 0.0).into();
            };
        }
    }
}

fn update_wand_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSprite, &mut AsepriteSlice)>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut aseprite) in sprite_query.iter_mut() {
            *aseprite = match actor.wands[wand_sprite.wand_index] {
                Some(_) => AsepriteSlice::new("wand"),
                None => AsepriteSlice::new("empty"),
            }
        }
    }
}

fn update_spell_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSpellSprite, &mut AsepriteSlice)>,
    floating_query: Query<&InventoryItemFloating>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (spell_sprite, mut aseprite) in sprite_query.iter_mut() {
            if let Some(wand) = &actor.wands[spell_sprite.wand_index] {
                if spell_sprite.spell_index < wand.slots.len() {
                    match wand.slots[spell_sprite.spell_index] {
                        Some(spell) => {
                            let floating = floating_query.single();
                            match floating.0 {
                                Some(InventoryItemFloatingContent::FromWand {
                                    wand_index,
                                    spell_index,
                                }) if wand_index == spell_sprite.wand_index
                                    && spell_index == spell_sprite.spell_index => {}
                                _ => {
                                    let props = spell_to_props(spell);
                                    *aseprite = AsepriteSlice::new(props.icon);
                                    continue;
                                }
                            }
                        }
                        None => {}
                    }
                }
            }

            *aseprite = AsepriteSlice::new("empty");
        }
    }
}

fn interaction_spell_sprite(
    mut interaction_query: Query<
        (&WandSpellSprite, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut player_query: Query<(&mut Player, &mut Actor)>,
    mut floating_query: Query<&mut InventoryItemFloating>,
    state: Res<State<GameMenuState>>,
) {
    for (slot, interaction, mut color) in &mut interaction_query {
        if *state.get() == GameMenuState::WandEditOpen {
            match *interaction {
                Interaction::Pressed => {
                    if let Ok((mut player, mut actor)) = player_query.get_single_mut() {
                        let mut floating = floating_query.single_mut();
                        match floating.0 {
                            Some(InventoryItemFloatingContent::FromInventory(index)) => {
                                match player.inventory[index] {
                                    Some(InventoryItem::Spell(selected_spell)) => {
                                        if let Some(ref mut wand) =
                                            &mut actor.wands[slot.wand_index]
                                        {
                                            match wand.slots[slot.spell_index] {
                                                None => {
                                                    player.inventory[index] = None;
                                                    wand.slots[slot.spell_index] =
                                                        Some(selected_spell);
                                                    *floating = InventoryItemFloating(None);
                                                }
                                                Some(existing) => {
                                                    player.inventory[index] =
                                                        Some(InventoryItem::Spell(existing));
                                                    wand.slots[slot.spell_index] =
                                                        Some(selected_spell);
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Some(InventoryItemFloatingContent::FromWand {
                                wand_index,
                                spell_index,
                            }) => {
                                // TODO
                                // 魔法の移動のとき、同じ配列に対して2つの可変参照を持つ可能性があるので配列を複製しているが、
                                // 複製すると、配列が異なる場合に正しく動かなくなってしまう
                                // 仕方ないので場合分けをしているが、もっと良い書き方があるかも
                                if wand_index == slot.wand_index {
                                    if spell_index == slot.spell_index {
                                        *floating = InventoryItemFloating(None);
                                    } else {
                                        match actor.wands[wand_index] {
                                            Some(ref mut wand_from) => {
                                                match wand_from.slots[slot.spell_index] {
                                                    None => {
                                                        let spell =
                                                            wand_from.slots[spell_index].clone();
                                                        wand_from.slots[slot.spell_index] = spell;
                                                        wand_from.slots[spell_index] = None;
                                                        actor.wands[wand_index] = Some(*wand_from);
                                                        *floating = InventoryItemFloating(None);
                                                    }
                                                    Some(existing) => {
                                                        let spell =
                                                            wand_from.slots[spell_index].clone();
                                                        wand_from.slots[slot.spell_index] = spell;
                                                        wand_from.slots[spell_index] =
                                                            Some(existing);
                                                        actor.wands[wand_index] = Some(*wand_from);
                                                    }
                                                }
                                            }
                                            _ => {
                                                warn!(
                                    "Invalid wand index, wand_index:{:?}, slot.wand_index:{:?}",
                                    wand_index, slot.wand_index
                                );
                                            }
                                        }
                                    }
                                } else {
                                    match (actor.wands[wand_index], actor.wands[slot.wand_index]) {
                                        (Some(ref mut wand_from), Some(ref mut wand_to)) => {
                                            match wand_to.slots[slot.spell_index] {
                                                None => {
                                                    let spell =
                                                        wand_from.slots[spell_index].clone();
                                                    wand_to.slots[slot.spell_index] = spell;
                                                    wand_from.slots[spell_index] = None;
                                                    actor.wands[wand_index] = Some(*wand_from);
                                                    actor.wands[slot.wand_index] = Some(*wand_to);
                                                    *floating = InventoryItemFloating(None);
                                                }
                                                Some(existing) => {
                                                    info!("existing spell: {:?}", existing);
                                                    let spell =
                                                        wand_from.slots[spell_index].clone();
                                                    wand_to.slots[slot.spell_index] = spell;
                                                    wand_from.slots[spell_index] = Some(existing);
                                                    actor.wands[wand_index] = Some(*wand_from);
                                                    actor.wands[slot.wand_index] = Some(*wand_to);
                                                }
                                            }
                                        }
                                        _ => {
                                            warn!(
                                    "Invalid wand index, wand_index:{:?}, slot.wand_index:{:?}",
                                    wand_index, slot.wand_index
                                );
                                        }
                                    }
                                }
                            }
                            None => {
                                if let Some(_) = actor.get_spell(slot.wand_index, slot.spell_index)
                                {
                                    *floating = InventoryItemFloating(Some(
                                        InventoryItemFloatingContent::FromWand {
                                            wand_index: slot.wand_index,
                                            spell_index: slot.spell_index,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }
                Interaction::Hovered => {
                    *color = Color::hsla(0.0, 0.0, 0.5, 0.2).into();
                }
                Interaction::None => {
                    *color = slot_color(slot.wand_index, slot.spell_index).into();
                }
            }
        } else {
            *color = slot_color(slot.wand_index, slot.spell_index).into();
        }
    }
}

fn slot_color(wand_index: usize, spell_index: usize) -> Color {
    return Color::hsla(
        0.0,
        0.0,
        0.4,
        if (wand_index + spell_index) % 2 == 0 {
            0.1
        } else {
            0.15
        },
    );
}

pub struct WandListPlugin;

impl Plugin for WandListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_wand_slot_visibility,
                update_wand_sprite,
                update_spell_sprite,
                interaction_spell_sprite,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
