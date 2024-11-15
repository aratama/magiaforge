use super::{
    floating::{InventoryItemFloating, InventoryItemFloatingContent},
    spell_information::{SpellInformation, SpellInformationItem},
};
use crate::{
    asset::GameAssets,
    constant::{MAX_SPELLS_IN_WAND, MAX_WANDS},
    controller::player::Player,
    entity::actor::Actor,
    inventory_item::InventoryItem,
    spell_props::spell_to_props,
    states::{GameMenuState, GameState},
    wand_props::wand_to_props,
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
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            ..default()
        },
        BorderColor(Color::hsla(0.0, 0.0, 0.0, 0.0)),
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "empty".into(),
            ..default()
        },
    ));
}

fn update_wand_slot_visibility(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSlot, &mut BorderColor, &mut Visibility)>,
    floating_query: Query<&InventoryItemFloating>,
) {
    let floating = floating_query.single();
    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut border, mut visibility) in sprite_query.iter_mut() {
            *visibility = match floating.0 {
                Some(InventoryItemFloatingContent::Wand(index))
                    if index == wand_sprite.wand_index =>
                {
                    Visibility::Hidden
                }
                _ => Visibility::Inherited,
            };

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

fn update_spell_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(
        &WandSpellSprite,
        &mut AsepriteSlice,
        &mut Visibility,
        &mut BorderColor,
    )>,
    floating_query: Query<&InventoryItemFloating>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (spell_sprite, mut aseprite, mut visibility, mut border) in sprite_query.iter_mut() {
            if let Some(wand) = &actor.wands[spell_sprite.wand_index] {
                *border = BorderColor(Color::hsla(
                    0.0,
                    0.0,
                    1.0,
                    if wand.index == spell_sprite.spell_index
                        && actor.current_wand == spell_sprite.wand_index
                    {
                        0.2
                    } else {
                        0.0
                    },
                ));

                let props = wand_to_props(wand.wand_type);
                *visibility = if spell_sprite.spell_index < props.capacity {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };

                if spell_sprite.spell_index < wand.slots.len() {
                    match wand.slots[spell_sprite.spell_index] {
                        Some(spell) => {
                            let floating = floating_query.single();
                            match floating.0 {
                                Some(InventoryItemFloatingContent::WandSpell {
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
            } else {
                *visibility = Visibility::Hidden;
            }

            *aseprite = AsepriteSlice::new("empty");
        }
    }
}

fn interaction_spell_sprite(
    mut interaction_query: Query<(&WandSpellSprite, &Interaction), Changed<Interaction>>,
    mut player_query: Query<(&mut Player, &mut Actor)>,
    mut floating_query: Query<&mut InventoryItemFloating>,
    state: Res<State<GameMenuState>>,
    mut spell_info_query: Query<&mut SpellInformation>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok((mut player, mut actor)) = player_query.get_single_mut() {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        Some(InventoryItemFloatingContent::InventoryItem(index)) => {
                            match player.inventory[index] {
                                Some(InventoryItem::Spell(selected_spell)) => {
                                    if let Some(ref mut wand) = &mut actor.wands[slot.wand_index] {
                                        match wand.slots[slot.spell_index] {
                                            None => {
                                                player.inventory[index] = None;
                                                wand.slots[slot.spell_index] = Some(selected_spell);
                                                *floating = InventoryItemFloating(None);
                                            }
                                            Some(existing) => {
                                                player.inventory[index] =
                                                    Some(InventoryItem::Spell(existing));
                                                wand.slots[slot.spell_index] = Some(selected_spell);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Some(InventoryItemFloatingContent::WandSpell {
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
                                                    actor.wands[wand_index] =
                                                        Some(wand_from.clone());
                                                    *floating = InventoryItemFloating(None);
                                                }
                                                Some(existing) => {
                                                    let spell =
                                                        wand_from.slots[spell_index].clone();
                                                    wand_from.slots[slot.spell_index] = spell;
                                                    wand_from.slots[spell_index] = Some(existing);
                                                    actor.wands[wand_index] =
                                                        Some(wand_from.clone());
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
                                match (
                                    actor.wands[wand_index].clone(),
                                    actor.wands[slot.wand_index].clone(),
                                ) {
                                    (Some(ref mut wand_from), Some(ref mut wand_to)) => {
                                        match wand_to.slots[slot.spell_index] {
                                            None => {
                                                let spell = wand_from.slots[spell_index].clone();
                                                wand_to.slots[slot.spell_index] = spell;
                                                wand_from.slots[spell_index] = None;
                                                actor.wands[wand_index] = Some(wand_from.clone());
                                                actor.wands[slot.wand_index] =
                                                    Some(wand_to.clone());
                                                *floating = InventoryItemFloating(None);
                                            }
                                            Some(existing) => {
                                                info!("existing spell: {:?}", existing);
                                                let spell = wand_from.slots[spell_index].clone();
                                                wand_to.slots[slot.spell_index] = spell;
                                                wand_from.slots[spell_index] = Some(existing);
                                                actor.wands[wand_index] = Some(wand_from.clone());
                                                actor.wands[slot.wand_index] =
                                                    Some(wand_to.clone());
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
                        Some(InventoryItemFloatingContent::Wand(_)) => {}
                        None => {
                            if let Some(_) = actor.get_spell(slot.wand_index, slot.spell_index) {
                                *floating = InventoryItemFloating(Some(
                                    InventoryItemFloatingContent::WandSpell {
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
                if let Ok((_, actor)) = player_query.get_single() {
                    match actor.wands[slot.wand_index] {
                        Some(ref wand) => {
                            if let Some(spell) = wand.slots[slot.spell_index] {
                                let mut spell_info = spell_info_query.single_mut();
                                *spell_info =
                                    SpellInformation(Some(SpellInformationItem::Spell(spell)));
                            }
                        }
                        None => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn update_spell_sprite_background(
    mut interaction_query: Query<
        (&WandSpellSprite, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    state: Res<State<GameMenuState>>,
) {
    for (slot, interaction, mut color) in &mut interaction_query {
        if *state.get() == GameMenuState::WandEditOpen {
            match *interaction {
                Interaction::Pressed => {}
                Interaction::Hovered => {
                    *color = Color::hsla(0.0, 0.0, 0.5, 0.3).into();
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

fn interact_wand_sprite(
    mut interaction_query: Query<(&WandSprite, &Interaction), Changed<Interaction>>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut floating_query: Query<&mut InventoryItemFloating>,
    state: Res<State<GameMenuState>>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut actor) = player_query.get_single_mut() {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        Some(InventoryItemFloatingContent::InventoryItem(_)) => {}
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
                update_spell_sprite_background,
                interact_wand_sprite,
                update_wand_information,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
