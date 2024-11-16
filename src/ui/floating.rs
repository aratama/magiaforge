use crate::{
    asset::GameAssets,
    constant::WAND_EDITOR_FLOATING_Z_INDEX,
    controller::player::Player,
    entity::actor::Actor,
    equipment::{self, equipment_to_props},
    inventory_item::{inventory_item_to_props, InventoryItem},
    spell_props::spell_to_props,
    states::{GameMenuState, GameState},
    wand_props::wand_to_props,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::{AsepriteSlice, AsepriteSliceUiBundle};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FloatingContent {
    Inventory(usize),
    WandSpell(usize, usize),
    Wand(usize),
    Equipment(usize),
}

#[derive(Component)]
pub struct Floating(pub Option<FloatingContent>);

impl Floating {
    pub fn get_item(&self, player: &Player, actor: &Actor) -> Option<InventoryItem> {
        match self.0 {
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

pub fn spawn_inventory_floating(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands.spawn((
        Floating(None),
        StateScoped(GameState::InGame),
        Interaction::default(),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(500.0),
                left: Val::Px(500.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                // border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            z_index: ZIndex::Global(WAND_EDITOR_FLOATING_Z_INDEX),
            // background_color: Color::hsla(0.0, 0.0, 0.0, 0.5).into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "empty".into(),
            ..default()
        },
    ));
}

fn update_inventory_floaing(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Style, &mut Floating)>,
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
        *floating = Floating(None);
    }
}

fn switch_floating_visibility(mut query: Query<(&Floating, &mut Visibility)>) {
    let (floating, mut visibility) = query.single_mut();
    *visibility = match floating.0 {
        Some(_) => Visibility::Visible,
        None => Visibility::Hidden,
    };
}

fn switch_floating_slice(
    player_query: Query<(&Player, &Actor)>,
    mut floating_query: Query<(&Floating, &mut AsepriteSlice, &mut Style), With<Floating>>,
) {
    if let Ok((player, actor)) = player_query.get_single() {
        let (floating, mut floating_slice, mut style) = floating_query.single_mut();
        match floating.0 {
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
                    *floating_slice = slice.into();
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
                            *floating_slice = props.icon.into();
                            style.width = Val::Px(32.0);
                        }
                        _ => {
                            *floating_slice = "empty".into();
                        }
                    },
                    None => {
                        *floating_slice = "empty".into();
                    }
                }
            }
            Some(FloatingContent::Wand(wand_index)) => match &actor.wands[wand_index] {
                Some(wand) => {
                    let props = wand_to_props(wand.wand_type);
                    *floating_slice = props.icon.into();
                    style.width = Val::Px(64.0);
                }
                None => {
                    *floating_slice = "empty".into();
                }
            },
            Some(FloatingContent::Equipment(equipment)) => match player.equipments[equipment] {
                Some(equipment) => {
                    let props = equipment_to_props(equipment);
                    *floating_slice = props.icon.into();
                    style.width = Val::Px(32.0);
                }
                None => {
                    *floating_slice = "empty".into();
                }
            },
            None => {
                *floating_slice = "empty".into();
            }
        }
    }
}

fn cancel_on_close(state: Res<State<GameMenuState>>, mut floating_query: Query<&mut Floating>) {
    if state.is_changed() {
        if *state.get() == GameMenuState::Closed {
            let mut floating = floating_query.single_mut();
            *floating = Floating(None);
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
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
