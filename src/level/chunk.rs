use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::level::entities::Spawn;
use crate::level::tile::Tile;
use crate::level::world::GameLevel;
use crate::registry::Registry;
use crate::registry::SpawnEntityProps;
use crate::registry::TileType;
use bevy::prelude::*;
use serde_json::from_value;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Debug, Copy)]
pub struct Bounds {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Bounds {
    pub fn iter(&self) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for x in self.min_x..self.max_x {
            for y in self.min_y..self.max_y {
                v.push((x, y));
            }
        }
        v
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.min_x && x < self.max_x && y >= self.min_y && y < self.max_y
    }
}

#[derive(Clone, Debug)]
pub struct LevelChunk {
    pub level: GameLevel,
    pub tiles: Vec<Tile>,
    pub loading_index: usize,
    pub entities: HashMap<(i32, i32), SpawnEntityProps>,
    pub bounds: Bounds,
    pub dirty: Option<Bounds>,
}

impl LevelChunk {
    /// LDTKで定義されたレベルを読み取ります
    ///
    /// LDTKでは Entities という名前のレイヤーに、以下の3種類の命名規則でエンティティを配置できます
    /// 1. この関数内でハードコーディングされたもの。Ldtkのカスタムフィールドを読み取る必要がある場合はこれ。"Spell" など
    /// 2. Spawn としてハードコーディングされたもの。registry.actor.ronで拡張できる。"RandomChest"や"MagicCircle"など
    /// 3. registry.actor.ronで定義されたActorType。"StoneLantern"など
    /// このいずれにも当てはまらなかったエンティティは警告とともに無視されます。
    /// レベルの番号を指定して、画像データからレベル情報を取得します
    /// このとき、該当するレベルの複数のスライスからランダムにひとつが選択されます、
    pub fn new(registry: &Registry, level: &GameLevel, lazy_loading: bool) -> Self {
        let ldtk = registry.ldtk();
        let Some(ldtk_level) = ldtk.get_level(&level) else {
            panic!("Level not found: {:?}", level);
        };

        let min_x = (ldtk_level.world_x / ldtk.coordinate.default_grid_size) as i32;
        let max_x =
            ((ldtk_level.world_x + ldtk_level.px_wid) / ldtk.coordinate.default_grid_size) as i32;
        let min_y = (ldtk_level.world_y / ldtk.coordinate.default_grid_size) as i32;
        let max_y =
            ((ldtk_level.world_y + ldtk_level.px_hei) / ldtk.coordinate.default_grid_size) as i32;

        // タイル読み込み

        let int_grid_layer = ldtk_level.get_layer("Tiles").unwrap();
        let map: HashMap<i64, &str> = ldtk.get_tile_mapping("Tiles");
        let mut tiles: Vec<Tile> = Vec::new();
        for tile_value in int_grid_layer.int_grid_csv.iter() {
            tiles.push(if *tile_value == 0 {
                Tile::default()
            } else {
                match map.get(&tile_value) {
                    Some(tile) => Tile::new(tile),
                    None => {
                        warn!("Unknown tile: {:?}", tile_value);
                        Tile::default()
                    }
                }
            });
        }

        let tiles_length = tiles.len();

        // エンティティ読み込み

        let entity_layer = ldtk_level.get_layer("Entities").unwrap();

        let mut entities: HashMap<(i32, i32), SpawnEntityProps> = HashMap::new();

        for entity in entity_layer.entity_instances.iter() {
            let key = (entity.grid[0] as i32, entity.grid[1] as i32);

            let encoded = if entity.field_instances.is_empty() {
                serde_json::Value::String(entity.identifier.clone())
            } else {
                let mut field_map = serde_json::Map::new();

                field_map.insert("iid".to_string(), entity.iid.clone().into());

                for field in entity.field_instances.iter() {
                    let value = field.value.as_ref().unwrap_or(&serde_json::Value::Null);
                    field_map.insert(field.identifier.clone(), value.clone());
                }

                let mut external = serde_json::Map::new();
                external.insert(
                    entity.identifier.clone(),
                    serde_json::Value::Object(field_map),
                );

                serde_json::Value::Object(external.clone())
            };

            let spawn = match from_value(encoded.clone()) {
                Ok(spawn) => spawn,
                Err(_) => {
                    warn!("Failed to parse entity: {:?}", &encoded);
                    continue;
                }
            };

            entities.insert(
                key,
                SpawnEntityProps {
                    entity: spawn,
                    spawn_offset_x: 0.0,
                },
            );
        }

        return Self {
            level: level.clone(),
            tiles,
            entities,
            bounds: Bounds {
                min_x,
                max_x,
                min_y,
                max_y,
            },
            loading_index: if lazy_loading { 0 } else { tiles_length },
            dirty: if lazy_loading {
                None
            } else {
                Some(Bounds {
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                })
            },
        };
    }

    pub fn get_level_tile(&self, x: i32, y: i32) -> &Tile {
        if !self.bounds.contains(x, y) {
            static BLANK_TILE: LazyLock<Tile> = LazyLock::new(|| Tile::default());
            return &BLANK_TILE;
        }
        let w = self.bounds.max_x - self.bounds.min_x;
        let i = ((y - self.bounds.min_y) * w + (x - self.bounds.min_x)) as usize;
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
        let (x, y) = position_to_index(p);
        self.get_tile(x, y)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if !self.bounds.contains(x, y) {
            return;
        }
        let w = self.bounds.max_x - self.bounds.min_x;
        let i = ((y - self.bounds.min_y) * w + (x - self.bounds.min_x)) as usize;
        self.tiles[i] = tile;
        self.dirty = if let Some(Bounds {
            min_x,
            min_y,
            max_x,
            max_y,
        }) = self.dirty
        {
            Some(Bounds {
                min_x: min_x.min(x),
                min_y: min_y.min(y),
                max_x: max_x.max(x),
                max_y: max_y.max(y),
            })
        } else {
            Some(Bounds {
                min_x: x,
                min_y: y,
                max_x: x,
                max_y: y,
            })
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
            if targets.contains(&&self.get_tile(x, y - i)) {
                continue;
            }
            return false;
        }
        return true;
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(
            TILE_SIZE * self.bounds.min_x as f32,
            -TILE_SIZE * self.bounds.min_y as f32,
        )
    }

    pub fn entry_points(&self) -> Vec<Vec2> {
        self.entities
            .iter()
            .filter(|(_, v)| match v.entity {
                Spawn::BrokenMagicCircle { .. } => true,
                _ => false,
            })
            .map(|(k, _)| self.offset() + index_to_position(*k))
            .collect()
    }

    pub fn get_spawn_tiles(&self, registry: &Registry) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        for (x, y) in self.bounds.iter() {
            let props = registry.get_tile(&self.get_tile(x, y));
            if props.tile_type == TileType::Floor && self.entities.get(&(x, y)).is_none() {
                tiles.push((x, y));
            }
        }
        tiles
    }

    pub fn remove_isolated_tiles(&mut self, registry: &Registry, default_tile: &Tile) {
        // 縦２タイルのみ孤立して残っているものがあれば削除
        for y in self.bounds.min_y..(self.bounds.max_y + 1) {
            for x in self.bounds.min_x..(self.bounds.max_x + 1) {
                if !self.is_wall(&registry, x, y + 0)
                    && self.is_wall(&registry, x, y + 1)
                    && !self.is_wall(&registry, x, y + 2)
                {
                    warn!("filling gap at {} {}", x, y);
                    self.set_tile(x, y + 1, default_tile.clone());
                } else if !self.is_wall(&registry, x, y + 0)
                    && self.is_wall(&registry, x, y + 1)
                    && self.is_wall(&registry, x, y + 2)
                    && !self.is_wall(&registry, x, y + 3)
                {
                    warn!("filling gap at {} {}", x, y);
                    self.set_tile(x, y + 1, default_tile.clone());
                    self.set_tile(x, y + 2, default_tile.clone());
                }
            }
        }
    }

    pub fn contains_by_index(&self, x: i32, y: i32) -> bool {
        self.bounds.contains(x, y)
    }

    pub fn contains(&self, position: Vec2) -> bool {
        let (x, y) = position_to_index(position);
        self.contains_by_index(x, y)
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
        (-position.y / TILE_SIZE).floor() as i32,
    )
}
