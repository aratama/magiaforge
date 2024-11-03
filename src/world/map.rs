use crate::{entity::GameEntity, world::tile::Tile};
use bevy::{
    a11y::accesskit::Vec2,
    prelude::{Image, Resource},
};

#[derive(Clone, Copy)]
struct LevelTileMapile {
    tile: Tile,
}

#[derive(Clone, Resource)]
pub struct LevelTileMap {
    tiles: Vec<LevelTileMapile>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub entities: Vec<(GameEntity, i32, i32)>,
    pub entry_point: Vec2,
}

impl LevelTileMap {
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return Tile::Blank;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return self.tiles[i].tile;
    }

    /// 指定した位置のタイルが、指定したタイルと同じ種類かどうかを返します
    /// 範囲外を指定した場合は、trueを返します
    pub fn equals(&self, x: i32, y: i32, tile: Tile) -> bool {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return true;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return self.tiles[i].tile == tile;
    }

    #[allow(dead_code)]
    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        self.tiles[i].tile = tile;
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y) == Tile::StoneTile
    }
}

pub fn image_to_tilemap(
    level_image: &Image,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> LevelTileMap {
    let texture_width = level_image.width();
    let mut tiles: Vec<LevelTileMapile> = Vec::new();
    let mut entities = Vec::new();
    let mut entry_point = Vec2::new(0.0, 0.0);
    for y in min_y..max_y {
        for x in min_x..max_x {
            let i = 4 * (y * texture_width as i32 + x) as usize;
            let r = level_image.data[i + 0];
            let g = level_image.data[i + 1];
            let b = level_image.data[i + 2];
            let a = level_image.data[i + 3];

            match (r, g, b, a) {
                (203, 219, 252, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                    });
                }
                (82, 75, 36, 255) => {
                    tiles.push(LevelTileMapile { tile: Tile::Wall });
                }
                (118, 66, 138, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                    });
                    entities.push((GameEntity::BookShelf, x, y));
                }
                (251, 242, 54, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                    });
                    entities.push((GameEntity::Chest, x, y));
                }
                (48, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                    });
                    entities.push((GameEntity::MagicCircle, x, y));
                }
                (255, 0, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                    });
                    entry_point = Vec2::new(x as f64, y as f64);
                }
                _ => {
                    tiles.push(LevelTileMapile { tile: Tile::Blank });
                }
            }
        }
    }
    return LevelTileMap {
        tiles,
        min_x,
        max_x,
        min_y,
        max_y,
        entities,
        entry_point,
    };
}

pub fn image_to_empty_tiles(tilemap: &LevelTileMap) -> Vec<(i32, i32)> {
    let mut tiles = Vec::new();
    for y in tilemap.min_y..tilemap.max_y {
        for x in tilemap.min_x..tilemap.max_x {
            match tilemap.get_tile(x, y) {
                Tile::StoneTile => {
                    tiles.push((x, y));
                }
                _ => {}
            }
        }
    }
    tiles
}
