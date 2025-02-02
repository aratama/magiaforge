use crate::actor::Actor;
use crate::level::tile::Tile;
use crate::level::world::GameWorld;
use crate::se::SEEvent;
use crate::se::SCENE2;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

fn despawn(
    mut commands: Commands,
    level: Res<GameWorld>,
    query: Query<(Entity, &Transform, &Actor, Option<&Name>)>,
    mut se: EventWriter<SEEvent>,
) {
    for (entity, transform, actor, name) in query.iter() {
        let position = transform.translation.truncate();
        let tile = level.get_tile_by_coords(position);
        if actor.v <= 0.0 && tile == Tile::new("Crack") {
            commands.entity(entity).despawn_recursive();

            se.send(SEEvent::pos(SCENE2, position));
            info!(
                "[falling] {:?} falled into {:?}",
                name.unwrap_or(&Name::new("(no name)")),
                tile
            );
        }
    }
}
pub struct FallingPlugin;

impl Plugin for FallingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, despawn.in_set(FixedUpdateGameActiveSet));
    }
}
