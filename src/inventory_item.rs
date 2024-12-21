use crate::{
    equipment::EquipmentType,
    language::{Dict, Languages},
    spell::{get_spell_appendix, SpellType},
    wand::WandType,
};
use bevy::reflect::Reflect;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect)]
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
            InventoryItemType::Spell(spell) => spell.to_props().icon,
            InventoryItemType::Wand(wand) => wand.to_props().icon,
            InventoryItemType::Equipment(equipment) => equipment.to_props().icon,
        }
    }

    pub fn get_icon_width(&self) -> f32 {
        match self {
            InventoryItemType::Spell(..) => 32.0,
            InventoryItemType::Wand(..) => 64.0,
            InventoryItemType::Equipment(..) => 32.0,
        }
    }
}

pub struct InventoryItemProps {
    pub icon: &'static str,
    pub name: Dict<&'static str>,
    pub description: Dict<&'static str>,
    pub appendix: Dict<&'static str>,
}

impl InventoryItemType {
    pub fn to_props(&self) -> InventoryItemProps {
        match self {
            InventoryItemType::Spell(spell) => {
                let props = spell.to_props();
                InventoryItemProps {
                    icon: props.icon,
                    name: props.name,
                    description: props.description,
                    appendix: Dict::empty(),
                }
            }
            InventoryItemType::Wand(wand) => {
                let props = wand.to_props();
                InventoryItemProps {
                    icon: props.icon,
                    name: props.name,
                    description: props.description,
                    appendix: Dict::empty(),
                }
            }
            InventoryItemType::Equipment(equipment) => {
                let props = equipment.to_props();
                InventoryItemProps {
                    icon: props.icon,
                    name: props.name,
                    description: props.description,
                    appendix: Dict::empty(),
                }
            }
        }
    }
}

pub fn get_inventory_item_description(item: InventoryItemType, language: Languages) -> String {
    match item {
        InventoryItemType::Spell(spell) => {
            let props = spell.to_props();
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
        other => other.to_props().description.get(language).to_string(),
    }
}
