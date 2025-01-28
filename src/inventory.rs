use crate::constant::MAX_ITEMS_IN_INVENTORY;
use crate::spell::Spell;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

/// インベントリ内のアイテムを表します
/// インベントリ内のアイテムは未精算の場合があり、それを表すために使われます
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct InventoryItem {
    pub spell: Spell,

    /// 清算済みの場合は0、未清算の場合は価格が設定されます
    pub price: u32,
}

impl InventoryItem {
    pub fn new(spell: Spell) -> InventoryItem {
        InventoryItem { spell, price: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct Inventory(pub Vec<Option<InventoryItem>>);

impl Inventory {
    pub fn new() -> Inventory {
        Inventory(vec![None; MAX_ITEMS_IN_INVENTORY])
    }

    pub fn from_vec(vec: Vec<Spell>) -> Inventory {
        let mut inventory = Inventory(vec![None; MAX_ITEMS_IN_INVENTORY]);
        for i in 0..MAX_ITEMS_IN_INVENTORY {
            inventory.0[i] = vec.get(i).map(|s| InventoryItem::new(s.clone()));
        }
        inventory
    }

    pub fn get(&self, index: usize) -> &Option<InventoryItem> {
        let Inventory(ref inventory) = *self;
        &inventory[index]
    }

    pub fn set(&mut self, index: usize, item: Option<InventoryItem>) {
        let Inventory(ref mut inventory) = self;
        inventory[index] = item;
    }

    pub fn insert(&mut self, item: InventoryItem) -> bool {
        let Inventory(ref mut inventory) = *self;
        let mut i = 0;
        while i < MAX_ITEMS_IN_INVENTORY {
            match &inventory[i] {
                None => {
                    inventory[i] = Some(item);
                    return true;
                }
                Some(_) => {
                    i += 1;
                }
            }
        }
        return false;
    }

    #[allow(dead_code)]
    pub fn insert_spell(&mut self, item_type: Spell) -> bool {
        self.insert(InventoryItem {
            spell: item_type,
            price: 0,
        })
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
            match (a.as_ref().unwrap(), b.as_ref().unwrap()) {
                (a, b) => a.spell.cmp(&b.spell),
            }
        });
        let mut i = 0;
        for item in cloned {
            if MAX_ITEMS_IN_INVENTORY <= i {
                break;
            }
            let width = 1;
            self.0[i] = item.clone();
            for j in 1..width {
                self.0[i + j] = None;
            }
            i += width;
        }
    }

    // 現在所持している有料呪文の合計金額を返します
    pub fn dept(&self) -> u32 {
        let mut total = 0;
        for ref item in &self.0 {
            if let Some(item) = item {
                total += item.price;
            }
        }
        total
    }
}
