use super::asset::GameAssets;
use super::states::GameState;
use bevy::audio::Volume;
use bevy::prelude::*;

#[derive(Component)]
struct BGM;

fn setup_world_bgm(mut commands: Commands, asset: Res<GameAssets>, bgm: Query<&BGM>) {
    if bgm.is_empty() {
        commands.spawn((
            BGM,
            AudioBundle {
                source: asset.they.clone(),
                settings: PlaybackSettings {
                    volume: Volume::new(0.5),
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
