use super::enemy;
use super::states::GameState;
use super::wall::get_tile;
use super::wall::get_wall_collisions;
use super::wall::Tile;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::Friction;
use bevy_rapier2d::prelude::RigidBody;
use web_sys::js_sys::WebAssembly::Global;

#[derive(AssetCollection, Resource)]
pub struct AsepriteAssets {
    #[asset(path = "level.aseprite")]
    level: Handle<Aseprite>,

    #[asset(path = "tile.png")]
    tile: Handle<Image>,
}

const BLANK_TILE: [u8; 4] = [0, 0, 0, 0];
const WALL_TILE: [u8; 4] = [203, 219, 252, 255];
const EMPTY_TILE: [u8; 4] = [82, 75, 36, 255];

fn setup_world(
    mut commands: Commands,
    level: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    aseprite_assets: Res<AsepriteAssets>,
) {
    let level_handle = aseprite_assets.level.clone();
    if let Some(level) = level.get(level_handle.id()) {
        if let Some(img) = images.get(level.atlas_image.id()) {
            for y in 0..img.height() {
                for x in 0..img.width() {
                    match get_tile(img, x, y) {
                        Tile::Empty => {
                            commands.spawn((
                                StateScoped(GameState::InGame),
                                SpriteBundle {
                                    texture: aseprite_assets.tile.clone(),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(16.0, 16.0)),
                                        rect: Some(Rect::new(0.0, 0.0, 16.0, 16.0)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        x as f32 * 16.0,
                                        y as f32 * -16.0,
                                        0.0,
                                    )),
                                    ..Default::default()
                                },
                            ));
                        }
                        Tile::Wall => {
                            commands.spawn((
                                StateScoped(GameState::InGame),
                                SpriteBundle {
                                    texture: aseprite_assets.tile.clone(),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(16.0, 16.0)),
                                        rect: Some(Rect::new(0.0, 16.0 * 3.0, 16.0, 16.0 * 4.0)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        x as f32 * 16.0,
                                        y as f32 * -16.0,
                                        0.0,
                                    )),
                                    ..Default::default()
                                },
                                // todo: merge colliders
                                // Collider::cuboid(8.0, 8.0),
                                // RigidBody::Fixed,
                                // Friction::new(1.0),
                            ));
                        }
                        _ => {}
                    }
                }
            }

            for rect in get_wall_collisions(&img) {
                println!("rect: {:?}", rect);
                let w = 8.0 * (rect.width() + 1.0);
                let h = 8.0 * (rect.height() + 1.0);
                let x = rect.min.x as f32 * 16.0 + w - 8.0;
                let y = rect.min.y as f32 * -16.0 - h + 8.0;
                commands.spawn((
                    StateScoped(GameState::InGame),
                    Transform::from_translation(Vec3::new(x, y, 0.0)),
                    GlobalTransform::default(),
                    // todo: merge colliders
                    Collider::cuboid(w, h),
                    RigidBody::Fixed,
                    Friction::new(1.0),
                ));
            }
        }
    } else {
        println!("level not found");
    }
}

fn update_world(
    enemy_query: Query<&enemy::Enemy>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if enemy_query.is_empty() {
        next_game_state.set(GameState::MainMenu);
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);
        app.add_systems(
            FixedUpdate,
            update_world.run_if(in_state(GameState::InGame)),
        );
    }
}
