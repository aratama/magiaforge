use super::{
    floating::{Floating, FloatingContent},
    wand_list::slot_color,
};
use crate::{
    asset::GameAssets,
    controller::player::Player,
    entity::actor::Actor,
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
                            match floating.content {
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

pub struct SpellInWandPlugin;

impl Plugin for SpellInWandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_spell_sprite, interaction_spell_sprite).run_if(in_state(GameState::InGame)),
        );
    }
}
