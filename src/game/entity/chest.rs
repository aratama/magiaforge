use super::super::{constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

const SLICE_NAME: &str = "chest";

static ENTITY_WIDTH: f32 = 8.0;

static ENTITY_HEIGHT: f32 = 8.0;

#[derive(Default, Component)]
struct Chest;

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_chest(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    let tx = x + ENTITY_WIDTH - TILE_SIZE / 2.0;
    let ty = y - ENTITY_HEIGHT + TILE_SIZE / 2.0;
    let tz = 3.0 + (-ty * Z_ORDER_SCALE);
    commands.spawn((
        Name::new("chest"),
        StateScoped(GameState::InGame),
        Chest,
        AsepriteSliceBundle {
            aseprite: aseprite,
            slice: SLICE_NAME.into(),
            transform: Transform::from_translation(Vec3::new(tx, ty, tz)),
            ..default()
        },
        Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
        CollisionGroups::new(WALL_GROUP, PLAYER_GROUP | ENEMY_GROUP | BULLET_GROUP),
    ));
}
