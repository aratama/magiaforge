use super::super::{constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::LazyLock;

// Asepriteのスライス名
// スライスの原点はAsepriteのpivotで指定します
const SLICE_NAME: &str = "book_shelf";

static ENTITY_WIDTH: f32 = 16.0;

static ENTITY_HEIGHT: f32 = 8.0;

// repierでの衝突形状
static COLLIDER: LazyLock<Collider> =
    LazyLock::new(|| Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT));

#[derive(Default, Component)]
struct BookShelf;

pub fn spawn_book_shelf(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    let tx = x + ENTITY_WIDTH - TILE_SIZE / 2.0;
    let ty = y - ENTITY_HEIGHT + TILE_SIZE / 2.0;
    let tz = 3.0 + (-ty * Z_ORDER_SCALE);
    commands.spawn((
        Name::new("book_shelf"),
        StateScoped(GameState::InGame),
        BookShelf,
        AsepriteSliceBundle {
            slice: SLICE_NAME.into(),
            aseprite: aseprite,
            sprite: Sprite {
                // ここでanchorを設定しても反映されないことに注意
                // Aseprite側でスライスごとに pivot を設定することができるようになっており、
                // pivotが指定されている場合はそれが比率に変換されて anchor に設定されます
                // pivotが指定されていない場合は Center になります
                // https://github.com/Lommix/bevy_aseprite_ultra/blob/dc57882c8d3023e6879a29332ad42c6ddcf56380/src/loader.rs#L59
                // anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(tx, ty, tz)),
            ..default()
        },
        RigidBody::Fixed,
        COLLIDER.clone(),
        CollisionGroups::new(WALL_GROUP, PLAYER_GROUP | ENEMY_GROUP | BULLET_GROUP),
    ));
}
