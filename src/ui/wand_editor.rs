use super::{
    command_button::{command_button, CommandButton},
    floating::InventoryItemFloating,
    inventory::spawn_inventory,
    item_information::spawn_spell_information,
};
use crate::{
    asset::GameAssets,
    constant::WAND_EDITOR_Z_INDEX,
    controller::player::Player,
    entity::{
        actor::Actor,
        dropped_item::{spawn_dropped_item, DroppedItemType},
    },
    inventory_item::{sort_inventory, InventoryItem},
    language::Dict,
    set::GameSet,
    states::{GameMenuState, GameState},
    ui::floating::InventoryItemFloatingContent,
};
use bevy::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Component)]
struct WandEditorRoot;

#[derive(Component)]
struct SortButton;

#[derive(Component)]
struct ItemDropButton;

const MENU_THEME_COLOR: Color = Color::hsla(63.0, 0.12, 0.5, 0.95);

pub fn spawn_wand_editor(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            StateScoped(GameState::InGame),
            WandEditorRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: MENU_THEME_COLOR.into(),
                z_index: ZIndex::Global(WAND_EDITOR_Z_INDEX),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_inventory(&mut parent, &assets);

            parent
                .spawn((
                    StateScoped(GameState::InGame),
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    command_button(
                        parent,
                        assets,
                        SortButton,
                        160.0,
                        40.0,
                        false,
                        Dict {
                            ja: "並び替え",
                            en: "Sort",
                        },
                    );

                    command_button(
                        parent,
                        assets,
                        ItemDropButton,
                        160.0,
                        40.0,
                        true,
                        Dict {
                            ja: "置く",
                            en: "Drop",
                        },
                    );
                });
        });

    commands
        .spawn((
            StateScoped(GameState::InGame),
            WandEditorRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(900.0),
                    top: Val::Px(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: MENU_THEME_COLOR.into(),
                z_index: ZIndex::Global(WAND_EDITOR_Z_INDEX),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_spell_information(&mut parent, &assets);
        });
}

fn switch_sort_button_disabled(
    floating_query: Query<&InventoryItemFloating>,
    mut query: Query<&mut CommandButton, With<SortButton>>,
    player_query: Query<&Player>,
) {
    let InventoryItemFloating(floating) = floating_query.single();
    if let Ok(mut button) = query.get_single_mut() {
        if floating.is_some() {
            button.disabled = true;
            return;
        }
        if let Ok(player) = player_query.get_single() {
            let mut cloned = player.inventory.clone();
            sort_inventory(&mut cloned);
            button.disabled = cloned == player.inventory;
        }
    }
}

fn switch_item_drop_button_disabled(
    floating_query: Query<&InventoryItemFloating, Changed<InventoryItemFloating>>,
    mut query: Query<&mut CommandButton, With<ItemDropButton>>,
) {
    for InventoryItemFloating(floating) in floating_query.iter() {
        if let Ok(mut button) = query.get_single_mut() {
            button.disabled = floating.is_none();
        }
    }
}

fn handle_e_key(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        match state.get() {
            GameMenuState::Closed => {
                next.set(GameMenuState::WandEditOpen);
            }
            GameMenuState::WandEditOpen => {
                next.set(GameMenuState::Closed);
            }
            _ => {}
        }
    }
}

fn apply_wand_editor_visible(
    mut query: Query<&mut Visibility, With<WandEditorRoot>>,
    state: Res<State<GameMenuState>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = match state.get() {
            GameMenuState::WandEditOpen => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

fn item_drop_button_pressed(
    interaction_query: Query<&Interaction, (With<ItemDropButton>, Changed<Interaction>)>,
    mut floating_query: Query<&mut InventoryItemFloating>,
    mut player_query: Query<(&mut Player, &mut Actor, &Transform)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    if let Ok((mut player, mut actor, transform)) = player_query.get_single_mut() {
        for interaction in interaction_query.iter() {
            match interaction {
                Interaction::Pressed => {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        Some(InventoryItemFloatingContent::InventoryItem(index)) => {
                            if let Some(InventoryItem::Spell(spell)) = player.inventory[index] {
                                spawn_dropped_item(
                                    &mut commands,
                                    &assets,
                                    transform.translation.x,
                                    transform.translation.y,
                                    DroppedItemType::Spell(spell),
                                );
                                player.inventory[index] = None;
                                *floating = InventoryItemFloating(None);
                            }
                        }
                        Some(InventoryItemFloatingContent::Wand(index)) => {
                            if let Some(ref wand) = actor.wands[index] {
                                for slot in wand.slots {
                                    if let Some(spell) = slot {
                                        spawn_dropped_item(
                                            &mut commands,
                                            &assets,
                                            transform.translation.x,
                                            transform.translation.y,
                                            DroppedItemType::Spell(spell),
                                        );
                                    }
                                }
                                spawn_dropped_item(
                                    &mut commands,
                                    &assets,
                                    transform.translation.x,
                                    transform.translation.y,
                                    DroppedItemType::Wand(wand.wand_type),
                                );
                                actor.wands[index] = None;
                                *floating = InventoryItemFloating(None);
                            }
                        }
                        Some(InventoryItemFloatingContent::WandSpell {
                            wand_index,
                            spell_index,
                        }) => {
                            if let Some(ref mut wand) = actor.wands[wand_index] {
                                if let Some(spell) = wand.slots[spell_index] {
                                    spawn_dropped_item(
                                        &mut commands,
                                        &assets,
                                        transform.translation.x,
                                        transform.translation.y,
                                        DroppedItemType::Spell(spell),
                                    );
                                    wand.slots[spell_index] = None;
                                    *floating = InventoryItemFloating(None);
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn sort_button_pressed(
    interaction_query: Query<&Interaction, (With<SortButton>, Changed<Interaction>)>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for interaction in interaction_query.iter() {
            match interaction {
                Interaction::Pressed => {
                    sort_inventory(&mut player.inventory);
                }
                _ => {}
            }
        }
    }
}

pub struct WandEditorPlugin;

impl Plugin for WandEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_e_key,
                apply_wand_editor_visible,
                switch_item_drop_button_disabled,
                sort_button_pressed,
                switch_sort_button_disabled,
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            item_drop_button_pressed
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
