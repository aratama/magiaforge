use crate::constant::MAX_SPELLS_IN_WAND;
use crate::spell::SpellType;
use bevy::reflect::Reflect;

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

impl WandSpell {
    pub fn new(spell_type: SpellType) -> Self {
        Self {
            spell_type,
            price: 0,
        }
    }
}

#[derive(Reflect, Clone, Debug)]
pub struct Wand {
    pub wand_type: WandType,
    pub price: u32,
    pub slots: [Option<WandSpell>; MAX_SPELLS_IN_WAND],
    pub index: usize,
    pub delay: u32,
}

impl Wand {
    // pub fn new(wand_type: WandType) -> Self {
    //     Self {
    //         wand_type,
    //         price: 0,
    //         slots: [None; MAX_SPELLS_IN_WAND],
    //         index: 0,
    //         delay: 0,
    //     }
    // }

    pub fn with_slots(wand_type: WandType, slots: [Option<WandSpell>; MAX_SPELLS_IN_WAND]) -> Self {
        Self {
            wand_type,
            price: 0,
            slots,
            index: 0,
            delay: 0,
        }
    }

    pub fn dept(&self) -> u32 {
        self.slots
            .iter()
            .filter_map(|spell| spell.as_ref())
            .map(|spell| spell.price)
            .sum()
    }
}
