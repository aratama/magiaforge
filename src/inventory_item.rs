use crate::{constant::MAX_ITEMS_IN_INVENTORY, spell::SpellType, wand::WandType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryItem {
    Spell(SpellType),
}

pub type Inventory = [Option<InventoryItem>; MAX_ITEMS_IN_INVENTORY];
