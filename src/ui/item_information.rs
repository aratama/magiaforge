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
use bevy_aseprite_ultra::prelude::{AsepriteSlice, AsepriteSliceUiBundle};

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

pub fn spawn_spell_information(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            SpellInformation(None),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(8.0)),
                    width: Val::Px(300.0),
                    height: Val::Px(500.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                // background_color: Color::hsla(0.0, 0.0, 0.2, 0.95).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        SpellIcon,
                        ImageBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
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

                    parent.spawn((
                        SpellName,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: assets.dotgothic.clone(),
                                ..default()
                            },
                        ),
                    ));
                });

            parent.spawn((
                SpellDescription,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: assets.dotgothic.clone(),
                        ..default()
                    },
                ),
            ));
        });
}

fn update_spell_icon(
    mut query: Query<&mut AsepriteSlice, With<SpellIcon>>,
    spell_info: Query<&SpellInformation>,
    floating_query: Query<&Floating>,
) {
    let Floating(floating) = floating_query.single();
    if floating.is_some() {
        return;
    }

    let mut slice = query.single_mut();
    let spell_info = spell_info.single();
    match spell_info {
        SpellInformation(Some(SpellInformationItem::InventoryItem(item))) => {
            let props = inventory_item_to_props(*item);
            *slice = AsepriteSlice::new(props.icon);
        }
        SpellInformation(Some(SpellInformationItem::Wand(wand))) => {
            let props = wand_to_props(*wand);
            *slice = AsepriteSlice::new(props.slice);
        }
        _ => {
            *slice = AsepriteSlice::new("empty");
        }
    }
}

fn update_spell_name(
    mut query: Query<&mut Text, With<SpellName>>,
    spell_info: Query<&SpellInformation>,
    config: Res<GameConfig>,
    floating_query: Query<&Floating>,
) {
    let Floating(floating) = floating_query.single();
    if floating.is_some() {
        return;
    }

    let mut text = query.single_mut();
    let spell_info = spell_info.single();
    match spell_info {
        SpellInformation(Some(SpellInformationItem::InventoryItem(item))) => {
            let props = inventory_item_to_props(*item);
            text.sections[0].value = props.name.get(config.language).to_string();
        }
        SpellInformation(Some(SpellInformationItem::Wand(wand))) => {
            let props = wand_to_props(*wand);
            text.sections[0].value = props.name.get(config.language).to_string();
        }
        _ => {
            text.sections[0].value = "".to_string();
        }
    }
}

fn update_spell_description(
    mut query: Query<&mut Text, With<SpellDescription>>,
    spell_info: Query<&SpellInformation>,
    config: Res<GameConfig>,
    floating_query: Query<&Floating>,
) {
    let Floating(floating) = floating_query.single();
    if floating.is_some() {
        return;
    }

    let mut text = query.single_mut();
    let spell_info = spell_info.single();
    match spell_info {
        SpellInformation(Some(SpellInformationItem::InventoryItem(item))) => {
            text.sections[0].value = get_inventory_item_description(*item, config.language);
        }
        SpellInformation(Some(SpellInformationItem::Wand(wand))) => {
            let props = wand_to_props(*wand);
            text.sections[0].value = props.description.get(config.language).to_string();
        }
        _ => {
            text.sections[0].value = "".to_string();
        }
    }
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
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
