use crate::asset::GameAssets;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::item_panel::spawn_item_panel;
use crate::ui::item_panel::ItemPanel;
use crate::ui::popup::PopUp;
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

fn update_panel_item(
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

fn update_spell_sprite_visibility(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSpellSprite, &mut Visibility), With<ItemPanel>>,
    floating_query: Query<&Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (sprite, mut visibility) in sprite_query.iter_mut() {
            let float = floating_query.single();
            match float.content {
                Some(FloatingContent::Wand(w)) if w == sprite.wand_index => {
                    *visibility = Visibility::Hidden;
                }
                _ => {
                    match actor.get_wand(sprite.wand_index) {
                        Some(wand) if sprite.spell_index < wand.wand_type.to_props().capacity => {
                            *visibility = Visibility::default();
                        }
                        _ => *visibility = Visibility::Hidden,
                    };
                }
            }
        }
    }
}

fn interaction_spell_sprite(
    mut interaction_query: Query<(&WandSpellSprite, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
    mut popup_query: Query<&mut PopUp>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    let mut floating = floating_query.single_mut();
    let mut popup = popup_query.single_mut();

    for (slot, interaction) in &mut interaction_query {
        let content = FloatingContent::WandSpell(slot.wand_index, slot.spell_index);
        match *interaction {
            Interaction::Pressed => {
                floating.content = Some(FloatingContent::WandSpell(
                    slot.wand_index,
                    slot.spell_index,
                ));
            }
            Interaction::Hovered => {
                floating.target = Some(content);
                popup.set.insert(content);
                popup.hang = false;
            }
            Interaction::None => {
                popup.set.remove(&content);
            }
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
            (
                update_panel_item,
                interaction_spell_sprite,
                update_spell_sprite_visibility,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
