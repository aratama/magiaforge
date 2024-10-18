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
use bevy_rapier2d::prelude::*;

fn setup_world(
    mut commands: Commands,
    level: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
) {
    let level_handle = assets.level.clone();
    if let Some(level) = level.get(level_handle.id()) {
        if let Some(img) = images.get(level.atlas_image.id()) {
            for y in 0..img.height() {
                for x in 0..img.width() {
                    match get_tile(img, x as i32, y as i32) {
                        Tile::Empty => {
                            commands.spawn((
                                StateScoped(GameState::InGame),
                                SpriteBundle {
                                    texture: assets.tile.clone(),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                        rect: Some(Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        x as f32 * TILE_SIZE,
                                        y as f32 * -TILE_SIZE,
                                        0.0,
                                    )),
                                    ..Default::default()
                                },
                            ));
                        }
                        Tile::Wall => {
                            let tx = x as f32 * TILE_SIZE;
                            let ty = y as f32 * -TILE_SIZE;
                            let tz = 3.0 + (-ty * Z_ORDER_SCALE);

                            // 壁
                            if get_tile(img, x as i32, y as i32 + 1) == Tile::Empty {
                                commands.spawn((
                                    StateScoped(GameState::InGame),
                                    SpriteBundle {
                                        texture: assets.tile.clone(),
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(TILE_SIZE, 8.0)),
                                            rect: Some(Rect::new(
                                                TILE_SIZE * 3.0,
                                                TILE_SIZE * 0.0,
                                                TILE_SIZE * 4.0,
                                                TILE_SIZE * 0.5,
                                            )),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            tx,
                                            ty - 4.0,
                                            tz,
                                        )),
                                        ..Default::default()
                                    },
                                ));
                            }

                            // 天井
                            if get_tile(img, x as i32, y as i32 - 1) == Tile::Empty {
                                commands.spawn((
                                    StateScoped(GameState::InGame),
                                    SpriteBundle {
                                        texture: assets.tile.clone(),
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                            rect: Some(Rect::new(
                                                TILE_SIZE * 1.0,
                                                TILE_SIZE * 3.0,
                                                TILE_SIZE * 2.0,
                                                TILE_SIZE * 4.0,
                                            )),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            tx,
                                            ty + 8.0,
                                            tz + 1.0,
                                        )),
                                        ..Default::default()
                                    },
                                ));
                            }
                        }
                        Tile::BookShelf => {
                            spawn_book_shelf(
                                &mut commands,
                                assets.asset.clone(),
                                TILE_SIZE * x as f32,
                                TILE_SIZE * -1.0 * y as f32,
                            );
                        }
                        Tile::Chest => {
                            spawn_chest(
                                &mut commands,
                                assets.asset.clone(),
                                TILE_SIZE * x as f32,
                                TILE_SIZE * -1.0 * y as f32,
                            );
                        }
                        _ => {}
                    }
                }
            }

            for rect in get_wall_collisions(&img) {
                let w = 8.0 * (rect.width() + 1.0);
                let h = 8.0 * (rect.height() + 1.0);
                let x = rect.min.x as f32 * TILE_SIZE + w - 8.0;
                let y = rect.min.y as f32 * -TILE_SIZE - h + 8.0;
                commands.spawn((
                    StateScoped(GameState::InGame),
                    Transform::from_translation(Vec3::new(x, y, 0.0)),
                    GlobalTransform::default(),
                    // todo: merge colliders
                    Collider::cuboid(w, h),
                    RigidBody::Fixed,
                    Friction::new(1.0),
                    CollisionGroups::new(WALL_GROUP, BULLET_GROUP),
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
