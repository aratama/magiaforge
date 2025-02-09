use crate::spell::SpellProps;
use std::collections::HashMap;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct SpellRegistry {
    pub spells: HashMap<String, SpellProps>,
}
