#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Tile(pub String);

impl Tile {
    pub fn new(s: &str) -> Self {
        Tile(s.to_string())
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::new("Soil")
    }
}
