use crate::actor::Actor;
use crate::constant::WAND_EDITOR_Z_INDEX;
use crate::controller::player::Player;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::M18NTtext;
use crate::message::UNPAID;
use crate::registry::Registry;
use crate::spell::get_spell_appendix;
use crate::spell::Spell;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::wand_editor::MENU_THEME_COLOR;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;

const POPUP_WIDTH: f32 = 300.0;

const POPUP_HEIGHT: f32 = 300.0;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PopupContent {
    FloatingContent(FloatingContent),
    DiscoveredSpell(Spell),
}

#[derive(Component)]
pub struct PopUp {
    pub set: HashSet<PopupContent>,
    pub anchor_top: bool,
    pub anchor_left: bool,
    pub opacity: f32,
}

#[derive(Component)]
struct PopUpItemIcon;

#[derive(Component)]
struct PopUpItemName;

#[derive(Component)]
struct PopUpItemDescription;

pub fn spawn_spell_information(parent: &mut ChildBuilder, registry: &Registry) {
    parent
        .spawn((
            PopUp {
                set: HashSet::new(),
                anchor_top: false,
                anchor_left: false,
                opacity: 0.0,
            },
            BackgroundColor(MENU_THEME_COLOR),
            GlobalZIndex(WAND_EDITOR_Z_INDEX),
            // background_color: Color::hsla(0.0, 0.0, 0.2, 0.95).into(),
            Node {
                position_type: PositionType::Absolute,
                display: Display::None,
                padding: UiRect::all(Val::Px(8.0)),
                width: Val::Px(POPUP_WIDTH),
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
                            aseprite: registry.assets.atlas.clone(),
                            name: "empty".into(),
                        },
                    ));

                    parent.spawn((
                        PopUpItemName,
                        M18NTtext::empty(),
                        TextFont {
                            font: registry.assets.noto_sans_jp.clone(),
                            ..default()
                        },
                    ));
                });

            parent.spawn((
                PopUpItemDescription,
                M18NTtext::empty(),
                TextFont {
                    font: registry.assets.noto_sans_jp.clone(),
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
            info.left = Val::Px(cursor.x - if popup.anchor_left { 0.0 } else { POPUP_WIDTH });
            info.top = Val::Px(cursor.y - if popup.anchor_top { 0.0 } else { POPUP_HEIGHT });
        }
    }
}

fn update_spell_icon(
    registry: Registry,
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
            match first {
                PopupContent::FloatingContent(content) => match content.get_item(actor) {
                    Some(InventoryItem {
                        item_type: InventoryItemType::Spell(spell),
                        ..
                    }) => {
                        let props = registry.get_spell_props(&spell);
                        slice.name = props.icon.clone();
                    }
                    None => {}
                },
                PopupContent::DiscoveredSpell(spell) => {
                    let props = registry.get_spell_props(spell);
                    slice.name = props.icon.clone();
                }
            }
        }
    }
}

fn update_spell_name(
    registry: Registry,
    mut query: Query<&mut M18NTtext, With<PopUpItemName>>,
    popup_query: Query<&PopUp>,
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
        match first {
            Some(PopupContent::FloatingContent(content)) => {
                if let Some(InventoryItem {
                    item_type: InventoryItemType::Spell(spell),
                    ..
                }) = content.get_item(actor)
                {
                    text.0 = registry.get_spell_props(&spell).name.clone();
                }
            }
            Some(PopupContent::DiscoveredSpell(spell)) => {
                let props = registry.get_spell_props(spell);
                text.0 = props.name.clone();
            }
            _ => {}
        }
    }
}

fn update_item_description(
    registry: Registry,
    mut query: Query<&mut M18NTtext, With<PopUpItemDescription>>,
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
        match popup.set.iter().next() {
            Some(PopupContent::FloatingContent(content)) => {
                if let Some(InventoryItem {
                    item_type: InventoryItemType::Spell(spell),
                    ..
                }) = content.get_item(actor)
                {
                    let props = registry.get_spell_props(&spell);
                    let mut dict = props.description.clone();
                    let appendix = get_spell_appendix(&props.cast);
                    // dict += format!("\n{}", appendix).as_str();

                    // dict += format!(
                    //     "\n{}: {}",
                    //     (Dict {
                    //         ja: "ランク",
                    //         en: "Rank",
                    //     })
                    //     .get(config.language),
                    //     props.rank
                    // )
                    // .as_str();

                    dict += appendix;

                    if 0 < props.price {
                        dict = dict + UNPAID.to_string();

                        //  &format!("\n未清算:{}ゴールド", first.price);
                    }

                    text.0 = dict;
                }
            }
            Some(PopupContent::DiscoveredSpell(spell)) => {
                let props: &crate::spell::SpellProps = registry.get_spell_props(spell);
                let mut dict = props.description.clone();
                dict += get_spell_appendix(&props.cast);
                // dict += format!(
                //     "\n{}: {}",
                //     (Dict {
                //         ja: "ランク",
                //         en: "Rank",
                //     })
                //     .get(config.language),
                //     props.rank
                // )
                // .as_str();

                text.0 = dict;
            }
            _ => {}
        }
    }
}

fn update_visible(
    mut popup_query: Query<(&mut PopUp, &mut Node)>,
    floating_query: Query<&Floating>,
    actor_query: Query<(&Actor, &Player)>,
) {
    let (mut popup, mut popup_node) = popup_query.single_mut();

    let floating = floating_query.single();

    let mut visible = false;

    if floating.content == None {
        if let Ok((actor, player)) = actor_query.get_single() {
            if popup.set.is_empty() {
            } else {
                match popup.set.iter().next() {
                    Some(PopupContent::FloatingContent(content)) => {
                        visible = content.get_item(actor).is_some()
                    }
                    Some(PopupContent::DiscoveredSpell(spell)) => {
                        visible = player.discovered_spells.contains(spell);
                    }
                    _ => {}
                }
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

fn reset(mut popup_query: Query<(&mut PopUp, &mut Node)>) {
    let (mut popup, mut popup_node) = popup_query.single_mut();
    popup.set = HashSet::new();
    popup_node.display = Display::None;
    return;
}

pub struct PopUpPlugin;

impl Plugin for PopUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameMenuState::WandEditOpen), reset);

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
