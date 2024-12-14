use bevy::reflect::Reflect;

use crate::{constant::MAX_SPELLS_IN_WAND, spell::SpellType, wand_props::wand_to_props};

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WandType {
    CypressWand,
    KeyWand,
}

#[derive(Reflect, Clone, Debug)]
pub struct Wand {
    pub wand_type: WandType,
    pub slots: [Option<SpellType>; MAX_SPELLS_IN_WAND],
    pub index: usize,
}

impl Wand {
    pub fn shift(&mut self) {
        let props = wand_to_props(self.wand_type);
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
