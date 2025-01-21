use crate::asset::GameAssets;
use crate::collision::*;
use crate::component::entity_depth::EntityDepth;
use crate::component::falling::Falling;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::entity::fire::Burnable;
use crate::entity::piece::spawn_broken_piece;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Actor;
use super::ActorGroup;
use super::ActorSpriteGroup;
use super::ActorTypes;

const ENTITY_WIDTH: f32 = 16.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Default, Component, Reflect)]
pub struct Bookshelf;

/// 指定した位置に本棚を生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_book_shelf(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
) -> Entity {
    let aseprite_clone = aseprite.clone();

    let mut parent = commands.spawn((
        Name::new("book_shelf"),
        StateScoped(GameState::InGame),
        Actor {
            actor_type: ActorTypes::BookShelf,
            radius: 8.0,
            actor_group: ActorGroup::Entity,
            ..default()
        },
        Life::new(25),
        Bookshelf,
        Burnable {
            life: 60 * 20 + rand::random::<u32>() % 30,
        },
        EntityDepth::new(),
        Visibility::default(),
        Transform::from_translation(position.extend(0.0)),
        Falling,
        (
            RigidBody::Dynamic,
            Damping {
                linear_damping: 80.0,
                angular_damping: 0.0,
            },
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
            ColliderMassProperties::Density(10.0),
            *ENTITY_GROUPS,
            ExternalImpulse::default(),
        ),
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
    assets: Res<GameAssets>,
    query: Query<(Entity, &Life, &Transform, &Burnable), With<Bookshelf>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform, burnable) in query.iter() {
        if breakabke.life <= 0 || burnable.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
            writer.send(SEEvent::pos(SE::Break, position));
            for i in 0..6 {
                spawn_broken_piece(
                    &mut commands,
                    &assets,
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
