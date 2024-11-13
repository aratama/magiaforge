use crate::{constant::MAX_SPELLS_IN_WAND, spell::SpellType, wand_props::wand_to_props};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WandType {
    CypressWand,
    KeyWand,
}

#[derive(Clone, Debug)]
pub struct Wand {
    pub wand_type: WandType,
    pub slots: [Option<SpellType>; MAX_SPELLS_IN_WAND],
    pub index: usize,
}

impl Wand {
    pub fn shift(&mut self) {
        let props = wand_to_props(self.wand_type);
        self.index = (self.index + 1) % props.capacity;
        while self.slots[self.index].is_none() {
            self.index = (self.index + 1) % props.capacity;
        }
    }
}
