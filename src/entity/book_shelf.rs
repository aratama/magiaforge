use super::breakable::Breakable;
use super::EntityDepth;
use crate::command::GameCommand;
use crate::{constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

const ENTITY_WIDTH: f32 = 16.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Default, Component, Reflect)]
pub struct Bookshelf;

/// 指定した位置に本棚を生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_book_shelf(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    commands.spawn((
        Name::new("book_shelf"),
        StateScoped(GameState::InGame),
        Breakable { life: 25 },
        Bookshelf,
        EntityDepth,
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
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
        CollisionGroups::new(WALL_GROUP, ENTITY_GROUP | ACTOR_GROUP | BULLET_GROUP),
    ));
}

fn break_book_shelf(
    mut commands: Commands,
    query: Query<(Entity, &Breakable, &Transform), With<Bookshelf>>,
    mut writer: EventWriter<GameCommand>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            commands.entity(entity).despawn_recursive();
            writer.send(GameCommand::SEKuzureru(Some(
                transform.translation.truncate(),
            )));
        }
    }
}

pub struct BookshelfPlugin;

impl Plugin for BookshelfPlugin {
    fn build(&self, app: &mut App) {
        // ここを FixedUpdate にするとパーティクルの発生位置がおかしくなる
        app.add_systems(
            FixedUpdate,
            break_book_shelf.run_if(in_state(GameState::InGame)),
        );
        app.register_type::<Bookshelf>();
    }
}
