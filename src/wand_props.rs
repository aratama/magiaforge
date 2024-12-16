use crate::{language::Dict, wand::WandType};

pub struct WandProps {
    pub name: Dict,
    pub description: Dict,
    pub slice: &'static str,
    pub icon: &'static str,
    pub capacity: usize,
}

const CYPRESS_WAND: WandProps = WandProps {
    name: Dict {
        ja: "ヒノキの杖",
        en: "Cypress Wand",
    },
    description: Dict {
        ja: "ヒノキでできた、もっとも一般的な杖。",
        en: "The most common wand, made of cypress.",
    },
    slice: "wand_cypress",
    icon: "wand_icon_cypress",
    capacity: 8,
};

const KEY_WAND: WandProps = WandProps {
    name: Dict {
        ja: "鍵の杖",
        en: "Key Wand",
    },
    description: Dict {
        ja: "鍵の形をした杖。なぜこんな形なんだろう……？",
        en: "A wand shaped like a key.",
    },
    slice: "wand_key",
    icon: "wand_icon_key",
    capacity: 8,
};

impl WandType {
    pub fn to_props(&self) -> WandProps {
        match self {
            WandType::CypressWand => CYPRESS_WAND,
            WandType::KeyWand => KEY_WAND,
        }
    }
}
