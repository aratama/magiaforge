use crate::actor::Actor;
use crate::entity::piece::spawn_broken_piece;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::BREAK;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
struct StoneLantern;

fn break_stone_lantern(
    mut commands: Commands,
    registry: Registry,
    query: Query<(&Actor, &Transform), With<StoneLantern>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();

            writer.send(SEEvent::pos(BREAK, position));

            for i in 0..4 {
                spawn_broken_piece(
                    &mut commands,
                    &registry,
                    position,
                    &format!("stone_lantern_piece_{}", i),
                );
            }
        }
    }
}

pub struct StoneLanternPlugin;

impl Plugin for StoneLanternPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            break_stone_lantern.in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<StoneLantern>();
    }
}
