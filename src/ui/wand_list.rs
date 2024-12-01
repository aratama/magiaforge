use super::{
    floating::{Floating, FloatingContent},
    spell_in_wand::spawn_wand_spell_slot,
    wand_sprite::spawn_wand_sprite_in_list,
};
use crate::{
    asset::GameAssets,
    constant::{MAX_SPELLS_IN_WAND, MAX_WANDS},
    controller::player::Player,
    entity::actor::Actor,
    states::GameState,
};
use bevy::{
    prelude::*,
    ui::{Display, Node},
};

#[derive(Component)]
pub struct WandList;

#[derive(Component)]
pub struct WandSlot {
    wand_index: usize,
}

pub fn spawn_wand_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            WandList,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.),
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
            BorderColor(Color::hsla(0.0, 0.0, 1.0, 0.0)),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_wand_sprite_in_list(&mut parent, &assets, wand_index);

            for spell_index in 0..MAX_SPELLS_IN_WAND {
                spawn_wand_spell_slot(&mut parent, &assets, wand_index, spell_index);
            }
        });
}

fn update_wand_slot_visibility(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSlot, &mut BorderColor, &mut Visibility)>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();
    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut border, mut visibility) in sprite_query.iter_mut() {
            *visibility = match floating.0 {
                Some(FloatingContent::Wand(index)) if index == wand_sprite.wand_index => {
                    Visibility::Hidden
                }
                _ => Visibility::Inherited,
            };

            if wand_sprite.wand_index == actor.current_wand {
                *border = Color::hsla(0.0, 0.0, 1.0, 0.3).into();
            } else {
                *border = Color::hsla(0.0, 0.0, 1.0, 0.0).into();
            };
        }
    }
}

pub fn slot_color(wand_index: usize, spell_index: usize) -> Color {
    return Color::hsla(
        0.0,
        0.0,
        0.4,
        if (wand_index + spell_index) % 2 == 0 {
            0.1
        } else {
            0.12
        },
    );
}

pub struct WandListPlugin;

impl Plugin for WandListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_wand_slot_visibility,).run_if(in_state(GameState::InGame)),
        );
    }
}
