use crate::config::GameConfig;
use crate::{asset::GameAssets, audio::play_se};
use crate::{constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

const ENTITY_WIDTH: f32 = 16.0;

const ENTITY_HEIGHT: f32 = 8.0;

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
        BookShelf { life: 25 },
        AsepriteSliceBundle {
            slice: "book_shelf".into(),
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
        Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
        CollisionGroups::new(WALL_GROUP, ENEMY_GROUP | BULLET_GROUP),
    ));
}

fn update_book_shelf(
    mut commands: Commands,
    query: Query<(Entity, &BookShelf)>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for (entity, book_shelf) in query.iter() {
        if book_shelf.life <= 0 {
            commands.entity(entity).despawn_recursive();
            play_se(&audio, &config, assets.kuzureru.clone());
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
