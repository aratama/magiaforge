use super::{
    floating::{Floating, FloatingContent},
    item_panel::{spawn_item_panel, ItemPanel},
};
use crate::{
    asset::GameAssets,
    controller::player::Player,
    entity::actor::Actor,
    inventory::InventoryItem,
    inventory_item::InventoryItemType,
    states::{GameMenuState, GameState},
};
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
struct WandSpellSprite {
    wand_index: usize,
    spell_index: usize,
}

pub fn spawn_wand_spell_slot(
    mut parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
    spell_index: usize,
) {
    spawn_item_panel(
        &mut parent,
        &assets,
        WandSpellSprite {
            wand_index,
            spell_index,
        },
        64.0 + 32. * (spell_index as f32),
        0.0,
        None,
        Some(BackgroundColor(slot_color(wand_index, spell_index))),
    );
}

fn update_spell_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSpellSprite, &mut ItemPanel)>,
    floating_query: Query<&Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        let float = floating_query.single();
        for (sprite, mut panel) in sprite_query.iter_mut() {
            panel.0 = match float.content {
                Some(FloatingContent::WandSpell(w, s))
                    if w == sprite.wand_index && s == sprite.spell_index =>
                {
                    None
                }
                _ => actor
                    .get_wand_spell(sprite.wand_index, sprite.spell_index)
                    .map(|e| InventoryItem {
                        item_type: InventoryItemType::Spell(e.spell_type),
                        price: e.price,
                    }),
            };
        }
    }
}

fn interaction_spell_sprite(
    mut interaction_query: Query<(&WandSpellSprite, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    let mut floating = floating_query.single_mut();

    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                floating.content = Some(FloatingContent::WandSpell(
                    slot.wand_index,
                    slot.spell_index,
                ));
            }
            Interaction::Hovered => {
                floating.target = Some(FloatingContent::WandSpell(
                    slot.wand_index,
                    slot.spell_index,
                ));
            }
            _ => {}
        }
    }
}

fn slot_color(wand_index: usize, spell_index: usize) -> Color {
    return Color::hsla(
        60.0,
        0.3,
        0.4,
        if (wand_index + spell_index) % 2 == 0 {
            0.1
        } else {
            0.12
        },
    );
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
