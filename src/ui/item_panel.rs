use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::{asset::GameAssets, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct ItemPanel(pub Option<InventoryItem>);

#[derive(Component)]
struct ItemFrame;

#[derive(Component)]
struct ChargeAlert;

pub fn spawn_item_panel<T: Component>(
    builder: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    marker: T,
    x: f32,
    y: f32,
    globa_z_index: Option<GlobalZIndex>,
    background: Option<BackgroundColor>,
) {
    let mut b = builder.spawn((
        marker,
        ItemPanel(None),
        Interaction::default(),
        ZIndex(1),
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            // gird layoutだと、兄弟要素の大きさに左右されてレイアウトが崩れてしまう場合があるので、
            // Absoluteでずれないようにする
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "empty".into(),
        },
    ));

    if let Some(globa_z_index) = globa_z_index {
        b.insert(globa_z_index);
    }

    if let Some(background) = background {
        b.insert(background);
    }

    b.with_children(|builder| {
        builder.spawn((
            ItemFrame,
            AseUiSlice {
                aseprite: assets.atlas.clone(),
                name: "spell_frame".into(),
            },
            ZIndex(1),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
        ));
        builder.spawn((
            ChargeAlert,
            AseUiSlice {
                aseprite: assets.atlas.clone(),
                name: "charge_alert".into(),
            },
            ZIndex(-1),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
        ));
    });
}

fn update_inventory_slot(mut slot_query: Query<(&ItemPanel, &mut AseUiSlice)>) {
    for (slot, mut aseprite) in slot_query.iter_mut() {
        if let Some(item) = slot.0 {
            aseprite.name = item.item_type.get_icon().into();
        } else {
            aseprite.name = "empty".into();
        }
    }
}

fn update_panel_width(
    mut frame_query: Query<(&Parent, &mut Node), With<ItemFrame>>,
    mut slot_query: Query<(&ItemPanel, &mut Node), Without<ItemFrame>>,
) {
    for (parent, mut node) in frame_query.iter_mut() {
        let (slot, mut aseprite) = slot_query.get_mut(parent.get()).unwrap();
        if let Some(item) = slot.0 {
            let width = Val::Px(item.item_type.get_icon_width());
            node.width = width;
            aseprite.width = width;
        } else {
            node.width = Val::Px(32.0);
            node.height = Val::Px(32.0);
        }
    }
}

fn update_item_frame(
    slot_query: Query<&ItemPanel>,
    mut children_query: Query<(&Parent, &mut AseUiSlice), With<ItemFrame>>,
) {
    for (parent, mut aseprite) in children_query.iter_mut() {
        let item_optional = slot_query.get(parent.get()).unwrap();
        match item_optional.0 {
            Some(InventoryItem {
                item_type: InventoryItemType::Spell(..),
                ..
            }) => {
                aseprite.name = "spell_frame".into();
            }
            Some(InventoryItem {
                item_type: InventoryItemType::Equipment(..),
                ..
            }) => {
                aseprite.name = "equipment_frame".into();
            }
            Some(InventoryItem {
                item_type: InventoryItemType::Wand(..),
                ..
            }) => {
                aseprite.name = "wand_frame".into();
            }
            _ => {
                aseprite.name = "empty".into();
            }
        }
    }
}

fn update_charge_alert(
    slot_query: Query<&ItemPanel>,
    mut children_query: Query<(&Parent, &mut Node), With<ChargeAlert>>,
) {
    for (parent, mut aseprite) in children_query.iter_mut() {
        let slot = slot_query.get(parent.get()).unwrap();
        match slot.0 {
            Some(item) if 0 < item.price => {
                aseprite.display = Display::default();
            }
            _ => {
                aseprite.display = Display::None;
            }
        }
    }
}

pub struct ItemPanelPlugin;

impl Plugin for ItemPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_inventory_slot,
                update_charge_alert,
                update_item_frame,
                update_panel_width,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
