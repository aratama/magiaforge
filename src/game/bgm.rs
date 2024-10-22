use super::asset::GameAssets;
use super::states::GameState;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

#[cfg(not(feature = "debug"))]
const BGM_VOLUME: f32 = 0.2;

#[cfg(feature = "debug")]
const BGM_VOLUME: f32 = 0.0;

#[derive(Component)]
struct BGM;

fn setup_world_bgm(mut commands: Commands, asset: Res<GameAssets>, bgm: Query<&BGM>) {
    if bgm.is_empty() {
        commands.spawn((
            BGM,
            AudioBundle {
                source: asset.they.clone(),
                settings: PlaybackSettings {
                    volume: Volume::new(BGM_VOLUME),
                    mode: PlaybackMode::Loop,
                    ..default()
                },
            },
        ));
    }
}

pub struct BGMPlugin;

impl Plugin for BGMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world_bgm);
    }
}
