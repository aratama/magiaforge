use crate::states::GameState;
use crate::states::TimeState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

#[derive(Debug, Clone, Component)]
pub struct AnimatedSlice {
    pub slices: Vec<String>,
    pub wait: u32,
}

fn update(mut query: Query<(&AnimatedSlice, &mut AseSpriteSlice)>, frame_count: Res<FrameCount>) {
    for (animated, mut sprite) in query.iter_mut() {
        let frame_index = (frame_count.0 / animated.wait) % animated.slices.len() as u32;
        sprite.name = animated.slices[frame_index as usize].clone();
    }
}

pub struct AnimatedSlicePlugin;

impl Plugin for AnimatedSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update.run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
    }
}
