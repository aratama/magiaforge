use super::super::{entity::GameEntity, world::tile::Tile};
use bevy::prelude::{Image, Resource};

#[derive(Clone, Copy)]
struct LevelTileMapile {
    tile: Tile,
    life: i32,
}

#[derive(Clone, Resource)]
pub struct LevelTileMap {
    tiles: Vec<LevelTileMapile>,
    pub width: i32,
    pub height: i32,
    pub entities: Vec<(GameEntity, i32, i32)>,
}

impl LevelTileMap {
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Tile::Blank;
        }
        let i = (y * self.width + x) as usize;
        return self.tiles[i].tile;
    }

    /// 指定した位置のタイルが、指定したタイルと同じ種類かどうかを返します
    /// 範囲外を指定した場合は、trueを返します
    pub fn equals(&self, x: i32, y: i32, tile: Tile) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return true;
        }
        let i = (y * self.width + x) as usize;
        return self.tiles[i].tile == tile;
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let i = (y * self.width + x) as usize;
        self.tiles[i].tile = tile;
        self.tiles[i].life = 4;
    }

    pub fn get_life(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return 0;
        }
        let i = (y * self.width + x) as usize;
        return self.tiles[i].life;
    }

    pub fn set_life(&mut self, x: i32, y: i32, life: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let i = (y * self.width + x) as usize;
        self.tiles[i].life = life;
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y) == Tile::StoneTile
    }
}

pub fn image_to_tilemap(level_image: &Image) -> LevelTileMap {
    let width = level_image.width() as i32;
    let height = level_image.height() as i32;
    let mut tiles: Vec<LevelTileMapile> = Vec::new();
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
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        life: 4,
                    });
                }
                (82, 75, 36, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Wall,
                        life: 4,
                    });
                }
                (118, 66, 138, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        life: 4,
                    });
                    entities.push((GameEntity::BookShelf, x, y));
                }
                (251, 242, 54, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        life: 4,
                    });
                    entities.push((GameEntity::Chest, x, y));
                }
                _ => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Blank,
                        life: 4,
                    });
                }
            }
        }
    }
    return LevelTileMap {
        tiles,
        width,
        height,
        entities,
    };
}
