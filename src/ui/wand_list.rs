use crate::{
    asset::GameAssets,
    constant::{MAX_SPELLS_IN_WAND, MAX_WANDS},
    controller::player::Player,
    entity::actor::Actor,
    spell_props::spell_to_props,
    states::GameState,
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
struct SpellSprite {
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
                    row_gap: Val::Px(4.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            for wand_index in 0..MAX_WANDS {
                spawn_wand_slot(parent, &assets, wand_index);
            }
        });
}

fn spawn_wand_slot(parent: &mut ChildBuilder, assets: &Res<GameAssets>, wand_index: usize) {
    parent
        .spawn((
            WandSlot { wand_index },
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(2.),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                border_color: Color::WHITE.into(),
                background_color: Color::hsla(0.0, 0.0, 1.0, 0.2).into(),
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
                spawn_bullet_slot(&mut parent, &assets, wand_index, spell_index);
            }
        });
}

fn spawn_bullet_slot(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
    spell_index: usize,
) {
    parent.spawn((
        SpellSprite {
            wand_index,
            spell_index,
        },
        ImageBundle {
            style: Style {
                width: Val::Px(32.),
                height: Val::Px(32.),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            ..default()
        },
        BorderColor(Color::WHITE),
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "empty".into(),
            ..default()
        },
    ));
}

fn update_wand_slot_visibility(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSlot, &mut BackgroundColor, &mut BorderColor)>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut background, mut border) in sprite_query.iter_mut() {
            if wand_sprite.wand_index == actor.current_wand {
                *background = Color::hsla(0.0, 0.0, 1.0, 0.2).into();
                *border = BorderColor(Color::hsla(0.0, 0.0, 1.0, 0.5));
            } else {
                *background = Color::hsla(0.0, 0.0, 1.0, 0.0).into();
                *border = BorderColor(Color::hsla(0.0, 0.0, 1.0, 0.1));
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
    mut sprite_query: Query<
        (&SpellSprite, &mut AsepriteSlice, &mut BorderColor),
        Changed<SpellSprite>,
    >,
) {
    if let Ok(actor) = player_query.get_single() {
        for (spell_sprite, mut aseprite, mut border) in sprite_query.iter_mut() {
            if let Some(wand) = &actor.wands[spell_sprite.wand_index] {
                if spell_sprite.spell_index < wand.slots.len() {
                    match wand.slots[spell_sprite.spell_index] {
                        Some(spell) => {
                            let props = spell_to_props(spell);
                            *aseprite = AsepriteSlice::new(props.slice);
                        }
                        None => {
                            *aseprite = AsepriteSlice::new("empty");
                        }
                    }
                } else {
                    *aseprite = AsepriteSlice::new("empty");
                }
            } else {
                *aseprite = AsepriteSlice::new("empty");
            }
        }
    }
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
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
