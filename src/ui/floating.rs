use crate::{
    asset::GameAssets,
    constant::{MAX_SPELLS_IN_WAND, WAND_EDITOR_FLOATING_Z_INDEX},
    controller::player::Player,
    entity::actor::Actor,
    equipment::equipment_to_props,
    inventory_item::{inventory_item_to_props, InventoryItem},
    spell::SpellType,
    spell_props::spell_to_props,
    states::{GameMenuState, GameState},
    wand::Wand,
    wand_props::wand_to_props,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FloatingContent {
    Inventory(usize),
    WandSpell(usize, usize),
    Wand(usize),
    Equipment(usize),
}

#[derive(Component)]
pub struct Floating {
    pub content: Option<FloatingContent>,
    pub target: Option<FloatingContent>,
}

impl Floating {
    pub fn get_item(&self, player: &Player, actor: &Actor) -> Option<InventoryItem> {
        match self.content {
            None => None,
            Some(FloatingContent::Inventory(index)) => player.inventory.get(index),
            Some(FloatingContent::WandSpell(wand_index, spell_index)) => actor.wands[wand_index]
                .clone()
                .and_then(|ref w| w.slots[spell_index])
                .map(|ref w| InventoryItem::Spell(*w)),
            Some(FloatingContent::Wand(wand_index)) => actor.wands[wand_index]
                .clone()
                .map(|ref w| InventoryItem::Wand(w.wand_type)),
            Some(FloatingContent::Equipment(index)) => player.equipments[index]
                .clone()
                .map(|ref e| InventoryItem::Equipment(*e)),
        }
    }
}

pub fn spawn_inventory_floating(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder.spawn((
        Floating {
            content: None,
            target: None,
        },
        Interaction::default(),
        GlobalZIndex(WAND_EDITOR_FLOATING_Z_INDEX),
        Visibility::Hidden,
        // background_color: Color::hsla(0.0, 0.0, 0.0, 0.5).into(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(500.0),
            left: Val::Px(500.0),
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            // border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "empty".into(),
        },
    ));
}

fn update_inventory_floaing(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Node, &mut Floating)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (mut style, mut floating) = query.single_mut();
    if let Ok(window) = windows_query.get_single() {
        if let Some(position) = window.cursor_position() {
            style.left = Val::Px(position.x - 16.0);
            style.top = Val::Px(position.y - 16.0);
        }
    }

    if mouse.just_pressed(MouseButton::Right) {
        floating.content = None;
    }
}

fn switch_floating_visibility(mut query: Query<(&Floating, &mut Visibility)>) {
    let (floating, mut visibility) = query.single_mut();
    *visibility = match floating.content {
        Some(_) => Visibility::Visible,
        None => Visibility::Hidden,
    };
}

fn switch_floating_slice(
    player_query: Query<(&Player, &Actor)>,
    mut floating_query: Query<(&Floating, &mut AseUiSlice, &mut Node), With<Floating>>,
) {
    if let Ok((player, actor)) = player_query.get_single() {
        let (floating, mut floating_slice, mut style) = floating_query.single_mut();
        match floating.content {
            Some(FloatingContent::Inventory(index)) => {
                let item = player.inventory.get(index);

                let slice = match item {
                    Some(item) => {
                        let props = inventory_item_to_props(item);
                        Some(props.icon)
                    }
                    _ => None,
                };
                if let Some(slice) = slice {
                    floating_slice.name = slice.into();
                    style.width = Val::Px(32.0);
                }

                style.width = match item {
                    Some(InventoryItem::Wand(_)) => Val::Px(64.0),
                    _ => Val::Px(32.0),
                }
            }
            Some(FloatingContent::WandSpell(wand_index, spell_index)) => {
                match &actor.wands[wand_index] {
                    Some(wand) => match wand.slots[spell_index] {
                        Some(spell) => {
                            let props = spell_to_props(spell);
                            floating_slice.name = props.icon.into();
                            style.width = Val::Px(32.0);
                        }
                        _ => {
                            floating_slice.name = "empty".into();
                        }
                    },
                    None => {
                        floating_slice.name = "empty".into();
                    }
                }
            }
            Some(FloatingContent::Wand(wand_index)) => match &actor.wands[wand_index] {
                Some(wand) => {
                    let props = wand_to_props(wand.wand_type);
                    floating_slice.name = props.icon.into();
                    style.width = Val::Px(64.0);
                }
                None => {
                    floating_slice.name = "empty".into();
                }
            },
            Some(FloatingContent::Equipment(equipment)) => match player.equipments[equipment] {
                Some(equipment) => {
                    let props = equipment_to_props(equipment);
                    floating_slice.name = props.icon.into();
                    style.width = Val::Px(32.0);
                }
                None => {
                    floating_slice.name = "empty".into();
                }
            },
            None => {
                floating_slice.name = "empty".into();
            }
        }
    }
}

fn cancel_on_close(state: Res<State<GameMenuState>>, mut floating_query: Query<&mut Floating>) {
    if state.is_changed() {
        if *state.get() == GameMenuState::Closed {
            let mut floating = floating_query.single_mut();
            floating.content = None;
        }
    }
}

fn drop(
    mouse: Res<ButtonInput<MouseButton>>,
    mut floating_query: Query<&mut Floating>,
    mut player_query: Query<(&mut Player, &mut Actor)>,
) {
    let mut floating = floating_query.single_mut();
    if mouse.just_released(MouseButton::Left) {
        let content_optional = floating.content;
        let target_optional = floating.target;

        floating.content = None;
        floating.target = None;

        if let Ok((mut player, mut actor)) = player_query.get_single_mut() {
            if let Some(content) = content_optional {
                if let Some(target) = target_optional {
                    // 移動元のアイテムを取得
                    let item_optional_from = get_inventory_item(content, &player, &actor);
                    // 移動先のアイテムを取得
                    let item_optional_to = get_inventory_item(target, &player, &actor);
                    // 移動元が杖の場合は杖に含まれている魔法を取得
                    let spells_from = match content {
                        FloatingContent::Wand(w) => {
                            if let Some(ref wand) = actor.wands[w] {
                                wand.slots
                            } else {
                                [None; MAX_SPELLS_IN_WAND]
                            }
                        }
                        _ => [None; MAX_SPELLS_IN_WAND],
                    };
                    let spells_to = match target {
                        FloatingContent::Wand(w) => {
                            if let Some(ref wand) = actor.wands[w] {
                                wand.slots
                            } else {
                                [None; MAX_SPELLS_IN_WAND]
                            }
                        }
                        _ => [None; MAX_SPELLS_IN_WAND],
                    };
                    // 移動先に書きこみ
                    let ok_target = set_item(
                        target,
                        item_optional_from,
                        &spells_from,
                        &mut player,
                        &mut actor,
                        true,
                    );
                    // 移動元に書きこみ
                    let ok_content = set_item(
                        content,
                        item_optional_to,
                        &spells_to,
                        &mut player,
                        &mut actor,
                        true,
                    );

                    if ok_target && ok_content {
                        // 移動先に書きこみ
                        set_item(
                            target,
                            item_optional_from,
                            &spells_from,
                            &mut player,
                            &mut actor,
                            false,
                        );
                        // 移動元に書きこみ
                        set_item(
                            content,
                            item_optional_to,
                            &spells_to,
                            &mut player,
                            &mut actor,
                            false,
                        );
                    }
                }
            }
        }
    }
}

fn get_inventory_item(
    content: FloatingContent,
    player: &Player,
    actor: &Actor,
) -> Option<InventoryItem> {
    match content {
        FloatingContent::Inventory(i) => player.inventory.get(i),
        FloatingContent::WandSpell(w, i) => actor.get_spell(w, i).map(InventoryItem::Spell),
        FloatingContent::Wand(w) => {
            if let Some(ref wand) = actor.wands[w] {
                Some(InventoryItem::Wand(wand.wand_type))
            } else {
                None
            }
        }
        FloatingContent::Equipment(e) => player.equipments[e].map(|e| InventoryItem::Equipment(e)),
    }
}

fn set_item(
    dest: FloatingContent,
    item: Option<InventoryItem>,
    slots: &[Option<SpellType>; MAX_SPELLS_IN_WAND],
    player: &mut Player,
    actor: &mut Actor,
    dry_run: bool,
) -> bool {
    match (dest, item) {
        (FloatingContent::Inventory(i), _) => {
            if !dry_run {
                player.inventory.set(i, item);
                for spell in slots.iter() {
                    if let Some(spell) = spell {
                        player.inventory.insert(InventoryItem::Spell(*spell));
                    }
                }
            }
            true
        }
        (FloatingContent::Wand(w), Some(InventoryItem::Wand(wand_type))) => {
            if !dry_run {
                actor.wands[w] = Some(Wand {
                    wand_type,
                    slots: *slots,
                    index: 0,
                });
            }
            true
        }
        (FloatingContent::Wand(w), None) => {
            if !dry_run {
                actor.wands[w] = None
            }
            true
        }
        (FloatingContent::WandSpell(w, s), Some(InventoryItem::Spell(spell_type))) => {
            if !dry_run {
                if let Some(ref mut wand) = actor.wands[w] {
                    wand.slots[s] = Some(spell_type);
                }
            }
            true
        }
        (FloatingContent::WandSpell(w, s), None) => {
            if !dry_run {
                if let Some(ref mut wand) = actor.wands[w] {
                    wand.slots[s] = None;
                }
            }
            true
        }
        (FloatingContent::Equipment(e), Some(InventoryItem::Equipment(equipment))) => {
            if !dry_run {
                player.equipments[e] = Some(equipment)
            }
            true
        }
        (FloatingContent::Equipment(e), None) => {
            if !dry_run {
                player.equipments[e] = None;
            }
            true
        }
        _ => {
            warn!("Invalid operation dest:{:?} item:{:?}", dest, item);
            false
        }
    }
}

pub struct InventoryItemFloatingPlugin;

impl Plugin for InventoryItemFloatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_inventory_floaing,
                switch_floating_visibility,
                switch_floating_slice,
                cancel_on_close,
                drop,
            )
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(GameMenuState::WandEditOpen)),
        );
    }
}
