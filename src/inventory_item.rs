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

    pub fn get_icon(&self, constants: &GameConstants) -> String {
        match self {
            InventoryItemType::Spell(spell) => spell.to_props(&constants).icon.clone(),
        }
    }

    pub fn get_icon_width(&self) -> f32 {
        match self {
            InventoryItemType::Spell(..) => 32.0,
        }
    }
}

pub struct InventoryItemProps {
    pub icon: String,
    pub name: Dict<String>,
    pub description: Dict<String>,
}

impl InventoryItemType {
    pub fn to_props(&self, constants: &GameConstants) -> InventoryItemProps {
        match self {
            InventoryItemType::Spell(spell) => {
                let props = spell.to_props(&constants);
                InventoryItemProps {
                    icon: props.icon.clone(),
                    name: props.name.clone(),
                    description: props.description.clone(),
                }
            }
        }
    }
}
