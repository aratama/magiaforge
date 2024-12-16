use crate::{
    equipment::{equipment_to_props, EquipmentType},
    language::{Dict, Languages},
    spell::SpellType,
    spell_props::{get_spell_appendix, spell_to_props},
    wand::WandType,
    wand_props::wand_to_props,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryItemType {
    Wand(WandType),
    Spell(SpellType),
    Equipment(EquipmentType),
}

impl InventoryItemType {
    pub fn get_width(&self) -> usize {
        match self {
            InventoryItemType::Spell(_) => 1,
            InventoryItemType::Wand(_) => 2,
            InventoryItemType::Equipment(_) => 1,
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match self {
            InventoryItemType::Spell(spell) => spell_to_props(*spell).icon,
            InventoryItemType::Wand(wand) => wand_to_props(*wand).icon,
            InventoryItemType::Equipment(equipment) => equipment_to_props(*equipment).icon,
        }
    }
}

pub struct InventoryItemProps {
    pub icon: &'static str,
    pub name: Dict,
    pub description: Dict,
}

pub fn inventory_item_to_props(item: InventoryItemType) -> InventoryItemProps {
    match item {
        InventoryItemType::Spell(spell) => {
            let props = spell_to_props(spell);
            InventoryItemProps {
                icon: props.icon,
                name: props.name,
                description: props.description,
            }
        }
        InventoryItemType::Wand(wand) => {
            let props = wand_to_props(wand);
            InventoryItemProps {
                icon: props.icon,
                name: props.name,
                description: props.description,
            }
        }
        InventoryItemType::Equipment(equipment) => {
            let props = equipment_to_props(equipment);
            InventoryItemProps {
                icon: props.icon,
                name: props.name,
                description: props.description,
            }
        }
    }
}

pub fn get_inventory_item_description(item: InventoryItemType, language: Languages) -> String {
    match item {
        InventoryItemType::Spell(spell) => {
            let props = spell_to_props(spell);
            let cast = format!(
                "{}:{}",
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
                cast,
                appendix
            );
        }
        other => inventory_item_to_props(other)
            .description
            .get(language)
            .to_string(),
    }
}
