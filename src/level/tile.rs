#[derive(PartialEq, Eq, Clone, Copy, Debug, serde::Deserialize)]
pub enum Tile {
    Blank,
    Wall,
    PermanentWall,
    StoneTile,
    Grassland,
    Water,
    Ice,
    Lava,
    Crack,
    Soil,
}

impl Tile {
    /// アクターやエンティティが乗ることができないタイルです
    pub fn is_wall(&self) -> bool {
        match self {
            Tile::Wall => true,
            Tile::PermanentWall => true,
            Tile::Blank => true,
            _ => false,
        }
    }

    /// 上にアクターやエンティティが乗ることができるタイルを表します
    /// ただし、水面や溶岩など、通常は通行を避けるべきタイルも含まれます
    pub fn is_plane(&self) -> bool {
        match self {
            Tile::StoneTile => true,
            Tile::Ice => true,
            Tile::Grassland => true,
            Tile::Soil => true,
            Tile::Water => true,
            Tile::Lava => true,
            _ => false,
        }
    }

    /// アクターやエンティティが通常通行できるタイルです
    #[allow(dead_code)]
    pub fn is_floor(&self) -> bool {
        match self {
            Tile::StoneTile => true,
            Tile::Ice => true,
            Tile::Grassland => true,
            Tile::Soil => true,
            _ => false,
        }
    }
}
