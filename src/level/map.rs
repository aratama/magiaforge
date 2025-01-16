use crate::constant::TILE_HALF;
use crate::constant::TILE_SIZE;
use crate::entity::chest::ChestItem;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::level::entities::SpawnEntity;
use crate::level::tile::Tile;
use crate::spell::SpellType;
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

#[derive(Clone, Debug)]
pub struct LevelChunk {
    tiles: Vec<LevelTileMapile>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub entities: Vec<SpawnEntity>,
    pub entry_points: Vec<(i32, i32)>,
    pub dirty: Option<(i32, i32, i32, i32)>,
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
        let tile = self.get_tile(x, y);
        tile == Tile::Wall || tile == Tile::PermanentWall || tile == Tile::Blank
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x < self.min_x || x >= self.max_x || y < self.min_y || y >= self.max_y {
            return;
        }
        let w = self.max_x - self.min_x;
        let i = ((y - self.min_y) * w + (x - self.min_x)) as usize;
        self.tiles[i].tile = tile;
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
}

pub fn index_to_position((tx, ty): (i32, i32)) -> Vec2 {
    Vec2::new(tx as f32 * TILE_SIZE, ty as f32 * -TILE_SIZE) + Vec2::new(TILE_HALF, TILE_HALF)
}

/// レベルマップでのタイルの整数での位置を、ワールド座標に変換します
/// 変換後の座標はタイルの角ではなく、タイルの中央となります
/// これは通常はエンティティの中心と一致します
fn tuple_to_vec2((x, y): (i32, i32)) -> Vec2 {
    Vec2::new(
        x as f32 * TILE_SIZE + TILE_HALF,
        -y as f32 * TILE_SIZE - TILE_HALF,
    )
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

            let position = tuple_to_vec2((x, y));

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
                    entities.push(SpawnEntity::BookShelf {
                        // 本棚は横2タイルの幅があるため、エンティティの中央はタイルの中央より半タイル分だけ右になります
                        position: position + Vec2::new(TILE_HALF, 0.0),
                    });
                }
                (251, 242, 54, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::Chest {
                        position,
                        item: if (x, y) == (81, 28) {
                            ChestItem::Item(InventoryItem::new(InventoryItemType::Spell(
                                SpellType::Dash,
                            )))
                        } else if (x, y) == (30, 69) {
                            ChestItem::Item(InventoryItem::new(InventoryItemType::Spell(
                                SpellType::Levitation,
                            )))
                        } else if (x, y) == (53, 24) {
                            ChestItem::Item(InventoryItem::new(InventoryItemType::Spell(
                                SpellType::Bomb,
                            )))
                        } else if (x, y) == (100, 66) {
                            ChestItem::Item(InventoryItem::new(InventoryItemType::Spell(
                                SpellType::LightSword,
                            )))
                        } else {
                            ChestItem::Gold(4)
                        },
                    });
                }
                (255, 155, 87, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::CrateOrBarrel { position });
                }
                (48, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::MagicCircle { position });
                }
                (47, 96, 130, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::MultiPlayArenaMagicCircle { position });
                }
                (56, 111, 161, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::MagicCircleHome { position });
                }
                (255, 0, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entry_points.push((x, y));
                    entities.push(SpawnEntity::BrokenMagicCircle { position });
                }
                (255, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::Usage { position });
                }
                (254, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::Routes { position });
                }
                (223, 113, 38, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::StoneLantern {
                        position: tuple_to_vec2((x, y)),
                    });
                }
                (0, 222, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::ShopSpell { position });
                }
                (102, 57, 49, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::Crate { position });
                }
                (184, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::HugeSlime { position });
                }
                (255, 243, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::ShopRabbit { position });
                }
                (255, 244, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::TrainingRabbit { position });
                }
                (255, 245, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::GuideRabbit { position });
                }
                (255, 246, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::MultiplayerRabbit { position });
                }
                (255, 247, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::SinglePlayRabbit { position });
                }
                (255, 248, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::ReadingRabbit { position });
                }
                (255, 249, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::SpellListRabbit { position });
                }
                (182, 0, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::Sandbug { position });
                }
                (197, 255, 142, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::StoneTile,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::ShopDoor { position });
                }
                (68, 0, 94, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Biome,
                        zone: Zone::SafeZone,
                    });
                    entities.push(SpawnEntity::BGM { position });
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
                (55, 79, 225, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Water,
                        zone: Zone::SafeZone,
                    });
                }
                (205, 121, 54, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Lava,
                        zone: Zone::SafeZone,
                    });
                }
                (43, 43, 43, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Crack,
                        zone: Zone::SafeZone,
                    });
                }
                (222, 233, 255, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::Ice,
                        zone: Zone::SafeZone,
                    });
                }
                (0, 0, 0, 255) => {
                    tiles.push(LevelTileMapile {
                        tile: Tile::PermanentWall,
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
