use crate::wand::WandType;

pub struct WandProps {
    pub name: &'static str,
    pub description: &'static str,
    pub slice: &'static str,
    pub capacity: usize,
}

const CYPRESS_WAND: WandProps = WandProps {
    name: "ヒノキの杖",
    description: "ヒノキでできた、もっとも一般的な杖。",
    slice: "wand_cypress",
    capacity: 3,
};

const KEY_WAND: WandProps = WandProps {
    name: "鍵の杖",
    description: "鍵の形をした杖。",
    slice: "wand_key",
    capacity: 8,
};

pub fn wand_to_props(wand: WandType) -> WandProps {
    match wand {
        WandType::CypressWand => CYPRESS_WAND,
        WandType::KeyWand => KEY_WAND,
    }
}
