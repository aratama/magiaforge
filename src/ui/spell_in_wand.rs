use super::{
    floating::{Floating, FloatingContent},
    item_information::{SpellInformation, SpellInformationItem},
    wand_list::slot_color,
};
use crate::{
    asset::GameAssets,
    controller::player::Player,
    entity::actor::Actor,
    inventory_item::InventoryItem,
    spell_props::spell_to_props,
    states::{GameMenuState, GameState},
    wand_props::wand_to_props,
};
use bevy::{prelude::*, ui::Node};
use bevy_aseprite_ultra::prelude::*;

#[derive(Component, Debug, Clone)]
struct WandSpellSprite {
    wand_index: usize,
    spell_index: usize,
}

pub fn spawn_wand_spell_slot(
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
        BackgroundColor(slot_color(wand_index, spell_index)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(64.0 + 32. * (spell_index as f32)),
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

fn update_spell_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSpellSprite, &mut AseUiSlice, &mut Visibility)>,
    floating_query: Query<&Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (spell_sprite, mut aseprite, mut visibility) in sprite_query.iter_mut() {
            if let Some(wand) = &actor.wands[spell_sprite.wand_index] {
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
                                Some(FloatingContent::WandSpell(wand_index, spell_index))
                                    if wand_index == spell_sprite.wand_index
                                        && spell_index == spell_sprite.spell_index => {}
                                _ => {
                                    let props = spell_to_props(spell);
                                    aseprite.name = props.icon.to_string();
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

            aseprite.name = "empty".into();
        }
    }
}

fn interaction_spell_sprite(
    mut interaction_query: Query<(&WandSpellSprite, &Interaction), Changed<Interaction>>,
    mut player_query: Query<(&mut Player, &mut Actor)>,
    mut floating_query: Query<&mut Floating>,
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
                        Some(FloatingContent::Inventory(index)) => {
                            match player.inventory.get(index) {
                                Some(InventoryItem::Spell(selected_spell)) => {
                                    if let Some(ref mut wand) = &mut actor.wands[slot.wand_index] {
                                        match wand.slots[slot.spell_index] {
                                            None => {
                                                player.inventory.set(index, None);
                                                wand.slots[slot.spell_index] = Some(selected_spell);
                                                *floating = Floating(None);
                                            }
                                            Some(existing) => {
                                                player.inventory.set(
                                                    index,
                                                    Some(InventoryItem::Spell(existing)),
                                                );
                                                wand.slots[slot.spell_index] = Some(selected_spell);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Some(FloatingContent::WandSpell(wand_index, spell_index)) => {
                            // TODO
                            // 魔法の移動のとき、同じ配列に対して2つの可変参照を持つ可能性があるので配列を複製しているが、
                            // 複製すると、配列が異なる場合に正しく動かなくなってしまう
                            // 仕方ないので場合分けをしているが、もっと良い書き方があるかも
                            if wand_index == slot.wand_index {
                                if spell_index == slot.spell_index {
                                    *floating = Floating(None);
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
                                                    *floating = Floating(None);
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
                                                *floating = Floating(None);
                                            }
                                            Some(existing) => {
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
                        Some(FloatingContent::Wand(_)) => {}
                        Some(FloatingContent::Equipment(_)) => {}
                        None => {
                            if let Some(_) = actor.get_spell(slot.wand_index, slot.spell_index) {
                                *floating = Floating(Some(FloatingContent::WandSpell(
                                    slot.wand_index,
                                    slot.spell_index,
                                )));
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
                                    SpellInformation(Some(SpellInformationItem::InventoryItem(
                                        InventoryItem::Spell(spell),
                                    )));
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

pub struct SpellInWandPlugin;

impl Plugin for SpellInWandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_spell_sprite, interaction_spell_sprite).run_if(in_state(GameState::InGame)),
        );
    }
}
