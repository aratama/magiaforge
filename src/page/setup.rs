use crate::constant::LOADING_Z_INDEX;
use crate::states::GameState;
use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d::default(), StateScoped(GameState::Setup)));
    commands.spawn((
        StateScoped(GameState::Setup),
        ImageNode {
            image: asset_server.load("image/loading.png"),
            ..default()
        },
        // 最初は Overlay で暗くなっているので loading は一番手前にする
        GlobalZIndex(LOADING_Z_INDEX),
    ));
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
