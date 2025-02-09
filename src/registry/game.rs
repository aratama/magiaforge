use crate::language::Dict;
use crate::spell::Spell;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct BGMProps {
    pub author: String,
    pub title: String,
    pub appendix: String,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct GameRegistry {
    pub bgms: HashMap<String, BGMProps>,
    pub ending_bgm: String,
    pub debug_items: Vec<Spell>,
    pub debug_wands: Vec<Vec<Option<Spell>>>,

    pub tutorial_move: Dict<String>,
    pub tutorial_inventory: Dict<String>,
    pub tutorial_slot: Dict<String>,
    pub tutorial_close_inventory: Dict<String>,
    pub tutorial_cast: Dict<String>,
}
