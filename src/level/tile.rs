#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tile {
    Blank,
    Wall,
    Biome,
    StoneTile,
    Grassland,
    Water,
}
