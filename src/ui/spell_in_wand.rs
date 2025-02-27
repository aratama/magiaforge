use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::controller::player::Player;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::KETTEI_7;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::item_panel::spawn_item_panel;
use crate::ui::item_panel::ItemPanel;
use crate::ui::popup::PopUp;
use crate::ui::popup::PopupContent;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
struct WandSpellSprite {
    wand_index: usize,
    spell_index: usize,
}

pub fn spawn_wand_spell_slot(
    mut parent: &mut ChildBuilder,
    registry: &Registry,
    wand_index: usize,
    spell_index: usize,
) {
    spawn_item_panel(
        &mut parent,
        &registry,
        WandSpellSprite {
            wand_index,
            spell_index,
        },
        32.0 + 32. * (spell_index as f32),
        0.0,
        None,
        Some(BackgroundColor(slot_color(wand_index, spell_index))),
    );
}

fn update_panel_item(
    player_query: Query<&Actor, (With<Player>, With<Witch>)>,
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
                _ => actor.get_wand_spell(sprite.wand_index, sprite.spell_index),
            };
        }
    }
}

fn interaction_spell_sprite(
    mut interaction_query: Query<(&WandSpellSprite, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
    mut popup_query: Query<&mut PopUp>,
    mut se: EventWriter<SEEvent>,
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
                se.send(SEEvent::new(KETTEI_7));
                floating.content = Some(FloatingContent::WandSpell(
                    slot.wand_index,
                    slot.spell_index,
                ));
            }
            Interaction::Hovered => {
                floating.target = Some(content);
                popup.set.insert(PopupContent::FloatingContent(content));
                popup.anchor_left = true;
                popup.anchor_top = false;
            }
            Interaction::None => {
                popup.set.remove(&PopupContent::FloatingContent(content));
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
            (update_panel_item, interaction_spell_sprite).run_if(in_state(GameState::InGame)),
        );
    }
}
