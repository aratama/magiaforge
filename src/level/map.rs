use crate::aseprite_raw_loader::RawAseprite;
use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::level::entities::Spawn;
use crate::level::tile::Tile;
use crate::page::in_game::GameLevel;
use crate::registry::Registry;
use crate::registry::SpawnEntityProps;
use crate::registry::TileType;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Debug)]
pub struct LevelChunk {
    pub tiles: Vec<Tile>,
    pub entities: HashMap<(i32, i32), SpawnEntityProps>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub dirty: Option<(i32, i32, i32, i32)>,
}

impl LevelChunk {
    /// レベルの番号を指定して、画像データからレベル情報を取得します
    /// このとき、該当するレベルの複数のスライスからランダムにひとつが選択されます、
    pub fn new(
        registry: &Registry,
        raw_aseprites: &Res<Assets<RawAseprite>>,
        level: &GameLevel,
    ) -> Self {
        let raw_aseprite = raw_aseprites.get(registry.assets.raw_level.id()).unwrap();

        let slice = raw_aseprite
            .aseprite
            .slices()
            .iter()
            .find(|s| s.name == level.0)
            .unwrap();
        let slice_keys = slice.slice_keys[0];
        let tile_map = raw_aseprite.get_layer_by_name("tiles", 0).unwrap();
        let entities_map = raw_aseprite.get_layer_by_name("entities", 0).unwrap();
        let chunk = image_to_tilemap(
            &registry,
            &tile_map,
            &entities_map,
            slice_keys.x as i32,
            slice_keys.x + slice_keys.width as i32,
            slice_keys.y as i32,
            slice_keys.y + slice_keys.height as i32,
        );

        return chunk;
    }

    pub fn get_level_tile(&self, x: i32, y: i32) -> &Tile {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            static BLANK_TILE: LazyLock<Tile> = LazyLock::new(|| Tile::default());
            return &BLANK_TILE;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        return &self.tiles[i];
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        self.get_level_tile(x, y)
    }

    pub fn get_tile_type(&self, registry: &Registry, x: i32, y: i32) -> TileType {
        let tile = self.get_tile(x, y);
        registry.get_tile(&tile).tile_type
    }

    pub fn is_wall(&self, registry: &Registry, x: i32, y: i32) -> bool {
        self.get_tile_type(&registry, x, y) == TileType::Wall
    }

    pub fn get_tile_by_coords(&self, p: Vec2) -> &Tile {
        let x = (p.x / TILE_SIZE as f32).trunc() as i32;
        let y = (-p.y / TILE_SIZE as f32).trunc() as i32;
        self.get_tile(x, y)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        self.tiles[i] = tile;
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
    pub fn is_visible_ceil(&self, x: i32, y: i32, depth: i32, targets: &Vec<&Tile>) -> bool {
        for i in 0..depth {
            if targets.contains(&&self.get_tile(x, y - i)) {
                continue;
            }
            return false;
        }
        return true;
    }

    pub fn entry_points(&self) -> Vec<(i32, i32)> {
        self.entities
            .iter()
            .filter(|(_, v)| match v.entity {
                Spawn::BrokenMagicCircle => true,
                _ => false,
            })
            .map(|(k, _)| *k)
            .collect()
    }

    pub fn get_spawn_tiles(&self, registry: &Registry) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        for y in self.min_y..self.max_y {
            for x in self.min_x..self.max_x {
                let props = registry.get_tile(&self.get_tile(x, y));
                if props.tile_type == TileType::Floor && self.entities.get(&(x, y)).is_none() {
                    tiles.push((x, y));
                }
            }
        }
        tiles
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

fn image_to_tilemap(
    registry: &Registry,
    level_image: &Image,
    entities_image: &Image,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> LevelChunk {
    let texture_width = level_image.width();
    let map = &registry.tile().color_to_tile_mapping;
    let mut tiles: Vec<Tile> = Vec::new();
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
                    &Tile::default()
                }
            };
            tiles.push(tile.clone());
        }
    }

    let entity_map = &registry.tile().color_to_entity_mapping;
    let mut entities: HashMap<(i32, i32), SpawnEntityProps> = HashMap::new();

    for y in min_y..max_y {
        for x in min_x..max_x {
            let i = 4 * (y * texture_width as i32 + x) as usize;
            let r = entities_image.data[i + 0];
            let g = entities_image.data[i + 1];
            let b = entities_image.data[i + 2];
            let a = entities_image.data[i + 3];
            match entity_map.get(&(r, g, b, a)) {
                Some(tile) => {
                    entities.insert((x, y), tile.clone());
                }
                None => {
                    if (r == 0 && g == 0 && b == 0) || a == 0 {
                    } else {
                        warn!(
                            "Unknown entity: {:?} {:?} {:?} {:?} at ({},{})",
                            r, g, b, a, x, y
                        );
                    };
                }
            };
        }
    }

    return LevelChunk {
        tiles,
        entities,
        min_x,
        max_x,
        min_y,
        max_y,
        dirty: None,
    };
}
