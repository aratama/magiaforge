use crate::spell::SpellType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WandType {
    CypressWand,
}

pub struct Wand {
    pub wand_type: WandType,
    pub slots: Vec<Option<SpellType>>,
}

pub struct WandProps {
    pub slice: &'static str,
}

const CYPRESS_WAND: WandProps = WandProps {
    slice: "wand_cypress",
};

pub fn wand_to_props(wand: WandType) -> WandProps {
    match wand {
        WandType::CypressWand => CYPRESS_WAND,
    }
}
