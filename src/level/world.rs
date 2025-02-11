use std::collections::{HashMap, HashSet};

use super::chunk::position_to_index;
use crate::constant::*;
use crate::level::chunk::LevelChunk;
use crate::level::tile::Tile;
use crate::player_state::PlayerState;
use crate::registry::tile::TileType;
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

    #[allow(dead_code)]
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

    /// A* アルゴリズムを使って経路を探索します
    pub fn find_route(
        &self,
        registry: &Registry,
        start: (i32, i32),
        goal: (i32, i32),
        max_distance: i32,
    ) -> Option<Vec<(i32, i32)>> {
        // Early return if start or goal is blocked
        if self.get_movement_cost(registry, start, start, max_distance) == BLOCKED_PATH_COST
            || self.get_movement_cost(registry, goal, start, max_distance) == BLOCKED_PATH_COST
        {
            return None;
        }

        // 初期化
        let mut open_set = HashSet::from_iter(vec![start]);
        let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();
        g_score.insert(start, 0);
        let mut f_score: HashMap<(i32, i32), i32> = HashMap::new();
        f_score.insert(start, heuristic(start, goal));
        let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        // 探索
        while let Some(current) = pop_lowest_node(&mut open_set, &f_score) {
            if current == goal {
                return Some(self.reconstruct_path(current, &came_from));
            }

            open_set.remove(&current);

            for neighbor in neighbors(current) {
                let cost = self.get_movement_cost(registry, neighbor, start, max_distance);
                if cost == BLOCKED_PATH_COST {
                    continue;
                }

                let tentative_g_score = g_score.get(&current).unwrap_or(&BLOCKED_PATH_COST) + cost;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&BLOCKED_PATH_COST) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, goal));
                    open_set.insert(neighbor);
                }
            }
        }

        None
    }

    fn get_movement_cost(
        &self,
        registry: &Registry,
        pos: (i32, i32),
        start: (i32, i32),
        max_distance: i32,
    ) -> i32 {
        if manhattan_distance(pos, start) >= max_distance {
            return BLOCKED_PATH_COST;
        }

        let tile = self.get_tile(pos.0, pos.1);
        let props = registry.get_tile(&tile);
        match props.tile_type {
            TileType::Wall | TileType::Surface => BLOCKED_PATH_COST,
            TileType::Floor => 1,
        }
    }

    fn reconstruct_path(
        &self,
        current: (i32, i32),
        came_from: &HashMap<(i32, i32), (i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let mut path = vec![current];
        let mut current = current;
        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }
        path.reverse();
        path
    }
}

const BLOCKED_PATH_COST: i32 = std::i32::MAX;

fn pop_lowest_node(
    open_set: &mut HashSet<(i32, i32)>,
    f_score: &HashMap<(i32, i32), i32>,
) -> Option<(i32, i32)> {
    open_set
        .iter()
        .min_by_key(|&node| f_score.get(node).unwrap_or(&BLOCKED_PATH_COST))
        .copied()
}

fn heuristic(from: (i32, i32), to: (i32, i32)) -> i32 {
    manhattan_distance(from, to)
}

fn neighbors(node: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = node;
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}
