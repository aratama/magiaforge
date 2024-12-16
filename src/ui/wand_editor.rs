use super::{
    command_button::{command_button, CommandButton},
    floating::Floating,
    inventory::spawn_inventory,
    item_information::{spawn_spell_information, SpellInformationRoot},
    menu_left::MenuLeft,
};
use crate::{
    asset::GameAssets,
    constant::{TILE_SIZE, WAND_EDITOR_Z_INDEX},
    controller::player::Player,
    entity::{actor::Actor, dropped_item::spawn_dropped_item},
    inventory_item::{spawn_inventory_item, InventoryItem},
    language::Dict,
    set::GameSet,
    states::{GameMenuState, GameState},
    ui::floating::FloatingContent,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Component)]
struct WandEditorRoot;

#[derive(Component)]
struct SortButton;

const MENU_THEME_COLOR: Color = Color::hsla(63.0, 0.12, 0.5, 0.95);

pub fn spawn_wand_editor(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            WandEditorRoot,
            MenuLeft::new(16.0, -144.0 * 2.0),
            GlobalZIndex(WAND_EDITOR_Z_INDEX),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(100.0),
                width: Val::Px(151.0 * 2.0),
                height: Val::Px(160.0 * 2.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_inventory(&mut parent, &assets);

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
        });

    builder
        .spawn((
            SpellInformationRoot,
            BackgroundColor(MENU_THEME_COLOR),
            GlobalZIndex(WAND_EDITOR_Z_INDEX),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                display: Display::None,
                // display: Display::Flex,
                //
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_spell_information(&mut parent, &assets);
        });
}

fn switch_sort_button_disabled(
    floating_query: Query<&Floating>,
    mut query: Query<&mut CommandButton, With<SortButton>>,
    player_query: Query<&Player>,
) {
    let Floating(floating) = floating_query.single();
    if let Ok(mut button) = query.get_single_mut() {
        if floating.is_some() {
            button.disabled = true;
            return;
        }
        if let Ok(player) = player_query.get_single() {
            let mut cloned = player.inventory.clone();
            cloned.sort();
            button.disabled = cloned == player.inventory;
        }
    }
}

fn handle_tab_key(
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

fn drop_item(
    mut floating_query: Query<&mut Floating>,
    mut player_query: Query<(&mut Player, &mut Actor, &Transform)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok((mut player, mut actor, transform)) = player_query.get_single_mut() {
            if let Ok(window) = q_window.get_single() {
                if let Some(cursor_in_screen) = window.cursor_position() {
                    if let Ok((camera, camera_global_transform)) = camera_query.get_single() {
                        if let Ok(mouse_in_world) =
                            camera.viewport_to_world(camera_global_transform, cursor_in_screen)
                        {
                            let player_position = transform.translation.truncate();
                            let pointer_in_world = mouse_in_world.origin.truncate();
                            let vector = pointer_in_world - player_position;
                            let angle = vector.to_angle();
                            let length = vector.length();

                            if TILE_SIZE * 3.0 < length {
                                return;
                            }

                            let dest = player_position + Vec2::from_angle(angle) * length;

                            let mut floating = floating_query.single_mut();
                            match floating.0 {
                                Some(FloatingContent::Inventory(index)) => {
                                    let item = player.inventory.get(index).unwrap();
                                    spawn_inventory_item(&mut commands, &assets, dest, item);
                                    player.inventory.set(index, None);
                                    *floating = Floating(None);
                                }
                                Some(FloatingContent::Wand(index)) => {
                                    if let Some(ref wand) = actor.wands[index] {
                                        for slot in wand.slots {
                                            if let Some(spell) = slot {
                                                spawn_dropped_item(
                                                    &mut commands,
                                                    &assets,
                                                    dest,
                                                    InventoryItem::Spell(spell),
                                                );
                                            }
                                        }
                                        spawn_dropped_item(
                                            &mut commands,
                                            &assets,
                                            dest,
                                            InventoryItem::Wand(wand.wand_type),
                                        );
                                        actor.wands[index] = None;
                                        *floating = Floating(None);
                                    }
                                }
                                Some(FloatingContent::WandSpell(wand_index, spell_index)) => {
                                    if let Some(ref mut wand) = actor.wands[wand_index] {
                                        if let Some(spell) = wand.slots[spell_index] {
                                            spawn_dropped_item(
                                                &mut commands,
                                                &assets,
                                                pointer_in_world,
                                                InventoryItem::Spell(spell),
                                            );
                                            wand.slots[spell_index] = None;
                                            *floating = Floating(None);
                                        }
                                    }
                                }
                                Some(FloatingContent::Equipment(index)) => {
                                    let item = player.equipments[index].unwrap();
                                    spawn_inventory_item(
                                        &mut commands,
                                        &assets,
                                        dest,
                                        InventoryItem::Equipment(item),
                                    );
                                    player.equipments[index] = None;
                                    *floating = Floating(None);
                                }
                                None => {}
                            }
                        }
                    }
                }
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
                    player.inventory.sort();
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
                handle_tab_key,
                sort_button_pressed,
                switch_sort_button_disabled,
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            drop_item
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
