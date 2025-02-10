use super::chunk::position_to_index;
use crate::constant::*;
use crate::level::chunk::LevelChunk;
use crate::level::tile::Tile;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::spell::Spell;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct GameLevel(pub String);

impl GameLevel {
    pub fn new<T: Into<String>>(level: T) -> Self {
        GameLevel(level.into())
    }
}

#[derive(Component, Debug, Clone)]
pub struct LevelScoped(pub GameLevel);

/// 現在のレベル、次のレベル、次のレベルでのプレイヤーキャラクターの状態など、
/// レベル間を移動するときの情報を保持します
#[derive(Resource, Debug, Clone)]
pub struct GameWorld {
    /// 現在プレイ中のレベルのマップ構造情報
    pub chunks: Vec<LevelChunk>,

    /// 次のレベル
    /// 魔法陣から転移するとこのレベルに移動します
    pub next_level: GameLevel,

    pub destination_iid: Option<String>,

    /// 次のプレイヤー状態
    /// 魔法陣から転移したとき、この状態でプレイヤーを初期化します
    pub next_state: Option<PlayerState>,

    /// 次に生成するショップアイテムのキュー
    /// これが空になったときは改めてキューを生成します
    pub shop_items: Vec<Spell>,
}

impl Default for GameWorld {
    fn default() -> Self {
        GameWorld {
            chunks: Vec::new(),
            next_level: GameLevel::new(HOME_LEVEL),
            destination_iid: None,
            next_state: None,
            shop_items: Vec::new(),
        }
    }
}

impl GameWorld {
    pub fn get_chunk(&self, level: &GameLevel) -> Option<&LevelChunk> {
        self.chunks.iter().find(|chunk| chunk.level == *level)
    }

    pub fn get_chunk_mut(&mut self, level: &GameLevel) -> Option<&mut LevelChunk> {
        self.chunks.iter_mut().find(|chunk| chunk.level == *level)
    }

    pub fn find_chunk_by_position(&self, position: Vec2) -> Option<&LevelChunk> {
        self.chunks.iter().find(|chunk| chunk.contains(position))
    }

    pub fn get_level_by_position(&self, position: Vec2) -> Option<GameLevel> {
        self.find_chunk_by_position(position)
            .map(|chunk| chunk.level.clone())
    }

    pub fn find_chunk_by_index(&self, x: i32, y: i32) -> Option<&LevelChunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.contains_by_index(x, y))
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        if let Some(chunk) = self.find_chunk_by_index(x, y) {
            chunk.get_tile(x, y).clone()
        } else {
            Tile::default()
        }
    }

    /// タイルを設定します
    /// また、このとき、周囲のタイルをdirtyにします
    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        for dy in -1..=3 {
            for dx in -1..=1 {
                let nx = x + dx;
                let ny = y + dy;
                if let Some(ref mut chunk) = self
                    .chunks
                    .iter_mut()
                    .find(|chunk| chunk.contains_by_index(nx, ny))
                {
                    if dx == 0 && dy == 0 {
                        chunk.set_tile(x, y, tile.clone());
                    } else {
                        chunk.set_dirty(nx, ny);
                    }
                }
            }
        }
    }

    pub fn get_tile_by_coords(&self, position: Vec2) -> Tile {
        if let Some(chunk) = self.find_chunk_by_position(position) {
            chunk.get_tile_by_coords(position).clone()
        } else {
            Tile::default()
        }
    }

    pub fn set_tile_by_position(&mut self, position: Vec2, tile: Tile) {
        let (x, y) = position_to_index(position);
        self.set_tile(x, y, tile);
    }

    pub fn is_wall(&self, registry: &Registry, x: i32, y: i32) -> bool {
        let Some(chunk) = self.find_chunk_by_index(x, y) else {
            return true;
        };
        chunk.is_wall(registry, x, y)
    }

    pub fn is_visible_ceil(&self, x: i32, y: i32, depth: i32, targets: &Vec<Tile>) -> bool {
        let Some(chunk) = self.find_chunk_by_index(x, y) else {
            return true;
        };
        chunk.is_visible_ceil(x, y, depth, targets)
    }
}
