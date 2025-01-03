use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::MAX_WANDS;
use crate::spell::SpellType;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct Wand {
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

    pub fn empty() -> Self {
        Self {
            slots: [None; MAX_SPELLS_IN_WAND],
            index: 0,
            delay: 0,
        }
    }

    pub fn with_slots(slots: [Option<WandSpell>; MAX_SPELLS_IN_WAND]) -> Self {
        Self {
            slots,
            index: 0,
            delay: 0,
        }
    }

    pub fn single(spell: Option<SpellType>) -> [Wand; MAX_WANDS] {
        let mut slots = [None; MAX_SPELLS_IN_WAND];
        slots[0] = spell.map(|s| WandSpell::new(s));
        [
            Wand::with_slots(slots),
            Wand::empty(),
            Wand::empty(),
            Wand::empty(),
        ]
    }

    pub fn dept(&self) -> u32 {
        self.slots
            .iter()
            .filter_map(|spell| spell.as_ref())
            .map(|spell| spell.price)
            .sum()
    }
}
