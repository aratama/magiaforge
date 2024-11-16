use crate::{
    constant::{MAX_ITEMS_IN_INVENTORY, MAX_ITEMS_IN_INVENTORY_COLUMN, MAX_ITEMS_IN_INVENTORY_ROW},
    inventory_item::InventoryItem,
    wand::Wand,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct Inventory([Option<InventoryItem>; MAX_ITEMS_IN_INVENTORY]);

impl Inventory {
    pub fn new() -> Inventory {
        Inventory([None; MAX_ITEMS_IN_INVENTORY])
    }

    pub fn get(&self, index: usize) -> Option<InventoryItem> {
        let Inventory(inventory) = *self;
        return inventory[index];
    }

    pub fn set(&mut self, index: usize, item: Option<InventoryItem>) {
        let Inventory(ref mut inventory) = self;
        inventory[index] = item;
    }

    pub fn try_set(&mut self, index: usize, item: InventoryItem) -> bool {
        if self.is_settable(index, item) {
            self.set(index, Some(item));
            return true;
        }
        return false;
    }

    pub fn is_settable(&self, index: usize, item: InventoryItem) -> bool {
        let x = index % MAX_ITEMS_IN_INVENTORY_ROW;
        let y = index / MAX_ITEMS_IN_INVENTORY_ROW;
        if MAX_ITEMS_IN_INVENTORY_COLUMN <= y {
            return false;
        }
        for i in 0..item.get_width() {
            if MAX_ITEMS_IN_INVENTORY_ROW <= x + i {
                return false;
            }
            if self.0[index + i].is_some() {
                return false;
            }
        }
        return true;
    }

    pub fn is_settable_optional(&self, index: usize, item: Option<InventoryItem>) -> bool {
        if let Some(item) = item {
            return self.is_settable(index, item);
        }
        return true;
    }

    pub fn insert(&mut self, item: InventoryItem) -> bool {
        let Inventory(ref mut inventory) = *self;
        let mut i = 0;
        while i < MAX_ITEMS_IN_INVENTORY {
            match inventory[i] {
                None => {
                    inventory[i] = Some(item);
                    return true;
                }
                Some(item) => {
                    i += item.get_width();
                }
            }
        }
        return false;
    }

    pub fn insert_wand(&mut self, wand: &Wand) -> Vec<InventoryItem> {
        let mut items = Vec::new();
        items.push(InventoryItem::Wand(wand.wand_type));
        for spell in wand.slots.iter() {
            if let Some(spell) = spell {
                items.push(InventoryItem::Spell(*spell));
            }
        }
        let mut not_inserted = Vec::new();
        for item in items.iter() {
            if !self.insert(*item) {
                not_inserted.push(*item);
            }
        }
        return not_inserted;
    }

    pub fn sort(&mut self) {
        let mut cloned = self.0.clone();
        cloned.sort_by(|a, b| {
            if a.is_none() {
                return std::cmp::Ordering::Greater;
            }
            if b.is_none() {
                return std::cmp::Ordering::Less;
            }
            match (a.unwrap(), b.unwrap()) {
                (a, b) => a.cmp(&b),
            }
        });
        let mut i = 0;
        for item in cloned {
            if MAX_ITEMS_IN_INVENTORY <= i {
                break;
            }
            let width = match item {
                Some(item) => item.get_width(),
                None => 1,
            };
            self.0[i] = item;
            for j in 1..width {
                self.0[i + j] = None;
            }
            i += width;
        }
    }
}
