use bevy::reflect::Reflect;

use crate::{constant::MAX_SPELLS_IN_WAND, spell::SpellType};

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WandType {
    CypressWand,
    KeyWand,
}

#[derive(Reflect, Clone, Copy, Debug)]
pub struct WandSpell {
    pub spell_type: SpellType,
    pub price: u32,
}

#[derive(Reflect, Clone, Debug)]
pub struct Wand {
    pub wand_type: WandType,
    pub price: u32,
    pub slots: [Option<WandSpell>; MAX_SPELLS_IN_WAND],
    pub index: usize,
}

impl Wand {
    pub fn dept(&self) -> u32 {
        self.slots
            .iter()
            .filter_map(|spell| spell.as_ref())
            .map(|spell| spell.price)
            .sum()
    }

    pub fn shift(&mut self) {
        let props = self.wand_type.to_props();
        self.index = (self.index + 1) % props.capacity;
        for _ in 0..MAX_SPELLS_IN_WAND {
            if self.slots[self.index].is_none() {
                self.index = (self.index + 1) % props.capacity;
                continue;
            } else {
                break;
            }
        }
    }
}
