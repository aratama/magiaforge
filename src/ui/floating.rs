use crate::asset::GameAssets;
use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::WAND_EDITOR_FLOATING_Z_INDEX;
use crate::controller::player::Equipment;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::hud::DropArea;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::level::tile::Tile;
use crate::page::in_game::CurrentLevel;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::item_panel::spawn_item_panel;
use crate::ui::item_panel::ItemPanel;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

pub fn spawn_inventory_floating(mut builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    spawn_item_panel(
        &mut builder,
        &assets,
        Floating {
            content: Some(FloatingContent::Inventory(2)),
            target: None,
        },
        500.0,
        500.0,
        Some(GlobalZIndex(WAND_EDITOR_FLOATING_Z_INDEX)),
        None,
    );
}

fn update_item_frame(
    query: Query<&Actor, With<Player>>,
    mut frame_query: Query<(&Floating, &mut ItemPanel)>,
) {
    if let Ok(actor) = query.get_single() {
        let (floating, mut panel) = frame_query.single_mut();
        panel.0 = floating.content.and_then(|f| f.get_item(actor));
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
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    map: Res<CurrentLevel>,
    mut se: EventWriter<SEEvent>,
) {
    let mut floating = floating_query.single_mut();
    if mouse.just_released(MouseButton::Left) {
        let content_optional = floating.content;
        let target_optional = floating.target;

        info!("from {:?} to {:?}", content_optional, target_optional);

        floating.content = None;
        floating.target = None;

        if let Ok(mut actor) = player_query.get_single_mut() {
            if let Some(content) = content_optional {
                let drop = drop_query.single();

                if drop.hover {
                    if let Some(ref chunk) = map.chunk {
                        if let Ok(window) = window_query.get_single() {
                            if let Some(cursor_in_screen) = window.cursor_position() {
                                let (camera, camera_global_transform) = camera_query.single();

                                if let Ok(mouse_in_world) = camera
                                    .viewport_to_world(camera_global_transform, cursor_in_screen)
                                {
                                    let pointer_in_world = mouse_in_world.origin.truncate();

                                    if chunk.get_tile_by_coords(pointer_in_world) == Tile::StoneTile
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
                        delay: 0,
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
                        wand.index = 0;
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
                cancel_on_close,
                drop,
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
