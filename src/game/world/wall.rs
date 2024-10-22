use super::{
    map::TileMapChunk, respawn_world, WorldTile, BULLET_GROUP, ENEMY_GROUP, PLAYER_GROUP,
    TILE_HALF, TILE_SIZE, WALL_GROUP,
};
use crate::game::{asset::GameAssets, audio::play_se, states::GameState, world::tile::Tile};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_rapier2d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody,
};
use std::collections::HashMap;

/// 壁タイルから衝突矩形を計算します
/// チェストや本棚なども侵入不可能ですが、それらは個別に衝突形状を持つため、ここでは壁のみを扱います
/// TODO: 本棚などのエンティティもここで一括で生成したほうが効率はいい？
/// でもエンティティが個別に削除されることも多そうなので、その場合はエンティティは別のほうがいいかも
/// https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs
pub fn get_wall_collisions(chunk: &TileMapChunk) -> Vec<Rect> {
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
    chunk: &TileMapChunk,
) {
    // 既存の壁コライダーを削除
    for entity in collider_query.iter() {
        commands.entity(entity).despawn();
    }

    // 衝突形状の生成
    for rect in get_wall_collisions(&chunk) {
        let w = TILE_HALF * (rect.width() + 1.0);
        let h = TILE_HALF * (rect.height() + 1.0);
        let x = rect.min.x as f32 * TILE_SIZE + w - TILE_HALF;
        let y = rect.min.y as f32 * -TILE_SIZE - h + TILE_HALF;
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
            CollisionGroups::new(WALL_GROUP, PLAYER_GROUP | ENEMY_GROUP | BULLET_GROUP),
        ));
    }

    commands.insert_resource(chunk.clone());
}

#[derive(Event)]
pub struct BreakWallEvent {
    /// 破壊の起点となった座標
    /// 例えば弾丸の当たった位置など
    pub position: Vec2,
}

fn process_break_wall_event(
    mut break_wall_events: EventReader<BreakWallEvent>,
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    assets: Res<GameAssets>,
    collider_query: Query<Entity, With<WallCollider>>,
    mut chunk: ResMut<TileMapChunk>,
    world_tile: Query<Entity, With<WorldTile>>,
) {
    let mut rebuild = false;

    for event in break_wall_events.read() {
        rebuild = true;

        // 格子点との距離をXとYそれぞれで計算し、どちらの方向が壁なのかを判定して、
        // 壁のあるほうを破壊します
        // TODO 壁が壊せないときがあって、このあたりのコードがおかしい
        // let x = event.position.x / TILE_SIZE;
        // let y = -event.position.y / TILE_SIZE;
        // let near_x = (x - x.round()).abs();
        // let near_y = (y - y.round()).abs();
        // if near_x < near_y {
        //     chunk.set_tile(x.round() as i32 - 1, y.floor() as i32 + 0, Tile::StoneTile);
        //     chunk.set_tile(x.round() as i32 + 0, y.floor() as i32 + 0, Tile::StoneTile);
        // } else {
        //     chunk.set_tile(x.floor() as i32 + 1, y.round() as i32 - 1, Tile::StoneTile);
        //     chunk.set_tile(x.floor() as i32 + 1, y.round() as i32 + 0, Tile::StoneTile);
        // }

        // 仕方ないので、弾丸の周囲の壁のうち最も近いものを破壊する
        let x = event.position.x / TILE_SIZE as f32;
        let y = -event.position.y / TILE_SIZE as f32;
        let mut tile_list = Vec::<Vec2>::new();
        for dy in 0..3 {
            for dx in 0..3 {
                let tx = (x + dx as f32) as i32;
                let ty = (y + dy as f32) as i32;
                let tile = chunk.get_tile(tx, ty);
                if tile == Tile::Wall {
                    tile_list.push(Vec2::new(
                        TILE_SIZE * tx as f32 + TILE_HALF,
                        TILE_SIZE * ty as f32 + TILE_HALF,
                    ));
                }
            }
        }
        if !tile_list.is_empty() {
            tile_list.sort_by(|a, b| {
                let dist_a = (*a - event.position).length_squared();
                let dist_b = (*b - event.position).length_squared();
                dist_a.partial_cmp(&dist_b).unwrap()
            });
            let rx = tile_list[0].x / TILE_SIZE as f32;
            let ry = tile_list[0].y / TILE_SIZE as f32;
            chunk.set_tile(rx as i32, ry as i32, Tile::StoneTile);
        }
    }

    break_wall_events.clear();

    // 壁のダメージ蓄積を実装するまでは確率的に壊れるようにする
    if rebuild {
        respawn_world(
            &mut commands,
            level_aseprites,
            &assets,
            collider_query,
            &chunk,
            &world_tile,
        );

        play_se(&mut commands, assets.kuzureru.clone());
    }
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BreakWallEvent>().add_systems(
            FixedUpdate,
            process_break_wall_event.run_if(in_state(GameState::InGame)),
        );
    }
}
