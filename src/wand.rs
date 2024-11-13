use crate::{constant::MAX_SPELLS_IN_WAND, spell::SpellType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WandType {
    CypressWand,
    KeyWand,
}

#[derive(Clone, Copy, Debug)]
pub struct Wand {
    pub wand_type: WandType,
    pub slots: [Option<SpellType>; MAX_SPELLS_IN_WAND],
}
