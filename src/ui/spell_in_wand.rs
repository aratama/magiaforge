use super::floating::{Floating, FloatingContent};
use crate::{
    asset::GameAssets,
    controller::player::Player,
    entity::actor::Actor,
    states::{GameMenuState, GameState},
};
use bevy::{prelude::*, ui::Node};
use bevy_aseprite_ultra::prelude::*;

#[derive(Component, Debug, Clone)]
struct WandSpellSprite {
    wand_index: usize,
    spell_index: usize,
}

#[derive(Component)]
struct ChargeAlert;

pub fn spawn_wand_spell_slot(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
    spell_index: usize,
) {
    parent
        .spawn((
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
        ))
        .with_children(|builder| {
            builder.spawn((
                ChargeAlert,
                AseUiSlice {
                    aseprite: assets.atlas.clone(),
                    name: "charge_alert".into(),
                },
            ));
        });
}

fn update_spell_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSpellSprite, &mut AseUiSlice, &mut Node)>,
    floating_query: Query<&Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (spell_sprite, mut aseprite, mut node) in sprite_query.iter_mut() {
            if let Some(wand) = &actor.wands[spell_sprite.wand_index] {
                node.display = if spell_sprite.spell_index < wand.wand_type.to_props().capacity {
                    Display::default()
                } else {
                    Display::None
                };

                if spell_sprite.spell_index < wand.slots.len() {
                    match wand.slots[spell_sprite.spell_index] {
                        Some(spell) => {
                            let floating = floating_query.single();
                            match floating.content {
                                Some(FloatingContent::WandSpell(wand_index, spell_index))
                                    if wand_index == spell_sprite.wand_index
                                        && spell_index == spell_sprite.spell_index =>
                                {
                                    aseprite.name = "empty".into();
                                }
                                _ => {
                                    let props = spell.spell_type.to_props();
                                    aseprite.name = props.icon.to_string();
                                }
                            }
                        }
                        None => {
                            aseprite.name = "empty".into();
                        }
                    }
                }
            } else {
                aseprite.name = "empty".into();
            }
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

fn update_alert_visibility(
    player_query: Query<&Actor, With<Player>>,
    slot_query: Query<&WandSpellSprite>,
    mut alert_query: Query<(&Parent, &mut Visibility), With<ChargeAlert>>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();
    if let Ok(actor) = player_query.get_single() {
        for (parent, mut alert) in alert_query.iter_mut() {
            if let Ok(slot) = slot_query.get(parent.get()) {
                match floating.content {
                    Some(FloatingContent::WandSpell(w, s))
                        if w == slot.wand_index && s == slot.spell_index =>
                    {
                        *alert = Visibility::Hidden;
                    }
                    _ => {
                        *alert = match actor.get_wand_spell(slot.wand_index, slot.spell_index) {
                            Some(item) if 0 < item.price => Visibility::Inherited,
                            _ => Visibility::Hidden,
                        };
                    }
                }
            } else {
                *alert = Visibility::Hidden;
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
                update_spell_sprite,
                interaction_spell_sprite,
                update_alert_visibility,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
