use super::asset::GameAssets;
use super::constant::BULLET_GROUP;
use super::constant::TILE_SIZE;
use super::constant::WALL_GROUP;
use super::constant::Z_ORDER_SCALE;
use super::enemy;
use super::entity::book_shelf::spawn_book_shelf;
use super::entity::chest::spawn_chest;
use super::states::GameState;
use super::wall::get_tile;
use super::wall::get_wall_collisions;
use super::wall::Tile;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;

#[derive(Component)]
struct BGM;

fn setup_world_bgm(mut commands: Commands, asset: Res<GameAssets>, bgm: Query<&BGM>) {
    if bgm.is_empty() {
        commands.spawn((
            BGM,
            AudioBundle {
                source: asset.they.clone(),
                ..default()
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
