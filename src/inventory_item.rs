use crate::{constant::MAX_ITEMS_IN_INVENTORY, spell::SpellType, wand::WandType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryItem {
    Wand(WandType),
    Spell(SpellType),
}

pub type Inventory = [Option<InventoryItem>; MAX_ITEMS_IN_INVENTORY];
