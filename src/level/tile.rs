#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tile {
    Blank,
    Wall,
    PermanentWall,
    Biome,
    StoneTile,
    Grassland,
    Water,
    Ice,
    Lava,
    Crack,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        match self {
            Tile::Wall => true,
            Tile::PermanentWall => true,
            Tile::Blank => true,
            _ => false,
        }
    }
}
