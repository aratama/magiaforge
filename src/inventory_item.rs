use crate::equipment::EquipmentType;
use crate::language::Dict;
use crate::spell::SpellType;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect, Serialize, Deserialize)]
pub enum InventoryItemType {
    Spell(SpellType),
    Equipment(EquipmentType),
}

impl InventoryItemType {
    pub fn get_width(&self) -> usize {
        match self {
            InventoryItemType::Spell(_) => 1,
            InventoryItemType::Equipment(_) => 1,
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match self {
            InventoryItemType::Spell(spell) => spell.to_props().icon,
            InventoryItemType::Equipment(equipment) => equipment.to_props().icon,
        }
    }

    pub fn get_icon_width(&self) -> f32 {
        match self {
            InventoryItemType::Spell(..) => 32.0,
            InventoryItemType::Equipment(..) => 32.0,
        }
    }
}

pub struct InventoryItemProps {
    pub icon: &'static str,
    pub name: Dict<&'static str>,
    pub description: Dict<&'static str>,
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
                }
            }
            InventoryItemType::Equipment(equipment) => {
                let props = equipment.to_props();
                InventoryItemProps {
                    icon: props.icon,
                    name: props.name,
                    description: props.description,
                }
            }
        }
    }
}
