use crate::ui::floating::Floating;
use crate::{
    asset::GameAssets,
    config::GameConfig,
    inventory_item::{get_inventory_item_description, inventory_item_to_props, InventoryItem},
    states::GameState,
    wand::WandType,
    wand_props::wand_to_props,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;

use super::inventory::InventoryGrid;

#[derive(PartialEq, Eq)]
pub enum SpellInformationItem {
    InventoryItem(InventoryItem),
    Wand(WandType),
}

#[derive(Component)]
pub struct SpellInformation(pub Option<SpellInformationItem>);

#[derive(Component)]
struct SpellIcon;

#[derive(Component)]
struct SpellName;

#[derive(Component)]
struct SpellDescription;

#[derive(Component)]
pub struct SpellInformationRoot;

pub fn spawn_spell_information(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            SpellInformation(None),
            // background_color: Color::hsla(0.0, 0.0, 0.2, 0.95).into(),
            Node {
                padding: UiRect::all(Val::Px(8.0)),
                width: Val::Px(300.0),
                height: Val::Px(300.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        SpellIcon,
                        Node {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        AseUiSlice {
                            aseprite: assets.atlas.clone(),
                            name: "empty".into(),
                        },
                    ));

                    parent.spawn((
                        SpellName,
                        Text::new(""),
                        TextFont {
                            font: assets.dotgothic.clone(),
                            ..default()
                        },
                    ));
                });

            parent.spawn((
                SpellDescription,
                Text::new(""),
                TextFont {
                    font: assets.dotgothic.clone(),
                    ..default()
                },
            ));
        });
}

fn update_information_position(
    mut spell_info: Query<&mut Node, With<SpellInformationRoot>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = q_window.get_single() {
        if let Some(cursor) = window.cursor_position() {
            let mut info = spell_info.single_mut();
            info.left = Val::Px(cursor.x);
            info.top = Val::Px(cursor.y);
        }
    }
}

fn update_spell_icon(
    mut query: Query<&mut AseUiSlice, With<SpellIcon>>,
    spell_info: Query<&SpellInformation>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();
    if floating.content.is_some() {
        return;
    }

    let mut slice = query.single_mut();
    let spell_info = spell_info.single();
    match spell_info {
        SpellInformation(Some(SpellInformationItem::InventoryItem(item))) => {
            let props = inventory_item_to_props(*item);
            slice.name = props.icon.into();
        }
        SpellInformation(Some(SpellInformationItem::Wand(wand))) => {
            let props = wand_to_props(*wand);
            slice.name = props.slice.into();
        }
        _ => {
            slice.name = "empty".into();
        }
    }
}

fn update_spell_name(
    mut query: Query<&mut Text, With<SpellName>>,
    spell_info: Query<&SpellInformation>,
    config: Res<GameConfig>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();
    if floating.content.is_some() {
        return;
    }

    let mut text = query.single_mut();
    let spell_info = spell_info.single();
    match spell_info {
        SpellInformation(Some(SpellInformationItem::InventoryItem(item))) => {
            let props = inventory_item_to_props(*item);
            text.0 = props.name.get(config.language).to_string();
        }
        SpellInformation(Some(SpellInformationItem::Wand(wand))) => {
            let props = wand_to_props(*wand);
            text.0 = props.name.get(config.language).to_string();
        }
        _ => {
            text.0 = "".to_string();
        }
    }
}

fn update_spell_description(
    mut query: Query<&mut Text, With<SpellDescription>>,
    spell_info: Query<&SpellInformation>,
    config: Res<GameConfig>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();
    if floating.content.is_some() {
        return;
    }

    let mut text = query.single_mut();
    let spell_info = spell_info.single();
    match spell_info {
        SpellInformation(Some(SpellInformationItem::InventoryItem(item))) => {
            text.0 = get_inventory_item_description(*item, config.language);
        }
        SpellInformation(Some(SpellInformationItem::Wand(wand))) => {
            let props = wand_to_props(*wand);
            text.0 = props.description.get(config.language).to_string();
        }
        _ => {
            text.0 = "".to_string();
        }
    }
}

fn update_visible(
    mut root_query: Query<&mut Node, With<SpellInformationRoot>>,
    text_query: Query<&SpellInformation>,
    inventory_grid_query: Query<&InventoryGrid>,
    floating_query: Query<&Floating>,
) {
    let mut root = root_query.single_mut();
    let text = text_query.single();
    let grid = inventory_grid_query.single();
    let floating = floating_query.single();
    root.display = if grid.hover && text.0 != None && floating.content == None {
        Display::Flex
    } else {
        Display::None
    };
}

pub struct SpellInformationPlugin;

impl Plugin for SpellInformationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_spell_name,
                update_spell_description,
                update_spell_icon,
                update_information_position,
                update_visible,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
