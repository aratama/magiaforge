use crate::entity::life::{Life, LifeBeingSprite};
use crate::entity::EntityDepth;
use crate::se::{SEEvent, SE};
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
    let aseprite_clone = aseprite.clone();

    let mut parent = commands.spawn((
        Name::new("book_shelf"),
        StateScoped(GameState::InGame),
        Life {
            life: 25,
            max_life: 25,
            amplitude: 0.0,
        },
        Bookshelf,
        EntityDepth,
        Transform::from_translation(Vec3::new(x, y, 0.0)),
        GlobalTransform::default(),
        InheritedVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
        CollisionGroups::new(
            WALL_GROUP,
            ENTITY_GROUP
                | WITCH_GROUP
                | WITCH_BULLET_GROUP
                | ENEMY_GROUP
                | ENEMY_BULLET_GROUP
                | RABBIT_GROUP,
        ),
    ));

    parent.with_children(move |parent| {
        parent.spawn((
            LifeBeingSprite,
            AseSpriteSlice {
                name: "book_shelf".to_string(),
                aseprite: aseprite_clone,
            },
        ));
    });
}

fn break_book_shelf(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform), With<Bookshelf>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(SE::Break, transform.translation.truncate()));
        }
    }
}

pub struct BookshelfPlugin;

impl Plugin for BookshelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            break_book_shelf
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Bookshelf>();
    }
}
