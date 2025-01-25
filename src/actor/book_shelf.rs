use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorSpriteGroup;
use crate::actor::LifeBeingSprite;
use crate::collision::*;
use crate::component::falling::Falling;
use crate::constant::TILE_HALF;
use crate::entity::fire::Burnable;
use crate::entity::piece::spawn_broken_piece;
use crate::registry::Registry;
use crate::se::SEEvent;

use crate::se::BREAK;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use vleue_navigator::prelude::PrimitiveObstacle;

const ENTITY_HALF_WIDTH: f32 = TILE_HALF * 2.0;

const ENTITY_HALF_HEIGHT: f32 = TILE_HALF;

#[derive(Default, Component, Reflect)]
pub struct Bookshelf;

pub fn default_bookshelf() -> Actor {
    Actor {
        extra: ActorExtra::BookShelf,
        actor_group: ActorGroup::Entity,
        life: 25,
        max_life: 25,
        ..default()
    }
}

/// 指定した位置に本棚を生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_book_shelf(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    actor: Actor,
) -> Entity {
    let aseprite_clone = aseprite.clone();

    let mut parent = commands.spawn((
        Name::new("book_shelf"),
        actor,
        Bookshelf,
        Burnable {
            life: 60 * 20 + rand::random::<u32>() % 30,
        },
        Transform::from_translation(position.extend(0.0)),
        Falling,
        (
            RigidBody::Dynamic,
            Damping {
                linear_damping: 80.0,
                angular_damping: 0.0,
            },
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(ENTITY_HALF_WIDTH, ENTITY_HALF_HEIGHT),
            ColliderMassProperties::Density(10.0),
            *ENTITY_GROUPS,
            ExternalImpulse::default(),
        ),
        PrimitiveObstacle::Rectangle(Rectangle::new(
            ENTITY_HALF_WIDTH * 2.0,
            ENTITY_HALF_HEIGHT * 2.0,
        )),
    ));

    parent.with_children(move |parent| {
        parent.spawn(ActorSpriteGroup).with_child((
            LifeBeingSprite,
            AseSpriteSlice {
                name: "book_shelf".to_string(),
                aseprite: aseprite_clone,
            },
        ));
    });

    parent.id()
}

fn break_book_shelf(
    mut commands: Commands,
    registry: Registry,
    query: Query<(&Actor, &Transform, &Burnable), With<Bookshelf>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (breakabke, transform, burnable) in query.iter() {
        if breakabke.life <= 0 || burnable.life <= 0 {
            let position = transform.translation.truncate();

            writer.send(SEEvent::pos(BREAK, position));
            for i in 0..6 {
                spawn_broken_piece(
                    &mut commands,
                    &registry,
                    position,
                    &format!("bookshelf_piece_{}", i),
                );
            }
        }
    }
}

pub struct BookshelfPlugin;

impl Plugin for BookshelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            break_book_shelf.in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Bookshelf>();
    }
}
