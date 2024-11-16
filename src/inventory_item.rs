use crate::{
    asset::GameAssets,
    entity::dropped_item::spawn_dropped_item,
    equipment::{equipment_to_props, Equipment},
    language::{Dict, Languages},
    spell::SpellType,
    spell_props::{get_spell_appendix, spell_to_props},
    wand::WandType,
    wand_props::wand_to_props,
};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryItem {
    Wand(WandType),
    Spell(SpellType),
    Equipment(Equipment),
}

impl InventoryItem {
    pub fn get_width(&self) -> usize {
        match self {
            InventoryItem::Spell(_) => 1,
            InventoryItem::Wand(_) => 2,
            InventoryItem::Equipment(_) => 1,
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match self {
            InventoryItem::Spell(spell) => spell_to_props(*spell).icon,
            InventoryItem::Wand(wand) => wand_to_props(*wand).icon,
            InventoryItem::Equipment(equipment) => equipment_to_props(*equipment).icon,
        }
    }
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
                InventoryItem::Spell(spell),
            );
        }
        InventoryItem::Wand(wand_type) => {
            spawn_dropped_item(
                &mut commands,
                &assets,
                position.x,
                position.y,
                InventoryItem::Wand(wand_type),
            );
        }
        InventoryItem::Equipment(equipment_type) => {
            spawn_dropped_item(
                &mut commands,
                &assets,
                position.x,
                position.y,
                InventoryItem::Equipment(equipment_type),
            );
        }
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
        InventoryItem::Equipment(equipment) => {
            let props = equipment_to_props(equipment);
            InventoryItemProps {
                icon: props.icon,
                name: props.name,
                description: props.description,
            }
        }
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
