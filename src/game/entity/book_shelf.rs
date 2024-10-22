use super::super::{constant::*, states::GameState};
use crate::game::{asset::GameAssets, audio::play_se};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

// Asepriteのスライス名
// スライスの原点はAsepriteのpivotで指定します
const SLICE_NAME: &str = "book_shelf";

static BOOKSHELF_ENTITY_WIDTH: f32 = 16.0;

static BOOKSHELF_ENTITY_HEIGHT: f32 = 8.0;

#[derive(Default, Component, Reflect)]
pub struct BookShelf {
    pub life: i32,
}

/// 指定した位置に本棚を生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_book_shelf(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    let z = 3.0 + (-y * Z_ORDER_SCALE);
    commands.spawn((
        Name::new("book_shelf"),
        StateScoped(GameState::InGame),
        BookShelf { life: 8 },
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
            transform: Transform::from_translation(Vec3::new(x, y, z)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(BOOKSHELF_ENTITY_WIDTH, BOOKSHELF_ENTITY_HEIGHT),
        CollisionGroups::new(WALL_GROUP, PLAYER_GROUP | ENEMY_GROUP | BULLET_GROUP),
    ));
}

fn update_book_shelf(
    mut commands: Commands,
    query: Query<(Entity, &BookShelf)>,
    assets: Res<GameAssets>,
) {
    for (entity, book_shelf) in query.iter() {
        if book_shelf.life <= 0 {
            commands.entity(entity).despawn();
            play_se(&mut commands, assets.kuzureru.clone());
        }
    }
}

pub struct BookshelfPlugin;

impl Plugin for BookshelfPlugin {
    fn build(&self, app: &mut App) {
        // ここを FixedUpdate にするとパーティクルの発生位置がおかしくなる
        app.add_systems(
            FixedUpdate,
            update_book_shelf.run_if(in_state(GameState::InGame)),
        );
        app.register_type::<BookShelf>();
    }
}
