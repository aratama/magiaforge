use crate::actor::Actor;
use crate::entity::fire::Burnable;
use crate::entity::piece::spawn_broken_piece;
use crate::level::world::LevelScoped;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::BREAK;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
pub struct Bookshelf;

fn break_book_shelf(
    mut commands: Commands,
    registry: Registry,
    query: Query<(&Actor, &Transform, &Burnable, &LevelScoped), With<Bookshelf>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (breakabke, transform, burnable, scope) in query.iter() {
        if breakabke.life <= 0 || burnable.life <= 0 {
            let position = transform.translation.truncate();

            writer.send(SEEvent::pos(BREAK, position));
            for i in 0..6 {
                spawn_broken_piece(
                    &mut commands,
                    &registry,
                    &scope.0,
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
