use crate::actor::Actor;
use crate::constant::WAND_EDITOR_FLOATING_Z_INDEX;
use crate::controller::player::Player;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::hud::DropArea;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::registry::TileType;
use crate::se::SEEvent;
use crate::se::CURSOR_8;
use crate::se::PICK_UP;
use crate::spell::Spell;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::item_panel::spawn_item_panel;
use crate::ui::item_panel::ItemPanel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FloatingContent {
    Inventory(usize),
    WandSpell(usize, usize),
}

impl FloatingContent {
    pub fn get_item(&self, actor: &Actor) -> Option<Spell> {
        match self {
            FloatingContent::Inventory(index) => actor.inventory.get(*index).clone(),
            FloatingContent::WandSpell(wand_index, spell_index) => {
                actor.wands[*wand_index].slots[*spell_index].clone()
            }
        }
    }
}

#[derive(Component)]
pub struct Floating {
    pub content: Option<FloatingContent>,
    pub target: Option<FloatingContent>,
}

pub fn spawn_inventory_floating(mut builder: &mut ChildBuilder, registry: &Registry) {
    spawn_item_panel(
        &mut builder,
        &registry,
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
    registry: Registry,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    world: Res<GameWorld>,
    mut se: EventWriter<SEEvent>,
) {
    let mut floating = floating_query.single_mut();
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    let content_optional = floating.content;
    let target_optional = floating.target;

    floating.content = None;
    floating.target = None;

    let Ok(mut actor) = player_query.get_single_mut() else {
        return;
    };

    let Some(content) = content_optional else {
        return;
    };

    se.send(SEEvent::new(CURSOR_8));

    let drop = drop_query.single();
    if drop.hover {
        // アイテムを地面に置きます
        if let Ok(window) = window_query.get_single() {
            if let Some(cursor_in_screen) = window.cursor_position() {
                let (camera, camera_global_transform) = camera_query.single();

                if let Ok(mouse_in_world) =
                    camera.viewport_to_world(camera_global_transform, cursor_in_screen)
                {
                    let pointer_in_world = mouse_in_world.origin.truncate();
                    let tile = world.get_tile_by_coords(pointer_in_world);
                    let props = registry.get_tile(&tile);
                    if props.tile_type != TileType::Wall {
                        if let Some(item) = content.get_inventory_item(&actor) {
                            content.set_item(None, &mut actor);

                            if let Some(level) = world.get_level_by_position(pointer_in_world) {
                                spawn_dropped_item(
                                    &mut commands,
                                    &registry,
                                    &level,
                                    pointer_in_world,
                                    &item,
                                    0,
                                );
                                floating.content = None;

                                se.send(SEEvent::new(PICK_UP));
                            }
                        }
                    }
                }
            }
        }
    } else if let Some(target) = target_optional {
        // アイテムを別のスロットに移動します

        // 移動元のアイテムを取得
        let item_optional_from = content.get_inventory_item(&actor);
        // 移動先のアイテムを取得
        let item_optional_to = target.get_inventory_item(&actor);

        if target.is_settable(&item_optional_from) && content.is_settable(&item_optional_to) {
            // 移動先に書きこみ
            target.set_item(item_optional_from, &mut actor);
            // 移動元に書きこみ
            content.set_item(item_optional_to, &mut actor);
        }
    }
}

impl FloatingContent {
    pub fn get_inventory_item(&self, actor: &Actor) -> Option<Spell> {
        match self {
            FloatingContent::Inventory(i) => actor.inventory.get(*i).clone(),
            FloatingContent::WandSpell(w, i) => actor.get_spell(*w, *i),
        }
    }

    pub fn set_item(&self, item: Option<Spell>, actor: &mut Actor) {
        match (self, item.clone()) {
            (FloatingContent::Inventory(i), _) => {
                actor.inventory.set(*i, item.as_ref().cloned());
            }

            (FloatingContent::WandSpell(w, s), Some(spell_type)) => {
                actor.wands[*w].slots[*s] = Some(spell_type);
                actor.wands[*w].index = 0;
            }
            (FloatingContent::WandSpell(w, s), None) => {
                actor.wands[*w].slots[*s] = None;
                actor.wands[*w].index = 0;
            }
        }
    }

    pub fn is_settable(&self, item: &Option<Spell>) -> bool {
        match (self, item) {
            (FloatingContent::Inventory(_), _) => true,
            (FloatingContent::WandSpell(..), Some(_)) => true,
            (FloatingContent::WandSpell(..), None) => true,
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
