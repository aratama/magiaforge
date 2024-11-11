use crate::spell::Spell;
pub enum WandType {
    CypressWand,
}

pub struct Wand {
    pub wand_type: WandType,
    pub slots: Vec<Option<Spell>>,
}
