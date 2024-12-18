use crate::{
    asset::GameAssets,
    constant::{MAX_SPELLS_IN_WAND, WAND_EDITOR_FLOATING_Z_INDEX},
    controller::player::{Equipment, Player},
    entity::{actor::Actor, dropped_item::spawn_dropped_item},
    hud::DropArea,
    inventory::InventoryItem,
    inventory_item::InventoryItemType,
    level::{tile::Tile, CurrentLevel},
    se::{SEEvent, SE},
    states::{GameMenuState, GameState},
    wand::{Wand, WandSpell},
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

impl FloatingContent {
    pub fn get_item(&self, actor: &Actor) -> Option<InventoryItem> {
        match self {
            FloatingContent::Inventory(index) => actor.inventory.get(*index),
            FloatingContent::WandSpell(wand_index, spell_index) => actor.wands[*wand_index]
                .clone()
                .and_then(|ref wand| match wand.slots[*spell_index] {
                    Some(spell) => Some(InventoryItem {
                        item_type: InventoryItemType::Spell(spell.spell_type),
                        price: spell.price,
                    }),
                    None => None,
                }),
            FloatingContent::Wand(wand_index) => {
                actor.wands[*wand_index].clone().and_then(|ref wand| {
                    Some(InventoryItem {
                        item_type: InventoryItemType::Wand(wand.wand_type),
                        price: wand.price,
                    })
                })
            }
            FloatingContent::Equipment(index) => {
                actor.equipments[*index].clone().map(|ref e| InventoryItem {
                    item_type: InventoryItemType::Equipment(e.equipment_type),
                    price: e.price,
                })
            }
        }
    }
}

#[derive(Component)]
pub struct Floating {
    pub content: Option<FloatingContent>,
    pub target: Option<FloatingContent>,
}

#[derive(Component)]
struct ItemFrame;

#[derive(Component)]
struct ChargeAlert;

impl Floating {
    pub fn get_item(&self, actor: &Actor) -> Option<InventoryItem> {
        self.content.and_then(|c| c.get_item(actor))
    }
}

pub fn spawn_inventory_floating(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
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
        ))
        .with_children(|builder| {
            builder.spawn((
                ItemFrame,
                AseUiSlice {
                    aseprite: assets.atlas.clone(),
                    name: "spell_frame".into(),
                },
                ZIndex(1),
            ));
            builder.spawn((
                ChargeAlert,
                AseUiSlice {
                    aseprite: assets.atlas.clone(),
                    name: "charge_alert".into(),
                },
            ));
        });
}

fn update_item_frame(
    query: Query<&Actor, With<Player>>,
    floating_query: Query<&Floating>,
    mut frame_query: Query<&mut AseUiSlice, With<ItemFrame>>,
) {
    if let Ok(actor) = query.get_single() {
        let mut aseprite = frame_query.single_mut();
        let floating = floating_query.single();
        match floating.content.and_then(|f| f.get_item(actor)) {
            Some(InventoryItem {
                item_type: InventoryItemType::Spell(..),
                ..
            }) => {
                aseprite.name = "spell_frame".into();
            }
            Some(InventoryItem {
                item_type: InventoryItemType::Equipment(..),
                ..
            }) => {
                aseprite.name = "equipment_frame".into();
            }
            Some(InventoryItem {
                item_type: InventoryItemType::Wand(..),
                ..
            }) => {
                aseprite.name = "wand_frame".into();
            }
            _ => {
                aseprite.name = "empty".into();
            }
        }
    }
}

fn update_inventory_floaing_position(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Node, With<Floating>>,
) {
    let mut style = query.single_mut();
    if let Ok(window) = windows_query.get_single() {
        if let Some(position) = window.cursor_position() {
            style.left = Val::Px(position.x - 16.0);
            style.top = Val::Px(position.y - 16.0);
        }
    }
}

fn update_floating_visibility(mut query: Query<(&Floating, &mut Visibility), Changed<Floating>>) {
    if let Ok((floating, mut visibility)) = query.get_single_mut() {
        *visibility = match floating.content {
            Some(_) => Visibility::Visible,
            None => Visibility::Hidden,
        };
    }
}

fn update_floating_visibility_by_menu_close(
    mut query: Query<&mut Floating>,
    menu: Res<State<GameMenuState>>,
) {
    if menu.is_changed() {
        if *menu.get() == GameMenuState::Closed {
            let mut floating = query.single_mut();
            floating.content = None;
        }
    }
}

fn update_alert_visibility(
    player_query: Query<&Actor, With<Player>>,
    floating_query: Query<&Floating>,
    mut alert_query: Query<&mut Visibility, With<ChargeAlert>>,
) {
    if let Ok(actor) = player_query.get_single() {
        let floating = floating_query.single();
        let mut alert = alert_query.single_mut();
        *alert = match floating.get_item(actor) {
            Some(item) if 0 < item.price => Visibility::Inherited,
            _ => Visibility::Hidden,
        };
    }
}

fn update_floating_slice(
    player_query: Query<&Actor, With<Player>>,
    mut floating_query: Query<(&Floating, &mut AseUiSlice, &mut Node), With<Floating>>,
) {
    if let Ok(actor) = player_query.get_single() {
        let (floating, mut floating_slice, mut style) = floating_query.single_mut();
        if let Some(item) = floating.get_item(actor) {
            floating_slice.name = item.item_type.get_icon().to_string();
            style.width = Val::Px(item.item_type.get_icon_width());
        } else {
            floating_slice.name = "empty".into();
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
    mut player_query: Query<&mut Actor, With<Player>>,
    drop_query: Query<&DropArea>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    map: Res<CurrentLevel>,
    mut se: EventWriter<SEEvent>,
) {
    let mut floating = floating_query.single_mut();
    if mouse.just_released(MouseButton::Left) {
        let content_optional = floating.content;
        let target_optional = floating.target;

        floating.content = None;
        floating.target = None;

        if let Ok(mut actor) = player_query.get_single_mut() {
            if let Some(content) = content_optional {
                let drop = drop_query.single();

                if drop.hover {
                    if let Some(ref chunk) = map.chunk {
                        if let Ok(window) = q_window.get_single() {
                            if let Some(cursor_in_screen) = window.cursor_position() {
                                if let Ok((camera, camera_global_transform)) =
                                    camera_query.get_single()
                                {
                                    if let Ok(mouse_in_world) = camera.viewport_to_world(
                                        camera_global_transform,
                                        cursor_in_screen,
                                    ) {
                                        let pointer_in_world = mouse_in_world.origin.truncate();

                                        if chunk.get_tile_by_coords(pointer_in_world)
                                            == Tile::StoneTile
                                        {
                                            if let Some(item) = content.get_inventory_item(&actor) {
                                                let spells = content.get_wand_spells(&actor);
                                                content.set_item(None, &spells, &mut actor, false);

                                                spawn_dropped_item(
                                                    &mut commands,
                                                    &assets,
                                                    pointer_in_world,
                                                    item,
                                                );
                                                floating.content = None;

                                                se.send(SEEvent::new(SE::PickUp));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(target) = target_optional {
                    // 移動元のアイテムを取得
                    let item_optional_from = content.get_inventory_item(&actor);
                    // 移動先のアイテムを取得
                    let item_optional_to = target.get_inventory_item(&actor);
                    // 移動元が杖の場合は杖に含まれている魔法を取得
                    let spells_from = content.get_wand_spells(&actor);

                    let spells_to = target.get_wand_spells(&actor);

                    // 移動先に書きこみ
                    let ok_target =
                        target.set_item(item_optional_from, &spells_from, &mut actor, true);
                    // 移動元に書きこみ
                    let ok_content =
                        content.set_item(item_optional_to, &spells_to, &mut actor, true);

                    if ok_target && ok_content {
                        // 移動先に書きこみ
                        target.set_item(item_optional_from, &spells_from, &mut actor, false);
                        // 移動元に書きこみ
                        content.set_item(item_optional_to, &spells_to, &mut actor, false);
                    }
                }
            }
        }
    }
}

impl FloatingContent {
    pub fn get_inventory_item(&self, actor: &Actor) -> Option<InventoryItem> {
        match self {
            FloatingContent::Inventory(i) => actor.inventory.get(*i),
            FloatingContent::WandSpell(w, i) => actor.get_spell(*w, *i).map(|w| InventoryItem {
                item_type: InventoryItemType::Spell(w.spell_type),
                price: w.price,
            }),
            FloatingContent::Wand(w) => {
                if let Some(ref wand) = actor.wands[*w] {
                    Some(InventoryItem {
                        item_type: InventoryItemType::Wand(wand.wand_type),
                        price: wand.price,
                    })
                } else {
                    None
                }
            }
            FloatingContent::Equipment(e) => actor.equipments[*e].map(|e| InventoryItem {
                item_type: InventoryItemType::Equipment(e.equipment_type),
                price: e.price,
            }),
        }
    }

    pub fn get_wand_spells(&self, actor: &Actor) -> Box<[Option<WandSpell>; MAX_SPELLS_IN_WAND]> {
        Box::new(match self {
            FloatingContent::Wand(w) => {
                if let Some(ref wand) = actor.wands[*w] {
                    wand.slots
                } else {
                    [None; MAX_SPELLS_IN_WAND]
                }
            }
            _ => [None; MAX_SPELLS_IN_WAND],
        })
    }

    pub fn set_item(
        &self,
        item: Option<InventoryItem>,
        slots: &[Option<WandSpell>; MAX_SPELLS_IN_WAND],
        actor: &mut Actor,
        dry_run: bool,
    ) -> bool {
        match (self, item) {
            (FloatingContent::Inventory(i), _) => {
                if !dry_run {
                    actor.inventory.set(*i, item);
                    for spell in slots.iter() {
                        if let Some(spell) = spell {
                            actor.inventory.insert(InventoryItem {
                                item_type: InventoryItemType::Spell(spell.spell_type),
                                price: spell.price,
                            });
                        }
                    }
                }
                true
            }
            (
                FloatingContent::Wand(w),
                Some(InventoryItem {
                    item_type: InventoryItemType::Wand(wand_type),
                    price,
                }),
            ) => {
                if !dry_run {
                    actor.wands[*w] = Some(Wand {
                        wand_type,
                        price,
                        slots: *slots,
                        index: 0,
                    });
                }
                true
            }
            (FloatingContent::Wand(w), None) => {
                if !dry_run {
                    actor.wands[*w] = None
                }
                true
            }
            (
                FloatingContent::WandSpell(w, s),
                Some(InventoryItem {
                    item_type: InventoryItemType::Spell(spell_type),
                    price,
                }),
            ) => {
                if !dry_run {
                    if let Some(ref mut wand) = actor.wands[*w] {
                        wand.slots[*s] = Some(WandSpell { spell_type, price });
                    }
                }
                true
            }
            (FloatingContent::WandSpell(w, s), None) => {
                if !dry_run {
                    if let Some(ref mut wand) = actor.wands[*w] {
                        wand.slots[*s] = None;
                    }
                }
                true
            }
            (
                FloatingContent::Equipment(e),
                Some(InventoryItem {
                    item_type: InventoryItemType::Equipment(equipment),
                    price,
                }),
            ) => {
                if !dry_run {
                    actor.equipments[*e] = Some(Equipment {
                        equipment_type: equipment,
                        price,
                    })
                }
                true
            }
            (FloatingContent::Equipment(e), None) => {
                if !dry_run {
                    actor.equipments[*e] = None;
                }
                true
            }
            _ => {
                warn!("Invalid operation dest:{:?} item:{:?}", self, item);
                false
            }
        }
    }
}

pub struct InventoryItemFloatingPlugin;

impl Plugin for InventoryItemFloatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_inventory_floaing_position,
                update_floating_slice,
                cancel_on_close,
                drop,
                update_alert_visibility,
                update_item_frame,
            )
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(GameMenuState::WandEditOpen)),
        );

        app.add_systems(
            Update,
            (
                update_floating_visibility,
                update_floating_visibility_by_menu_close,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
