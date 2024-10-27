use super::{map::LevelTileMap, BULLET_GROUP, ENEMY_GROUP, TILE_HALF, TILE_SIZE, WALL_GROUP};
use crate::{states::GameState, world::Tile};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody,
};
use std::collections::HashMap;

/// 壁タイルから衝突矩形を計算します
/// チェストや本棚なども侵入不可能ですが、それらは個別に衝突形状を持つため、ここでは壁のみを扱います
/// TODO: 本棚などのエンティティもここで一括で生成したほうが効率はいい？
/// でもエンティティが個別に削除されることも多そうなので、その場合はエンティティは別のほうがいいかも
/// https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs
pub fn get_wall_collisions(chunk: &LevelTileMap) -> Vec<Rect> {
    let width = chunk.width;
    let height = chunk.height;

    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    // combine wall tiles into flat "plates" in each individual row
    let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

    for y in 0..height {
        let mut row_plates: Vec<Plate> = Vec::new();
        let mut plate_start = None;

        // + 1 to the width so the algorithm "terminates" plates that touch the right edge
        for x in 0..width + 1 {
            match (
                plate_start,
                chunk.get_tile(x as i32, y as i32) == Tile::Wall,
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

        plate_stack.push(row_plates);
    }

    // combine "plates" into rectangles across multiple rows
    let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
    let mut prev_row: Vec<Plate> = Vec::new();
    let mut wall_rects: Vec<Rect> = Vec::new();

    // an extra empty row so the algorithm "finishes" the rects that touch the top edge
    plate_stack.push(Vec::new());

    for (y, current_row) in plate_stack.into_iter().enumerate() {
        for prev_plate in &prev_row {
            if !current_row.contains(prev_plate) {
                // remove the finished rect so that the same plate in the future starts a new rect
                if let Some(rect) = rect_builder.remove(prev_plate) {
                    wall_rects.push(rect);
                }
            }
        }
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct WallCollider;

pub fn respawn_wall_collisions(
    commands: &mut Commands,
    collider_query: &Query<Entity, With<WallCollider>>,
    chunk: &LevelTileMap,
) {
    // 既存の壁コライダーを削除
    for entity in collider_query {
        commands.entity(entity).despawn_recursive();
    }

    // 衝突形状の生成
    for rect in get_wall_collisions(&chunk) {
        let w = TILE_HALF * (rect.width() + 1.0);
        let h = TILE_HALF * (rect.height() + 1.0);
        let x = rect.min.x as f32 * TILE_SIZE + w;
        let y = rect.min.y as f32 * -TILE_SIZE - h;
        commands.spawn((
            WallCollider,
            Name::new("wall collider"),
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
            CollisionGroups::new(WALL_GROUP, ENEMY_GROUP | BULLET_GROUP),
        ));
    }

    commands.insert_resource(chunk.clone());
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, _app: &mut App) {}
}
