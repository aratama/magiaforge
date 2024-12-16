use super::{
    floating::{Floating, FloatingContent},
    item_information::{SpellInformation, SpellInformationItem},
};
use crate::{
    asset::GameAssets,
    controller::player::Player,
    entity::actor::Actor,
    states::{GameMenuState, GameState},
    wand_props::wand_to_props,
};
use bevy::{prelude::*, ui::Node};
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
        BackgroundColor(Color::hsla(0.0, 0.0, 0.5, 0.1)),
        Node {
            width: Val::Px(64.),
            height: Val::Px(32.),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "empty".into(),
        },
    ));
}

fn update_wand_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSprite, &mut AseUiSlice)>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();

    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut aseprite) in sprite_query.iter_mut() {
            aseprite.name = match floating.content {
                Some(FloatingContent::Wand(wand_index)) if wand_index == wand_sprite.wand_index => {
                    "empty".into()
                }
                _ => match &actor.wands[wand_sprite.wand_index] {
                    Some(wand) => {
                        let props = wand_to_props(wand.wand_type);
                        props.icon.into()
                    }
                    None => "empty".into(),
                },
            }
        }
    }
}

fn interact_wand_sprite(
    mut interaction_query: Query<(&WandSprite, &Interaction), Changed<Interaction>>,
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
                floating.content = Some(FloatingContent::Wand(slot.wand_index));
            }
            Interaction::Hovered => {
                floating.target = Some(FloatingContent::Wand(slot.wand_index));
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
