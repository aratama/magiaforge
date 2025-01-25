use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::MAX_WANDS;
use crate::spell::Spell;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Reflect, Clone, Debug, Serialize, Deserialize)]
pub struct WandSpell {
    pub spell_type: Spell,
    pub price: u32,
}

impl Default for Wand {
    fn default() -> Self {
        Self {
            slots: [None, None, None, None, None, None, None, None],
            index: 0,
            delay: 0,
        }
    }
}

impl WandSpell {
    pub fn new(spell_type: Spell) -> Self {
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
    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|spell| spell.is_none())
    }

    pub fn with_slots(slots: [Option<WandSpell>; MAX_SPELLS_IN_WAND]) -> Self {
        Self {
            slots,
            index: 0,
            delay: 0,
        }
    }

    pub fn from_vec(slots: &Vec<Vec<Option<Spell>>>) -> [Wand; MAX_WANDS] {
        let mut wands = [
            Wand::default(),
            Wand::default(),
            Wand::default(),
            Wand::default(),
        ];
        for (i, wand_slots) in slots.iter().enumerate().take(MAX_WANDS) {
            for (j, spell_type) in wand_slots.iter().enumerate().take(MAX_SPELLS_IN_WAND) {
                wands[i].slots[j] = spell_type.as_ref().map(|s| WandSpell::new(s.clone()));
            }
        }
        wands
    }

    pub fn dept(&self) -> u32 {
        self.slots
            .iter()
            .filter_map(|spell| spell.as_ref())
            .map(|spell| spell.price)
            .sum()
    }
}
