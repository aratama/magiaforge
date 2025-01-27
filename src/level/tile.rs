#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize)]
pub struct Tile(pub String);

impl Tile {
    pub fn new(s: &str) -> Self {
        Tile(s.to_string())
    }
}

impl Tile {
    /// アクターやエンティティが乗ることができないタイルです
    pub fn is_wall(&self) -> bool {
        match self.0.as_str() {
            "Wall" => true,
            "PermanentWall" => true,
            "Blank" => true,
            _ => false,
        }
    }

    /// 上にアクターやエンティティが乗ることができるタイルを表します
    /// ただし、水面や溶岩など、通常は通行を避けるべきタイルも含まれます
    pub fn is_plane(&self) -> bool {
        match self.0.as_str() {
            "StoneTile" => true,
            "Ice" => true,
            "Grassland" => true,
            "Soil" => true,
            "Water" => true,
            "Lava" => true,
            _ => false,
        }
    }

    /// アクターやエンティティが通常通行できるタイルです
    #[allow(dead_code)]
    pub fn is_floor(&self) -> bool {
        match self.0.as_str() {
            "StoneTile" => true,
            "Ice" => true,
            "Grassland" => true,
            "Soil" => true,
            _ => false,
        }
    }
}
