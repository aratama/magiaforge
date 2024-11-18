use bevy::prelude::Component;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tile {
    Blank,
    Wall,
    StoneTile,
}

#[derive(Component)]
pub struct WorldTile;
