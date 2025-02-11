use super::world::LevelScoped;
use crate::collision::WALL_GROUPS;
use crate::collision::WATER_GROUPS;
use crate::constant::*;
use crate::level::chunk::LevelChunk;
use crate::level::tile::Tile;
use crate::registry::Registry;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CoefficientCombineRule;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::Friction;
use bevy_rapier2d::prelude::RigidBody;
use std::collections::HashMap;

/// 壁タイルから衝突矩形を計算します
/// チェストや本棚なども侵入不可能ですが、それらは個別に衝突形状を持つため、ここでは壁のみを扱います
/// https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs
pub fn get_wall_collisions(chunk: &LevelChunk, targets: Vec<Tile>) -> Vec<Rect> {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    // combine wall tiles into flat "plates" in each individual row
    let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

    for y in chunk.bounds.min_y..chunk.bounds.max_y {
        let mut row_plates: Vec<Plate> = Vec::new();
        let mut plate_start = None;

        for x in chunk.bounds.min_x..chunk.bounds.max_x {
            match (
                plate_start,
                targets.contains(&chunk.get_tile(x as i32, y as i32)),
            ) {
                (Some(s), false) => {
                    row_plates.push(Plate {
                        left: s,
                        right: (x - 1) as i32,
                    });
                    plate_start = None;
                }
                (None, true) => plate_start = Some(x as i32),
                _ => (),
            }
        }
        if let Some(s) = plate_start {
            row_plates.push(Plate {
                left: s,
                right: chunk.bounds.max_x - 1,
            });
        }

        plate_stack.push(row_plates);
    }

    // combine "plates" into rectangles across multiple rows
    let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
    let mut prev_row: Vec<Plate> = Vec::new();
    let mut wall_rects: Vec<Rect> = Vec::new();

    // an extra empty row so the algorithm "finishes" the rects that touch the top edge
    plate_stack.push(Vec::new());

    for (plate_index, current_row) in plate_stack.into_iter().enumerate() {
        for prev_plate in &prev_row {
            if !current_row.contains(prev_plate) {
                // remove the finished rect so that the same plate in the future starts a new rect
                if let Some(rect) = rect_builder.remove(prev_plate) {
                    wall_rects.push(rect);
                }
            }
        }

        let y = chunk.bounds.min_y + plate_index as i32;

        for plate in &current_row {
            rect_builder
                .entry(plate.clone())
                .and_modify(|e| e.max.y += 1.0)
                .or_insert(Rect::new(
                    plate.left as f32,
                    y as f32,
                    plate.right as f32,
                    y as f32,
                ));
        }
        prev_row = current_row;
    }

    wall_rects
}

#[derive(Debug, Clone, Eq, PartialEq, Component)]
pub struct WallCollider;

pub fn spawn_wall_collisions(commands: &mut Commands, registry: &Registry, chunk: &LevelChunk) {
    let wall_tiles = registry.get_wall_tiles();

    // 衝突形状の生成
    for rect in get_wall_collisions(&chunk, wall_tiles) {
        let w = TILE_HALF * (rect.width() + 1.0);
        let h = TILE_HALF * (rect.height() + 1.0);
        let x = rect.min.x as f32 * TILE_SIZE + w;
        let y = rect.min.y as f32 * -TILE_SIZE - h;
        commands.spawn((
            Name::new("wall collider"),
            LevelScoped(chunk.level.clone()),
            WallCollider,
            StateScoped(GameState::InGame),
            Transform::from_translation(Vec3::new(x, y, 0.0)),
            GlobalTransform::default(),
            Collider::cuboid(w, h),
            RigidBody::Fixed,
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            *WALL_GROUPS,
        ));
    }

    let surface_tiles = registry.get_surface_tiles();

    for rect in get_wall_collisions(&chunk, surface_tiles) {
        let w = TILE_HALF * (rect.width() + 1.0);
        let h = TILE_HALF * (rect.height() + 1.0);
        let x = rect.min.x as f32 * TILE_SIZE + w;
        let y = rect.min.y as f32 * -TILE_SIZE - h;
        commands.spawn((
            Name::new("water collider"),
            WallCollider,
            LevelScoped(chunk.level.clone()),
            StateScoped(GameState::InGame),
            Transform::from_translation(Vec3::new(x, y, 0.0)),
            GlobalTransform::default(),
            // todo: merge colliders
            Collider::cuboid(w, h),
            RigidBody::Fixed,
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            *WATER_GROUPS,
        ));
    }
}
