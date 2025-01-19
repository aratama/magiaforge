use crate::constant::GameConstants;
use crate::language::Dict;
use crate::spell::SpellType;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect, Serialize, Deserialize)]
pub enum InventoryItemType {
    Spell(SpellType),
}

impl InventoryItemType {
    pub fn get_width(&self) -> usize {
        match self {
            InventoryItemType::Spell(_) => 1,
        }
    }

    pub fn get_icon(&self, constants: &GameConstants) -> &'static str {
        match self {
            InventoryItemType::Spell(spell) => spell.to_props(&constants).icon,
        }
    }

    pub fn get_icon_width(&self) -> f32 {
        match self {
            InventoryItemType::Spell(..) => 32.0,
        }
    }
}

pub struct InventoryItemProps {
    pub icon: &'static str,
    pub name: Dict<&'static str>,
    pub description: Dict<&'static str>,
}

impl InventoryItemType {
    pub fn to_props(&self, constants: &GameConstants) -> InventoryItemProps {
        match self {
            InventoryItemType::Spell(spell) => {
                let props = spell.to_props(&constants);
                InventoryItemProps {
                    icon: props.icon,
                    name: props.name,
                    description: props.description,
                }
            }
        }
    }
}
