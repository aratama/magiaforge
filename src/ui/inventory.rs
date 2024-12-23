use crate::asset::GameAssets;
use crate::constant::MAX_ITEMS_IN_INVENTORY;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::states::GameState;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::item_panel::spawn_item_panel;
use crate::ui::item_panel::ItemPanel;
use crate::ui::popup::PopUp;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct InventoryGrid {
    pub hover: bool,
}

#[derive(Component)]
struct InventoryItemSlot(usize);

pub const INVENTORY_IMAGE_HEIGHT: f32 = 168.0;

pub fn spawn_inventory(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((Node {
            width: Val::Px(151.0 * 2.0),
            height: Val::Px(INVENTORY_IMAGE_HEIGHT * 2.0),
            ..default()
        },))
        .with_children(|builder| {
            // 背景画像
            builder.spawn((
                ZIndex(0),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(151.0 * 2.0),
                    height: Val::Px(INVENTORY_IMAGE_HEIGHT * 2.0),
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                AseUiSlice {
                    aseprite: assets.atlas.clone(),
                    name: "inventory".into(),
                },
            ));

            builder
                .spawn((
                    InventoryGrid { hover: false },
                    Interaction::default(),
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(16.0 * 8.0 * 2.0),
                        height: Val::Px(16.0 * 8.0 * 2.0),
                        left: Val::Px(16.0),
                        top: Val::Px(16.0),
                        ..default()
                    },
                ))
                .with_children(|mut builder| {
                    //スロット
                    for i in 0..MAX_ITEMS_IN_INVENTORY {
                        spawn_item_panel(
                            &mut builder,
                            &assets,
                            InventoryItemSlot(i),
                            (i % 8) as f32 * 32.0,
                            (i / 8) as f32 * 32.0,
                            None,
                            None,
                        );
                    }
                });
        });
}

fn update_inventory_slot(
    player_query: Query<&Actor, With<Player>>,
    mut slot_query: Query<(&InventoryItemSlot, &mut ItemPanel)>,
    floating_query: Query<&Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        let floating = floating_query.single();
        for (slot, mut panel) in slot_query.iter_mut() {
            match floating.content {
                Some(FloatingContent::Inventory(i)) if i == slot.0 => {
                    panel.0 = None;
                }
                _ => {
                    panel.0 = actor.inventory.get(slot.0);
                }
            }
        }
    }
}

fn interaction(
    mut interaction_query: Query<(&InventoryItemSlot, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    mut popup_query: Query<&mut PopUp>,
) {
    let mut popup = popup_query.single_mut();
    let mut floating = floating_query.single_mut();
    for (slot, interaction) in &mut interaction_query {
        let content = FloatingContent::Inventory(slot.0);
        match *interaction {
            Interaction::Pressed => match floating.content {
                None => {
                    floating.content = Some(FloatingContent::Inventory(slot.0));
                }
                _ => {}
            },
            Interaction::Hovered => {
                floating.target = Some(content);
                popup.set.insert(content);
                popup.hang = true;
            }
            Interaction::None => {
                popup.set.remove(&content);
            }
        }
    }
}

fn root_interaction(
    mut interaction_query: Query<(&Interaction, &mut InventoryGrid), Changed<Interaction>>,
) {
    for (interaction, mut grid) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                grid.hover = true;
            }
            Interaction::None => {
                grid.hover = false;
            }
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_inventory_slot, interaction, root_interaction)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
