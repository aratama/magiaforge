use crate::component::vertical::Vertical;
use crate::page::in_game::LevelSetup;
use crate::se::{SEEvent, SE};
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

/// 水面、亀裂などの床に落ちた場合にdespawnするコンポーネントを表します
/// Verticalと併用します
#[derive(Component, Debug)]
#[require(Vertical)]
pub struct Falling;

fn despawn(
    mut commands: Commands,
    level: Res<LevelSetup>,
    query: Query<(Entity, &Transform, Option<&Vertical>, Option<&Name>), With<Falling>>,
    mut se: EventWriter<SEEvent>,
) {
    if let Some(ref chunk) = level.chunk {
        for (entity, transform, vertical, name) in query.iter() {
            let position = transform.translation.truncate();
            let tile = chunk.get_tile_by_coords(position);
            if !tile.is_plane() && vertical.map(|v| v.v == 0.0).unwrap_or(true) {
                commands.entity(entity).despawn_recursive();

                se.send(SEEvent::pos(SE::Scene2, position));
                info!(
                    "[falling] {:?} falled into {:?}",
                    name.unwrap_or(&Name::new("(no name)")),
                    tile
                );
            }
        }
    }
}
pub struct FallingPlugin;

impl Plugin for FallingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, despawn.in_set(FixedUpdateGameActiveSet));
    }
}
