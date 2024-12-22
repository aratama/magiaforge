use super::{
    command_button::{command_button, CommandButton},
    floating::Floating,
    inventory::{spawn_inventory, INVENTORY_IMAGE_HEIGHT},
    menu_left::MenuLeft,
    popup::spawn_spell_information,
};
use crate::{
    asset::GameAssets,
    constant::WAND_EDITOR_Z_INDEX,
    controller::player::Player,
    entity::actor::Actor,
    language::Dict,
    states::{GameMenuState, GameState},
};
use bevy::prelude::*;

#[derive(Component)]
struct WandEditorRoot;

#[derive(Component)]
struct SortButton;

pub const MENU_THEME_COLOR: Color = Color::hsla(63.0, 0.12, 0.5, 0.95);

pub fn spawn_wand_editor(mut builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    let mut editor_builder = builder.spawn((
        WandEditorRoot,
        MenuLeft::new(16.0, -144.0 * 2.0),
        GlobalZIndex(WAND_EDITOR_Z_INDEX),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(100.0),
            width: Val::Px(151.0 * 2.0),
            height: Val::Px(INVENTORY_IMAGE_HEIGHT * 2.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        },
    ));

    editor_builder.with_children(|mut parent| {
        spawn_inventory(&mut parent, &assets);

        command_button(
            parent,
            assets,
            SortButton,
            160.0,
            32.0,
            false,
            Dict {
                ja: "並び替え",
                en: "Sort",
            },
        );
    });

    spawn_spell_information(&mut builder, &assets);
}

fn switch_sort_button_disabled(
    floating_query: Query<&Floating>,
    mut query: Query<&mut CommandButton, With<SortButton>>,
    player_query: Query<&Actor, With<Player>>,
) {
    let floating = floating_query.single();
    if let Ok(mut button) = query.get_single_mut() {
        if floating.content.is_some() {
            button.disabled = true;
            return;
        }
        if let Ok(actor) = player_query.get_single() {
            let mut cloned = actor.inventory.clone();
            cloned.sort();
            button.disabled = cloned == actor.inventory;
        }
    }
}

fn toggle_inventory(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
) {
    match state.get() {
        GameMenuState::Closed => {
            if keys.just_pressed(KeyCode::Tab) {
                next.set(GameMenuState::WandEditOpen);
            }
        }
        GameMenuState::WandEditOpen => {
            if keys.just_pressed(KeyCode::Tab)
                || keys.just_pressed(KeyCode::KeyW)
                || keys.just_pressed(KeyCode::KeyA)
                || keys.just_pressed(KeyCode::KeyS)
                || keys.just_pressed(KeyCode::KeyD)
            {
                next.set(GameMenuState::Closed);
            }
        }
        _ => {}
    }
}

fn sort_button_pressed(
    interaction_query: Query<&Interaction, (With<SortButton>, Changed<Interaction>)>,
    mut player_query: Query<&mut Actor, With<Player>>,
) {
    if let Ok(mut actor) = player_query.get_single_mut() {
        for interaction in interaction_query.iter() {
            match interaction {
                Interaction::Pressed => {
                    actor.inventory.sort();
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
                toggle_inventory,
                sort_button_pressed,
                switch_sort_button_disabled,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
