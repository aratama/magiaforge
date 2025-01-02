use crate::constant::TILE_SIZE;
use crate::entity::EntityType;
use crate::level::tile::Tile;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Zone {
    /// モンスターがスポーンしないエリア
    SafeZone,

    /// モンスターがスポーンするエリア
    Dungeon,
}

#[derive(Clone, Copy, Debug)]
struct LevelTileMapile {
    tile: Tile,
    zone: Zone,
}

#[derive(Clone, Resource, Debug)]
pub struct LevelChunk {
    tiles: Vec<LevelTileMapile>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub entities: Vec<(EntityType, i32, i32)>,
    pub entry_points: Vec<(i32, i32)>,
}

impl LevelChunk {
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return Tile::Blank;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return self.tiles[i].tile;
    }

    pub fn get_tile_by_coords(&self, p: Vec2) -> Tile {
        let x = (p.x / TILE_SIZE as f32).floor() as i32;
        let y = (-p.y / TILE_SIZE as f32).floor() as i32;
        self.get_tile(x, y)
    }

    pub fn get_zone(&self, x: i32, y: i32) -> Zone {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return Zone::SafeZone;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return self.tiles[i].zone;
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

    /// 実際に描画する天井タイルかどうかを返します
    /// 天井が奥の床を隠して見えづらくなるのを避けるため、
    /// 天井タイルが3連続するところだけを描画します
    pub fn is_visible_ceil(&self, x: i32, y: i32) -> bool {
        (self.equals(x + 0, y - 0, Tile::Wall) || self.equals(x + 0, y - 0, Tile::Blank))
            && (self.equals(x + 0, y - 1, Tile::Wall) || self.equals(x + 0, y - 1, Tile::Blank))
            && (self.equals(x + 0, y - 2, Tile::Wall) || self.equals(x + 0, y - 2, Tile::Blank))
    }
}

pub fn image_to_tilemap(
    level_image: &Image,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> LevelChunk {
    let texture_width = level_image.width();
    let mut tiles: Vec<LevelTileMapile> = Vec::new();
    let mut entities = Vec::new();
    let mut entry_points = Vec::<(i32, i32)>::new();
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
                        tile: Tile::Biome,
                        zone: Zone::Dungeon,
                    });
                }
                (234, 255, 214, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                }
                (82, 75, 36, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Wall,
                        zone: Zone::SafeZone,
                    });
                }
                (118, 66, 138, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::BookShelf, x, y));
                }
                (251, 242, 54, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::Chest, x, y));
                }
                (255, 155, 87, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::CrateOrBarrel, x, y));
                }
                (48, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::MagicCircle, x, y));
                }
                (47, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::MultiPlayArenaMagicCircle, x, y));
                }
                (56, 111, 161, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::MagicCircleHome, x, y));
                }
                (255, 0, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entry_points.push((x, y));
                    entities.push((EntityType::BrokenMagicCircle, x, y));
                }
                (255, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::Usage, x, y));
                }
                (254, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::Routes, x, y));
                }
                (223, 113, 38, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::StoneLantern, x, y));
                }
                (0, 222, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::ShopSpell, x, y));
                }
                (102, 57, 49, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::Crate, x, y));
                }
                (184, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::HugeSlime, x, y));
                }
                (255, 243, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::ShopRabbit, x, y));
                }
                (255, 244, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::TrainingRabbit, x, y));
                }
                (255, 245, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::GuideRabbit, x, y));
                }
                (255, 246, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::MultiplayerRabbit, x, y));
                }
                (255, 247, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::SinglePlayRabbit, x, y));
                }
                (255, 248, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::ReadingRabbit, x, y));
                }
                (255, 249, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::SpellListRabbit, x, y));
                }
                (182, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::Sandbug, x, y));
                }
                (197, 255, 142, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::ShopDoor, x, y));
                }
                (68, 0, 94, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push((EntityType::BGM, x, y));
                }
                (153, 229, 80, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Grassland,
                        zone: Zone::SafeZone,
                    });
                }
                (156, 156, 156, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                }
                _ => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Blank,
                        zone: Zone::SafeZone,
                    });
                    panic!(
                        "Unknown color: ({}, {}, {}, {}) at ({}, {})",
                        r, g, b, a, x, y
                    );
                }
            }
        }
    }
    return LevelChunk {
        tiles,
        min_x,
        max_x,
        min_y,
        max_y,
        entities,
        entry_points,
    };
}

pub fn image_to_spawn_tiles(tilemap: &LevelChunk) -> Vec<(i32, i32)> {
    let mut tiles = Vec::new();
    for y in tilemap.min_y..tilemap.max_y {
        for x in tilemap.min_x..tilemap.max_x {
            match tilemap.get_zone(x, y) {
                Zone::Dungeon => {
                    tiles.push((x, y));
                }
                _ => {}
            }
        }
    }
    tiles
}
