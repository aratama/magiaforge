use crate::registry::Registry;
use crate::spell::Spell;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

/// TODO 枝が一個だけになって今は不要なので削除する？
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect, Serialize, Deserialize)]
pub enum InventoryItemType {
    Spell(Spell),
}

impl InventoryItemType {
    pub fn get_width(&self) -> usize {
        match self {
            InventoryItemType::Spell(_) => 1,
        }
    }

    pub fn get_icon(&self, registry: &Registry) -> String {
        match self {
            InventoryItemType::Spell(spell) => registry.get_spell_props(spell).icon.clone(),
        }
    }

    pub fn get_icon_width(&self) -> f32 {
        match self {
            InventoryItemType::Spell(..) => 32.0,
        }
    }
}
