use crate::ui::floating::{Floating, FloatingContent};
use crate::ui::item_information::{SpellInformation, SpellInformationItem};
use crate::{
    asset::GameAssets, constant::MAX_ITEMS_IN_INVENTORY, controller::player::Player,
    entity::actor::Actor, states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct InventoryGrid {
    pub hover: bool,
}

#[derive(Component)]
struct InventoryItemSlot(usize);

#[derive(Component)]
struct YellowFrame;

pub fn spawn_inventory(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((Node {
            width: Val::Px(151.0 * 2.0),
            height: Val::Px(160.0 * 2.0),
            ..default()
        },))
        .with_children(|builder| {
            // 背景画像
            builder.spawn((
                ZIndex(0),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(151.0 * 2.0),
                    height: Val::Px(160.0 * 2.0),
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
                .with_children(|builder| {
                    //スロット
                    for i in 0..MAX_ITEMS_IN_INVENTORY {
                        builder
                            .spawn((
                                InventoryItemSlot(i),
                                Interaction::default(),
                                ZIndex(1),
                                Node {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                    // gird layoutだと、兄弟要素の大きさに左右されてレイアウトが崩れてしまう場合があるので、
                                    // Absoluteでずれないようにする
                                    position_type: PositionType::Absolute,
                                    left: Val::Px((i % 8) as f32 * 32.0),
                                    top: Val::Px((i / 8) as f32 * 32.0),
                                    ..default()
                                },
                                AseUiSlice {
                                    aseprite: assets.atlas.clone(),
                                    name: "empty".into(),
                                },
                            ))
                            .with_child((
                                YellowFrame,
                                AseUiSlice {
                                    aseprite: assets.atlas.clone(),
                                    name: "empty".into(),
                                },
                                ZIndex(-1),
                            ));
                    }
                });
        });
}

fn update_inventory_slot(
    query: Query<&Player>,
    mut slot_query: Query<(
        &InventoryItemSlot,
        &mut AseUiSlice,
        &mut Node,
        &mut Visibility,
    )>,
    floating_query: Query<&Floating>,
) {
    if let Ok(player) = query.get_single() {
        let floating = floating_query.single();

        let mut hidden: HashSet<usize> = HashSet::new();

        for (slot, mut aseprite, mut style, mut visibility) in slot_query.iter_mut() {
            let item_optional = player.inventory.get(slot.0);

            if let Some(item) = item_optional {
                let width = item.item_type.get_width();
                aseprite.name = match floating.content {
                    Some(FloatingContent::Inventory(index)) if index == slot.0 => "empty".into(),
                    _ => item.item_type.get_icon().into(),
                };
                style.width = Val::Px(32.0 * width as f32);
                *visibility = Visibility::Inherited;

                for d in 1..width {
                    hidden.insert(slot.0 + d);
                }
            } else {
                style.width = Val::Px(32.0);
                aseprite.name = "empty".into();
            }
        }

        for (sprite, _, _, mut visibility) in slot_query.iter_mut() {
            if hidden.contains(&sprite.0) {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
            }
        }
    }
}

fn update_yellow_frame(
    query: Query<&Player>,
    slot_query: Query<&InventoryItemSlot>,
    mut children_query: Query<(&Parent, &mut AseUiSlice), With<YellowFrame>>,
) {
    if let Ok(player) = query.get_single() {
        for (parent, mut aseprite) in children_query.iter_mut() {
            let slot = slot_query.get(parent.get()).unwrap();
            let item_optional = player.inventory.get(slot.0);
            match item_optional {
                Some(item) if 0 < item.price => {
                    aseprite.name = "yellow_frame".into();
                }
                _ => {
                    aseprite.name = "empty".into();
                }
            }
        }
    }
}

fn interaction(
    mut interaction_query: Query<(&InventoryItemSlot, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    player_query: Query<(&Player, &Actor, &Transform)>,
    mut spell_info_query: Query<&mut SpellInformation>,
) {
    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let mut floating = floating_query.single_mut();
                match floating.content {
                    None => {
                        floating.content = Some(FloatingContent::Inventory(slot.0));
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                let mut floating = floating_query.single_mut();
                floating.target = Some(FloatingContent::Inventory(slot.0));

                let mut spell_info = spell_info_query.single_mut();
                if let Ok((player, actor, _)) = player_query.get_single() {
                    let floating_item = floating.get_item(&player, &actor);
                    if player.inventory.is_settable_optional(slot.0, floating_item) {
                        *spell_info = match player.inventory.get(slot.0) {
                            Some(slot_item) => SpellInformation(Some(
                                SpellInformationItem::InventoryItem(slot_item),
                            )),
                            None => SpellInformation(None),
                        };
                    } else {
                        *spell_info = SpellInformation(None);
                    }
                } else {
                    *spell_info = SpellInformation(None);
                }
            }
            Interaction::None => {}
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
            (
                update_inventory_slot,
                interaction,
                root_interaction,
                update_yellow_frame,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
