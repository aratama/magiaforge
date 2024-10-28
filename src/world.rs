use super::asset::GameAssets;
use super::entity::witch::spawn_witch;
use super::entity::witch::WitchType;
use super::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;

fn setup_world(mut commands: Commands, assets: Res<GameAssets>, frame_count: Res<FrameCount>) {
    spawn_witch(
        &mut commands,
        &assets,
        -50.0,
        0.0,
        WitchType::PlayerWitch,
        *frame_count,
        30,
        150,
    );
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);
    }
}
