use crate::constant::MAX_ITEMS_IN_INVENTORY;
use crate::spell::Spell;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct Inventory(pub Vec<Option<Spell>>);

impl Inventory {
    pub fn new() -> Inventory {
        Inventory(vec![None; MAX_ITEMS_IN_INVENTORY])
    }

    pub fn from_vec(vec: Vec<Spell>) -> Inventory {
        let mut inventory = Inventory(vec![None; MAX_ITEMS_IN_INVENTORY]);
        for i in 0..MAX_ITEMS_IN_INVENTORY {
            inventory.0[i] = vec.get(i).cloned();
        }
        inventory
    }

    pub fn get(&self, index: usize) -> &Option<Spell> {
        let Inventory(ref inventory) = *self;
        &inventory[index]
    }

    pub fn set(&mut self, index: usize, item: Option<Spell>) {
        let Inventory(ref mut inventory) = self;
        inventory[index] = item;
    }

    pub fn insert(&mut self, item: Spell) -> bool {
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
        self.insert(item_type)
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
                (a, b) => a.cmp(&b),
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

    pub fn as_string_array(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|item| match item {
                Some(item) => item.0.clone(),
                None => "".to_string(),
            })
            .collect()
    }
}
