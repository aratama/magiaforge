use bevy::prelude::*;

use crate::{
    entity::actor::Actor,
    states::{GameState, TimeState},
};

#[derive(Component, Debug)]
pub struct Flip;

fn flip(
    mut parent_query: Query<&Actor, With<Flip>>,
    mut sprite_query: Query<(&Parent, &mut Sprite)>,
) {
    for (parent, mut sprite) in sprite_query.iter_mut() {
        if let Ok(chicken) = parent_query.get_mut(parent.get()) {
            sprite.flip_x = chicken.pointer.x < 0.0;
        }
    }
}

pub struct FlipPlugin;

impl Plugin for FlipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            flip.run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
    }
}
