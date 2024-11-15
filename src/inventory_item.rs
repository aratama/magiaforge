use crate::{constant::MAX_ITEMS_IN_INVENTORY, spell::SpellType};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryItem {
    Spell(SpellType),
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
            (InventoryItem::Spell(a), InventoryItem::Spell(b)) => a.cmp(&b),
        }
    });
}
