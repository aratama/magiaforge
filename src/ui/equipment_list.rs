use crate::asset::GameAssets;
use crate::constant::MAX_SPELLS_IN_WAND;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::item_panel::spawn_item_panel;
use crate::ui::item_panel::ItemPanel;
use crate::ui::popup::PopUp;
use crate::ui::popup::PopupContent;
use bevy::prelude::*;
use bevy::ui::Display;

#[derive(Component)]
pub struct EquipmentContainer;

#[derive(Component, Debug, Clone)]
struct EquipmentSprite {
    index: usize,
}

pub fn spawn_equipment_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            EquipmentContainer,
            BorderColor(Color::hsla(0.0, 0.0, 1.0, 0.0)),
            Node {
                width: Val::Px(64. + 32. * MAX_SPELLS_IN_WAND as f32),
                height: Val::Px(32.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            for index in 0..MAX_SPELLS_IN_WAND {
                spawn_item_panel(
                    &mut parent,
                    &assets,
                    EquipmentSprite { index },
                    32.0 + 32. * (index as f32),
                    0.0,
                    None,
                    Some(BackgroundColor(slot_color(5, index))),
                );
            }
        });
}

fn update_equipment_sprite(
    mut sprite_query: Query<(&EquipmentSprite, &mut ItemPanel)>,
    player_query: Query<&Actor, With<Player>>,
    floating_query: Query<&mut Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        let float = floating_query.single();
        for (sprite, mut panel) in sprite_query.iter_mut() {
            panel.0 = match float.content {
                Some(FloatingContent::Equipment(index)) if index == sprite.index => None,
                _ => actor.equipments[sprite.index].map(|e| InventoryItem {
                    item_type: InventoryItemType::Equipment(e.equipment_type),
                    price: e.price,
                }),
            };
        }
    }
}

fn interact(
    sprite_query: Query<(&EquipmentSprite, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
    mut popup_query: Query<&mut PopUp>,
) {
    let mut floating = floating_query.single_mut();
    let mut popup = popup_query.single_mut();
    if *state == GameMenuState::WandEditOpen {
        for (sprite, interaction) in sprite_query.iter() {
            let content = FloatingContent::Equipment(sprite.index);
            match interaction {
                Interaction::Pressed => {
                    floating.content = Some(content);
                }
                Interaction::Hovered => {
                    floating.target = Some(content);
                    popup.set.insert(PopupContent::FloatingContent(content));
                    popup.anchor_left = true;
                    popup.anchor_top = false;
                }
                Interaction::None => {
                    popup.set.remove(&PopupContent::FloatingContent(content));
                }
            }
        }
    }
}

fn slot_color(wand_index: usize, spell_index: usize) -> Color {
    return Color::hsla(
        120.0,
        0.3,
        0.4,
        if (wand_index + spell_index) % 2 == 0 {
            0.1
        } else {
            0.12
        },
    );
}

pub struct EquipmentListPlugin;

impl Plugin for EquipmentListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_equipment_sprite, interact).run_if(in_state(GameState::InGame)),
        );
    }
}
