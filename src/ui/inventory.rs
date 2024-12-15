use crate::entity::dropped_item::spawn_dropped_item;
use crate::ui::floating::{Floating, FloatingContent};
use crate::ui::item_information::{SpellInformation, SpellInformationItem};
use crate::{
    asset::GameAssets, constant::MAX_ITEMS_IN_INVENTORY, controller::player::Player,
    entity::actor::Actor, inventory_item::InventoryItem, states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use std::collections::HashSet;

use super::item_information::SpellInformationRoot;

#[derive(Component)]
pub struct InventoryGrid;

#[derive(Component)]
struct InventoryItemSlot(usize);

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
                    InventoryGrid,
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
                        builder.spawn((
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
            let item = player.inventory.get(slot.0);

            if let Some(item) = item {
                let width = item.get_width();
                aseprite.name = match floating.0 {
                    Some(FloatingContent::Inventory(index)) if index == slot.0 => "empty".into(),
                    _ => item.get_icon().into(),
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
            *visibility = if hidden.contains(&sprite.0) {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
    }
}

fn interaction(
    mut interaction_query: Query<(&InventoryItemSlot, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    mut player_query: Query<(&mut Player, &mut Actor, &Transform)>,
    mut spell_info_query: Query<&mut SpellInformation>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    for (slot, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok((mut player, mut actor, player_position)) = player_query.get_single_mut()
                {
                    let mut floating = floating_query.single_mut();
                    match floating.0 {
                        None => {
                            *floating = Floating(Some(FloatingContent::Inventory(slot.0)));
                        }
                        Some(FloatingContent::Inventory(index)) => {
                            if index == slot.0 {
                                *floating = Floating(None);
                            } else {
                                match (player.inventory.get(index), player.inventory.get(slot.0)) {
                                    (Some(floating_item), None) => {
                                        if player.inventory.is_settable(slot.0, floating_item) {
                                            player.inventory.set(slot.0, Some(floating_item));
                                            player.inventory.set(index, None);
                                            *floating = Floating(None);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Some(FloatingContent::WandSpell(wand_index, spell_index)) => match actor
                            .wands[wand_index]
                        {
                            None => {
                                *floating = Floating(None);
                            }
                            Some(ref mut wand) => {
                                let spell = wand.slots[spell_index];
                                player
                                    .inventory
                                    .set(slot.0, spell.and_then(|s| Some(InventoryItem::Spell(s))));
                                wand.slots[spell_index] = None;
                                actor.wands[wand_index] = Some(wand.clone());
                                *floating = Floating(None);
                            }
                        },
                        Some(FloatingContent::Wand(wand_index)) => {
                            if let Some(ref wand) = actor.wands[wand_index] {
                                let current = player.inventory.get(slot.0);
                                player
                                    .inventory
                                    .set(slot.0, Some(InventoryItem::Wand(wand.wand_type)));

                                // 杖に入っていた呪文はインベントリの空きスロットに入れる
                                let mut not_inserted = Vec::new();
                                for slot in wand.slots {
                                    if let Some(spell) = slot {
                                        if !player.inventory.insert(InventoryItem::Spell(spell)) {
                                            not_inserted.push(spell);
                                        }
                                    }
                                }

                                // インベントリに入らなかった分は床にばらまかれる
                                for spell in not_inserted {
                                    // drop items
                                    spawn_dropped_item(
                                        &mut commands,
                                        &assets,
                                        player_position.translation.truncate(),
                                        InventoryItem::Spell(spell),
                                    );
                                }

                                actor.wands[wand_index] = None;

                                match current {
                                    None => {
                                        *floating = Floating(None);
                                    }
                                    Some(_) => {
                                        *floating =
                                            Floating(Some(FloatingContent::Inventory(slot.0)));
                                    }
                                }
                            }
                        }
                        Some(FloatingContent::Equipment(index)) => {
                            let equipment = player.equipments[index].unwrap();
                            if player
                                .inventory
                                .try_set(slot.0, InventoryItem::Equipment(equipment))
                            {
                                player.equipments[index] = None;
                                *floating = Floating(None);
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                let mut spell_info = spell_info_query.single_mut();
                if let Ok((player, actor, _)) = player_query.get_single() {
                    let floating = floating_query.single_mut();
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
    mut interaction_query: Query<&Interaction, (With<InventoryGrid>, Changed<Interaction>)>,
    mut spell_info_query: Query<&mut Node, With<SpellInformationRoot>>,
) {
    let mut spell = spell_info_query.single_mut();
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => spell.display = Display::Flex,
            Interaction::None => spell.display = Display::None,
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
