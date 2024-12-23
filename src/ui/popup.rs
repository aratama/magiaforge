use crate::constant::WAND_EDITOR_Z_INDEX;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::inventory_item::InventoryItemType;
use crate::spell::get_spell_appendix;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::wand_editor::MENU_THEME_COLOR;
use crate::{asset::GameAssets, config::GameConfig, states::GameState};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;

const POPUP_HEIGHT: f32 = 300.0;

#[derive(Component)]
pub struct PopUp {
    pub set: HashSet<FloatingContent>,
    pub hang: bool,
    pub opacity: f32,
}

#[derive(Component)]
struct PopUpItemIcon;

#[derive(Component)]
struct PopUpItemName;

#[derive(Component)]
struct PopUpItemDescription;

pub fn spawn_spell_information(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            PopUp {
                set: HashSet::new(),
                hang: false,
                opacity: 0.0,
            },
            BackgroundColor(MENU_THEME_COLOR),
            GlobalZIndex(WAND_EDITOR_Z_INDEX),
            // background_color: Color::hsla(0.0, 0.0, 0.2, 0.95).into(),
            Node {
                position_type: PositionType::Absolute,
                display: Display::None,
                padding: UiRect::all(Val::Px(8.0)),
                width: Val::Px(300.0),
                height: Val::Px(POPUP_HEIGHT),
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
                        PopUpItemIcon,
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
                        PopUpItemName,
                        Text::new(""),
                        TextFont {
                            font: assets.dotgothic.clone(),
                            ..default()
                        },
                    ));
                });

            parent.spawn((
                PopUpItemDescription,
                Text::new(""),
                TextFont {
                    font: assets.dotgothic.clone(),
                    ..default()
                },
            ));
        });
}

fn update_information_position(
    mut spell_info: Query<(&PopUp, &mut Node)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = q_window.get_single() {
        if let Some(cursor) = window.cursor_position() {
            let (popup, mut info) = spell_info.single_mut();
            info.left = Val::Px(cursor.x);
            info.top = Val::Px(cursor.y - if popup.hang { 0.0 } else { POPUP_HEIGHT });
        }
    }
}

fn update_spell_icon(
    mut query: Query<&mut AseUiSlice, With<PopUpItemIcon>>,
    popup_query: Query<&PopUp>,
    floating_query: Query<&Floating>,
    actor_query: Query<&Actor, With<Player>>,
) {
    let floating = floating_query.single();
    if floating.content.is_some() {
        return;
    }

    if let Ok(actor) = actor_query.get_single() {
        let mut slice = query.single_mut();
        let popup = popup_query.single();
        if let Some(first) = popup.set.iter().next() {
            match first.get_item(actor) {
                Some(item) => {
                    let props = item.item_type.to_props();
                    slice.name = props.icon.into();
                }
                None => {}
            }
        }
    }
}

fn update_spell_name(
    mut query: Query<&mut Text, With<PopUpItemName>>,
    popup_query: Query<&PopUp>,
    config: Res<GameConfig>,
    floating_query: Query<&Floating>,
    actor_query: Query<&Actor, With<Player>>,
) {
    let floating = floating_query.single();
    if floating.content.is_some() {
        return;
    }

    if let Ok(actor) = actor_query.get_single() {
        let mut text = query.single_mut();
        let popup = popup_query.single();
        let first = popup.set.iter().next();
        if let Some(first) = first.and_then(|f| f.get_item(actor)) {
            text.0 = first.item_type.to_props().name.get(config.language);
        }
    }
}

fn update_item_description(
    mut query: Query<&mut Text, With<PopUpItemDescription>>,
    config: Res<GameConfig>,
    floating_query: Query<&Floating>,
    actor_query: Query<&Actor, With<Player>>,
    popup_query: Query<&PopUp>,
) {
    let floating = floating_query.single();
    let popup = popup_query.single();
    if floating.content.is_some() {
        return;
    }
    let mut text = query.single_mut();
    if let Ok(actor) = actor_query.get_single() {
        let first = popup.set.iter().next();
        if let Some(first) = first.and_then(|f| f.get_item(actor)) {
            text.0 = first.item_type.to_props().description.get(config.language);

            if let InventoryItemType::Spell(spell) = first.item_type {
                let props = spell.to_props();
                let cast: crate::spell::SpellCast = props.cast;
                let appendix = get_spell_appendix(cast, config.language);
                text.0 += format!("\n{}", appendix).as_str();
            }

            if 0 < first.price {
                text.0 += &format!("\n未清算:{}ゴールド", first.price);
            }
        }
    }
}

fn update_visible(
    mut popup_query: Query<(&mut PopUp, &mut Node)>,
    floating_query: Query<&Floating>,
    actor_query: Query<&Actor, With<Player>>,
) {
    let (mut popup, mut popup_node) = popup_query.single_mut();
    let floating = floating_query.single();
    let mut visible = false;
    if let Ok(actor) = actor_query.get_single() {
        if popup.set.is_empty() {
        } else if let Some(first) = popup.set.iter().next() {
            if let Some(_) = first.get_item(actor) {
                visible = if floating.content == None {
                    true
                } else {
                    false
                };
            } else {
            }
        }
    }

    popup.opacity = (popup.opacity + if visible { 0.1 as f32 } else { -0.1 })
        .max(0.0)
        .min(1.0);

    popup_node.display = if popup.opacity == 0.0 {
        Display::None
    } else {
        Display::Flex
    };
}

pub struct PopUpPlugin;

impl Plugin for PopUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_spell_name,
                update_item_description,
                update_spell_icon,
                update_information_position,
                update_visible,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
