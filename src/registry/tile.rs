use crate::level::tile::Tile;
use std::collections::HashMap;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct TileRegistry {
    pub tile_types: HashMap<String, TileTypeProps>,
}

#[derive(serde::Deserialize, Debug)]
pub struct TileTypeProps {
    pub tile_type: TileType,

    #[serde(default)]
    pub layers: Vec<TileTypeLayer>,
    /// それぞれのタイルに以下の照度をランダムに割り当てます
    #[serde(default)]
    pub light_hue: f32,

    #[serde(default)]
    pub light_saturation: f32,

    #[serde(default)]
    pub light_lightness: f32,

    #[serde(default)]
    pub light_intensity: f32,

    #[serde(default)]
    pub light_radius: f32,

    #[serde(default)]
    pub light_density: f32,

    #[serde(default)]
    pub grasses: bool,

    #[serde(default)]
    pub break_into: Option<Tile>,
}

#[derive(serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Surface,
    Floor,
}

#[derive(serde::Deserialize, Debug)]
pub struct TileTypeLayer {
    pub depth: f32,
    pub tiling: Tiling,
}

#[derive(serde::Deserialize, Debug)]
pub enum Tiling {
    Simple { patterns: Vec<Vec<String>> },
    Auto { prefixes: Vec<Vec<String>> },
}
