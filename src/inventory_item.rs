use bevy::prelude::*;

use crate::{
    asset::GameAssets,
    constant::MAX_ITEMS_IN_INVENTORY,
    entity::dropped_item::{spawn_dropped_item, DroppedItemType},
    language::{Dict, Languages},
    spell::SpellType,
    spell_props::{get_spell_appendix, spell_to_props},
    wand::WandType,
    wand_props::wand_to_props,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryItem {
    Spell(SpellType),
    Wand(WandType),
    Lantern,
}

pub fn spawn_inventory_item(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    item: InventoryItem,
) {
    match item {
        InventoryItem::Spell(spell) => {
            spawn_dropped_item(
                &mut commands,
                &assets,
                position.x,
                position.y,
                DroppedItemType::Spell(spell),
            );
        }
        InventoryItem::Lantern => {
            spawn_dropped_item(
                &mut commands,
                &assets,
                position.x,
                position.y,
                DroppedItemType::Lantern,
            );
        }
        _ => {}
    }
}

pub struct InventoryItemProps {
    pub icon: &'static str,
    pub name: Dict,
    pub description: Dict,
}

pub fn inventory_item_to_props(item: InventoryItem) -> InventoryItemProps {
    match item {
        InventoryItem::Spell(spell) => {
            let props = spell_to_props(spell);
            InventoryItemProps {
                icon: props.icon,
                name: props.name,
                description: props.description,
            }
        }
        InventoryItem::Wand(wand) => {
            let props = wand_to_props(wand);
            InventoryItemProps {
                icon: props.icon,
                name: props.name,
                description: props.description,
            }
        }
        InventoryItem::Lantern => InventoryItemProps {
            icon: "lantern",
            name: Dict {
                ja: "ランタン",
                en: "Lantern",
            },
            description: Dict {
                ja: "暗闇を照らすランタン",
                en: "A lantern that illuminates the darkness",
            },
        },
    }
}

pub fn get_inventory_item_description(item: InventoryItem, language: Languages) -> String {
    match item {
        InventoryItem::Spell(spell) => {
            let props = spell_to_props(spell);
            let mana = format!(
                "{}: {}  {}:{}",
                Dict {
                    ja: "マナ消費",
                    en: "Mana Drain"
                }
                .get(language),
                props.mana_drain,
                Dict {
                    ja: "詠唱遅延",
                    en: "Cast Delay"
                }
                .get(language),
                props.cast_delay
            );
            let appendix = get_spell_appendix(props.cast, language);
            return format!(
                "{}\n{}\n{}",
                props.description.get(language),
                mana,
                appendix
            );
        }
        other => inventory_item_to_props(other)
            .description
            .get(language)
            .to_string(),
    }
}

pub type Inventory = [Option<InventoryItem>; MAX_ITEMS_IN_INVENTORY];

pub fn sort_inventory(inventory: &mut Inventory) {
    inventory.sort_by(|a, b| {
        if a.is_none() {
            return std::cmp::Ordering::Greater;
        }
        if b.is_none() {
            return std::cmp::Ordering::Less;
        }
        match (a.unwrap(), b.unwrap()) {
            (a, b) => a.cmp(&b),
        }
    });
}
