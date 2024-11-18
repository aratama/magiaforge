use crate::{entity::GameEntity, level::tile::Tile};
use bevy::{
    a11y::accesskit::Vec2,
    log::*,
    prelude::{Image, Resource},
};

#[derive(Clone, Copy)]
pub enum Biome {
    /// モンスターがスポーンしないエリア
    SafeZone,

    /// モンスターがスポーンするエリア
    Dungeon,
}

#[derive(Clone, Copy)]
struct LevelTileMapile {
    tile: Tile,
    biome: Biome,
}

#[derive(Clone, Resource)]
pub struct LevelTileMap {
    tiles: Vec<LevelTileMapile>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub entities: Vec<(GameEntity, i32, i32)>,
    pub entry_points: Vec<Vec2>,
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

    pub fn get_biome(&self, x: i32, y: i32) -> Biome {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return Biome::SafeZone;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return self.tiles[i].biome;
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
    let mut entry_points = Vec::new();
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
                        biome: Biome::Dungeon,
                    });
                }
                (234, 255, 214, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                }
                (82, 75, 36, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Wall,
                        biome: Biome::SafeZone,
                    });
                }
                (118, 66, 138, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::BookShelf, x, y));
                }
                (251, 242, 54, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::Chest, x, y));
                }
                (48, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::MagicCircle, x, y));
                }
                (47, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::MultiPlayArenaMagicCircle, x, y));
                }
                (56, 111, 161, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::MagicCircleHome, x, y));
                }
                (255, 0, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entry_points.push(Vec2::new(x as f64, y as f64));
                    entities.push((GameEntity::BrokenMagicCircle, x, y));
                }
                (255, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::Usage, x, y));
                }
                (254, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::Routes, x, y));
                }
                (223, 113, 38, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::StoneLantern, x, y));
                }
                (0, 222, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::Spell, x, y));
                }
                (0, 255, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::Wand, x, y));
                }
                (102, 57, 49, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        biome: Biome::SafeZone,
                    });
                    entities.push((GameEntity::Crate, x, y));
                }
                _ => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Blank,
                        biome: Biome::SafeZone,
                    });
                    error!("Unknown color: ({}, {}, {}, {})", r, g, b, a);
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
        entry_points,
    };
}

pub fn image_to_spawn_tiles(tilemap: &LevelTileMap) -> Vec<(i32, i32)> {
    let mut tiles = Vec::new();
    for y in tilemap.min_y..tilemap.max_y {
        for x in tilemap.min_x..tilemap.max_x {
            match tilemap.get_biome(x, y) {
                Biome::Dungeon => {
                    tiles.push((x, y));
                }
                _ => {}
            }
        }
    }
    tiles
}
