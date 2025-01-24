use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::level::entities::Spawn;
use crate::level::tile::Tile;
use crate::registry::Registry;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Zone {
    /// モンスターがスポーンしないエリア
    SafeZone,

    /// モンスターがスポーンするエリア
    Dungeon,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LevelTile {
    pub tile: Option<Tile>,
    pub zone: Zone,
    pub entity: Option<Spawn>,
    pub entry_point: bool,
}

impl Default for LevelTile {
    fn default() -> Self {
        LevelTile {
            tile: Some(Tile::Blank),
            zone: Zone::SafeZone,
            entity: None,
            entry_point: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LevelChunk {
    pub biome: Tile,
    pub tiles: Vec<LevelTile>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub dirty: Option<(i32, i32, i32, i32)>,
}

impl LevelChunk {
    pub fn get_level_tile(&self, x: i32, y: i32) -> &LevelTile {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return &LevelTile {
                tile: Some(Tile::Blank),
                zone: Zone::SafeZone,
                entity: None,
                entry_point: false,
            };
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return &self.tiles[i];
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.get_level_tile(x, y).tile.unwrap_or(self.biome)
    }

    pub fn get_tile_by_coords(&self, p: Vec2) -> Tile {
        let x = (p.x / TILE_SIZE as f32).trunc() as i32;
        let y = (-p.y / TILE_SIZE as f32).trunc() as i32;
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

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y).is_wall()
    }

    #[allow(dead_code)]
    pub fn is_floor(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y).is_floor()
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        self.tiles[i].tile = Some(tile);
        self.dirty = if let Some((min_x, min_y, max_x, max_y)) = self.dirty {
            Some((min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y)))
        } else {
            Some((x, y, x, y))
        };
    }

    pub fn set_tile_by_position(&mut self, position: Vec2, tile: Tile) {
        self.set_tile(
            (position.x / TILE_SIZE).trunc() as i32,
            (-position.y / TILE_SIZE).trunc() as i32,
            tile,
        );
    }

    /// 実際に描画する天井タイルかどうかを返します
    /// 天井が奥の床を隠して見えづらくなるのを避けるため、
    /// 天井タイルが3連続するところだけを描画します
    pub fn is_visible_ceil(&self, x: i32, y: i32, depth: i32, targets: &Vec<Tile>) -> bool {
        for i in 0..depth {
            if targets.contains(&self.get_tile(x, y - i)) {
                continue;
            }
            return false;
        }
        return true;
    }

    pub fn entry_points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        for y in self.min_y..self.max_y {
            for x in self.min_x..self.max_x {
                if self.get_tile(x, y) == Tile::StoneTile {
                    if let Some(LevelTile {
                        entry_point: true, ..
                    }) = self.tiles.get(
                        (y - self.min_y) as usize * (self.max_x - self.min_x) as usize
                            + (x - self.min_x) as usize,
                    ) {
                        points.push((x, y));
                    }
                }
            }
        }
        points
    }
}

pub fn index_to_position((tx, ty): (i32, i32)) -> Vec2 {
    Vec2::new(
        tx as f32 * TILE_SIZE + TILE_HALF,
        ty as f32 * -TILE_SIZE - TILE_HALF,
    )
}

#[allow(dead_code)]
pub fn position_to_index(position: Vec2) -> (i32, i32) {
    (
        (position.x / TILE_SIZE).floor() as i32,
        (position.y / -TILE_SIZE).floor() as i32,
    )
}

pub fn image_to_tilemap(
    registry: &Registry,
    biome_tile: Tile,
    level_image: &Image,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> LevelChunk {
    let map = &registry.spell().tiles;
    let texture_width = level_image.width();
    let mut tiles: Vec<LevelTile> = Vec::new();
    for y in min_y..max_y {
        for x in min_x..max_x {
            let i = 4 * (y * texture_width as i32 + x) as usize;
            let r = level_image.data[i + 0];
            let g = level_image.data[i + 1];
            let b = level_image.data[i + 2];
            let a = level_image.data[i + 3];
            let tile = match map.get(&(r, g, b, a)) {
                Some(tile) => tile,
                None => {
                    warn!("Unknown tile: {:?} {:?} {:?} {:?}", r, g, b, a);
                    &LevelTile::default()
                }
            };
            tiles.push(tile.clone());
        }
    }
    return LevelChunk {
        biome: biome_tile,
        tiles,
        min_x,
        max_x,
        min_y,
        max_y,
        dirty: None,
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
