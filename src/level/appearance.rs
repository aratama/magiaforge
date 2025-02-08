use super::world::GameLevel;
use super::world::LevelScoped;
use crate::component::animated_slice::AnimatedSlice;
use crate::constant::*;
use crate::entity::grass::spawn_grasses;
use crate::level::ceil::spawn_autotiles;
use crate::level::chunk::index_to_position;
use crate::level::chunk::LevelChunk;
use crate::level::tile::*;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::registry::TileType;
use crate::registry::Tiling;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use rand::seq::SliceRandom;

const WATER_PLANE_OFFEST: f32 = -4.0;

#[derive(Component)]
pub struct TileSprite(pub (i32, i32));

#[derive(Component, Clone)]
struct FoamTile;

#[derive(Component, Clone)]
struct CeilTile;

/// 床や壁の外観(スプライト)を生成します
pub fn spawn_world_tile(
    mut commands: &mut Commands,
    registry: &Registry,
    world: &GameWorld,
    chunk: &LevelChunk,
    x: i32,
    y: i32,
) {
    let mut rand = rand::thread_rng();
    let tile = world.get_tile(x, y);
    let props = registry.get_tile(&tile);
    match props.tile_type {
        TileType::Wall => {
            spawn_ceil_for_blank(&mut commands, registry, world, &chunk.level, x, y);
        }
        TileType::Surface => {
            // 水辺の岸の壁
            spawn_water_wall(&mut commands, registry, &chunk, x, y);

            for layer in &props.layers {
                match &layer.tiling {
                    Tiling::Auto { prefixes } => {
                        if let Some(frame_prefixes) = prefixes.choose(&mut rand) {
                            spawn_autotiles(
                                &frame_prefixes,
                                &mut commands,
                                registry,
                                &world,
                                &chunk.level,
                                &vec![&tile],
                                WATER_PLANE_OFFEST,
                                x,
                                y,
                                layer.depth,
                                1,
                                &FoamTile,
                            );
                        }
                    }
                    Tiling::Simple { patterns } => {
                        if let Some(slices) = patterns.choose(&mut rand) {
                            if let Some(s) = slices.choose(&mut rand) {
                                let mut builder = commands.spawn((
                                    TileSprite((x, y)),
                                    LevelScoped(chunk.level.clone()),
                                    StateScoped(GameState::InGame),
                                    AseSpriteSlice {
                                        aseprite: registry.assets.atlas.clone(),
                                        name: s.clone(),
                                    },
                                    Transform::from_xyz(
                                        x as f32 * TILE_SIZE,
                                        -y as f32 * TILE_SIZE + WATER_PLANE_OFFEST,
                                        layer.depth,
                                    ),
                                ));
                                if 2 <= slices.len() {
                                    builder.insert(AnimatedSlice {
                                        slices: slices.clone(),
                                        wait: 53,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        TileType::Floor => {
            for layer in &props.layers {
                match &layer.tiling {
                    Tiling::Auto {
                        prefixes: _prefixes,
                    } => {
                        // if let Some(prefix) = prefixes.choose(&mut rand) {
                        //     spawn_autotiles(
                        //         &prefix,
                        //         &mut commands,
                        //         registry,
                        //         &chunk,
                        //         &vec![Tile::new("Water")],
                        //         WATER_PLANE_OFFEST,
                        //         x,
                        //         y,
                        //         WATER_FOAM_LAYER_Z,
                        //         1,
                        //         &FoamTile,
                        //     );
                        // }
                    }
                    Tiling::Simple { patterns } => {
                        if let Some(frames) = patterns.choose(&mut rand) {
                            let mut buidler = commands.spawn((
                                TileSprite((x, y)),
                                LevelScoped(chunk.level.clone()),
                                StateScoped(GameState::InGame),
                                AseSpriteSlice {
                                    aseprite: registry.assets.atlas.clone(),
                                    name: frames[0].clone(),
                                },
                                Transform::from_xyz(
                                    x as f32 * TILE_SIZE,
                                    -y as f32 * TILE_SIZE,
                                    FLOOR_LAYER_Z,
                                ),
                            ));
                            if 2 <= frames.len() {
                                buidler.insert(AnimatedSlice {
                                    slices: frames.clone(),
                                    wait: 53,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    if 0.0 < props.light_intensity && 0.0 < props.light_density {
        if rand::random::<f32>() < props.light_density {
            commands.spawn((
                TileSprite((x, y)),
                LevelScoped(chunk.level.clone()),
                StateScoped(GameState::InGame),
                Transform::from_translation(index_to_position((x, y)).extend(0.0)),
                PointLight2d {
                    intensity: props.light_intensity,
                    color: Color::hsl(
                        props.light_hue,
                        props.light_saturation,
                        props.light_lightness,
                    ),
                    radius: props.light_radius,
                    ..default()
                },
            ));
        }
    }

    if props.grasses {
        if rand::random::<u32>() % 6 != 0 {
            let left_top = Vec2::new(x as f32 * TILE_SIZE, y as f32 * -TILE_SIZE);
            let center = left_top + Vec2::new(TILE_HALF, -TILE_HALF);
            spawn_grasses(&mut commands, &registry, &chunk.level, center);
        }
    }
}

// 水辺の岸の壁
fn spawn_water_wall(
    commands: &mut Commands,
    registry: &Registry,
    chunk: &LevelChunk,
    x: i32,
    y: i32,
) {
    if chunk.is_visible_ceil(
        x,
        y - 1,
        1,
        &vec![
            &Tile::new("StoneTile"),
            &Tile::new("Wall"),
            &Tile::new("PermanentWall"),
        ],
    ) {
        commands.spawn((
            TileSprite((x, y)),
            LevelScoped(chunk.level.clone()),
            StateScoped(GameState::InGame),
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "stone_wall".to_string(),
            },
            Transform::from_xyz(x as f32 * TILE_SIZE, -y as f32 * TILE_SIZE, SHORE_LAYER_Z),
        ));
    }
}

fn spawn_ceil_for_blank(
    commands: &mut Commands,
    registry: &Registry,
    world: &GameWorld,
    level: &GameLevel,
    x: i32,
    y: i32,
) {
    let tx = x as f32 * TILE_SIZE;
    let ty = y as f32 * -TILE_SIZE;
    let tz = ENTITY_LAYER_Z + (ty * Z_ORDER_SCALE);

    // 壁
    if !world.is_wall(&registry, x as i32, y as i32 + 1) {
        commands.spawn((
            TileSprite((x, y)),
            LevelScoped(level.clone()),
            Name::new("wall"),
            StateScoped(GameState::InGame),
            Transform::from_translation(Vec3::new(tx, ty, tz)),
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "high_wall_0".into(),
            },
        ));
    }

    // // 天井
    let wall_tile = Tile::new("Wall");
    let blank_tile = Tile::new("Blank");
    let permanent_wall_tile = Tile::new("PermanentWall");
    let targets = vec![&wall_tile, &blank_tile, &permanent_wall_tile];
    if world.is_visible_ceil(x, y, 3, &targets) {
        spawn_autotiles(
            &vec!["roof".to_string()],
            commands,
            registry,
            &world,
            &level,
            &targets,
            WALL_HEIGHT,
            x,
            y,
            CEIL_LAYER_Z,
            3,
            &CeilTile,
        )
    }
}
