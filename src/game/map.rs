use bevy::prelude::Image;

use super::{entity::GameEntity, tile::Tile};

#[derive(Clone)]
pub struct TileMapChunk {
    tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
    pub entities: Vec<(GameEntity, i32, i32)>,
}

impl TileMapChunk {
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Tile::Blank;
        }
        let i = (y * self.width + x) as usize;
        return self.tiles[i];
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y) == Tile::StoneTile
    }
}

pub fn image_to_tilemap(level_image: &Image) -> TileMapChunk {
    let width = level_image.width() as i32;
    let height = level_image.height() as i32;
    let mut tiles = Vec::new();
    let mut entities = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let i = 4 * (y * width as i32 + x) as usize;
            let r = level_image.data[i + 0];
            let g = level_image.data[i + 1];
            let b = level_image.data[i + 2];
            let a = level_image.data[i + 3];
            match (r, g, b, a) {
                (203, 219, 252, 255) => {
                    tiles.push(Tile::StoneTile);
                }
                (82, 75, 36, 255) => {
                    tiles.push(Tile::Wall);
                }
                (118, 66, 138, 255) => {
                    tiles.push(Tile::StoneTile);
                    entities.push((GameEntity::BookShelf, x, y));
                }
                (251, 242, 54, 255) => {
                    tiles.push(Tile::StoneTile);
                    entities.push((GameEntity::Chest, x, y));
                }
                _ => {
                    tiles.push(Tile::Blank);
                }
            }
        }
    }
    return TileMapChunk {
        tiles,
        width,
        height,
        entities,
    };
}
