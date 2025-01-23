use crate::component::animated_slice::AnimatedSlice;
use crate::constant::*;
use crate::entity::grass::spawn_grasses;
use crate::level::ceil::spawn_autotiles;
use crate::level::ceil::WALL_HEIGHT_IN_TILES;
use crate::level::map::image_to_tilemap;
use crate::level::map::LevelChunk;
use crate::level::tile::*;
use crate::page::in_game::GameLevel;
use crate::registry::Registry;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

const WATER_PLANE_OFFEST: f32 = -4.0;

#[derive(Component)]
pub struct TileSprite(pub (i32, i32));

/// レベルの番号を指定して、画像データからレベル情報を取得します
/// このとき、該当するレベルの複数のスライスからランダムにひとつが選択されます、
pub fn read_level_chunk_data(
    registry: &Registry,
    level_aseprites: &Res<Assets<Aseprite>>,
    images: &Res<Assets<Image>>,

    level: GameLevel,
    mut rng: &mut StdRng,
    biome_tile: Tile,
) -> LevelChunk {
    let level_aseprite = level_aseprites.get(registry.assets.level.id()).unwrap();
    let level_image = images.get(level_aseprite.atlas_image.id()).unwrap();

    let level_slice = match level {
        GameLevel::Level(level) => {
            let keys = level_aseprite
                .slices
                .keys()
                .filter(|s| s.starts_with(&format!("level_{}_", level)));
            keys.choose(&mut rng).unwrap()
        }
        GameLevel::MultiPlayArena => "multiplay_arena",
    };

    let slice = level_aseprite.slices.get(level_slice).unwrap();

    let chunk = image_to_tilemap(
        biome_tile,
        &level_image,
        slice.rect.min.x as i32,
        slice.rect.max.x as i32,
        slice.rect.min.y as i32,
        slice.rect.max.y as i32,
    );

    return chunk;
}

#[derive(Component)]
struct FoamTile;

#[derive(Component)]
struct CeilmTile;

/// 床や壁の外観(スプライト)を生成します
pub fn spawn_world_tilemap(commands: &mut Commands, registry: &Registry, chunk: &LevelChunk) {
    // 床と壁の生成
    for y in chunk.min_y..(chunk.max_y + WALL_HEIGHT_IN_TILES as i32) {
        for x in chunk.min_x..chunk.max_x {
            spawn_world_tile(commands, registry, chunk, x, y);
        }
    }
}

/// 床や壁の外観(スプライト)を生成します
pub fn spawn_world_tile(
    mut commands: &mut Commands,
    registry: &Registry,
    chunk: &LevelChunk,
    x: i32,
    y: i32,
) {
    match chunk.get_tile(x, y) {
        Tile::StoneTile => {
            spawn_stone_tile(&mut commands, registry, x, y);
        }
        Tile::Wall => {
            spawn_ceil_for_blank(&mut commands, registry, chunk, x, y);
        }
        Tile::PermanentWall => {
            spawn_ceil_for_blank(&mut commands, registry, chunk, x, y);
        }
        Tile::Grassland => {
            spawn_grassland(&mut commands, registry, x, y);
        }
        Tile::Blank => {
            spawn_ceil_for_blank(&mut commands, registry, chunk, x, y);
        }
        Tile::Water => {
            // 水辺の岸の壁
            spawn_water_wall(&mut commands, registry, &chunk, x, y);

            // 岸にできる泡
            spawn_autotiles(
                &vec!["water_form_0".to_string(), "water_form_1".to_string()],
                &mut commands,
                registry,
                &chunk,
                &vec![Tile::Water],
                WATER_PLANE_OFFEST,
                x,
                y,
                WATER_FOAM_LAYER_Z,
                1,
                FoamTile,
                FoamTile,
                FoamTile,
                FoamTile,
            );

            // 網状の泡の明るいほう
            let index = rand::random::<u32>() % 2;
            commands.spawn((
                TileSprite((x, y)),
                AnimatedSlice {
                    slices: (0..4)
                        .map(|i| format!("water_mesh_lighter_{}_{}", index, i))
                        .collect(),
                    wait: 53,
                },
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: format!("water_mesh_lighter_{}_0", index).to_string(),
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    -y as f32 * TILE_SIZE + WATER_PLANE_OFFEST,
                    WATER_MESH_LIGHTER_LAYER_Z,
                ),
            ));

            // 網状の泡
            commands.spawn((
                TileSprite((x, y)),
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: format!("water_mesh_{}", rand::random::<u32>() % 2).to_string(),
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    -y as f32 * TILE_SIZE + WATER_PLANE_OFFEST,
                    WATER_MESH_DARKER_LAYER_Z,
                ),
            ));
        }
        Tile::Lava => {
            // 水辺の岸の壁
            spawn_water_wall(&mut commands, registry, &chunk, x, y);

            let mut builder = commands.spawn((
                TileSprite((x, y)),
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: format!("lava_mesh_{}", rand::random::<u32>() % 2).to_string(),
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    -y as f32 * TILE_SIZE + WATER_PLANE_OFFEST,
                    WATER_MESH_DARKER_LAYER_Z,
                ),
            ));

            if rand::random::<u32>() % 6 == 0 {
                builder.insert(PointLight2d {
                    intensity: 0.4,
                    color: Color::hsl(22.0, 0.5, 0.5),
                    radius: TILE_SIZE * 4.0,
                    ..default()
                });
            }
        }
        Tile::Soil => {
            commands.spawn((
                TileSprite((x, y)),
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: "soil_tile".to_string(),
                },
                Transform::from_xyz(x as f32 * TILE_SIZE, -y as f32 * TILE_SIZE, FLOOR_LAYER_Z),
            ));
        }
        Tile::Crack => {
            spawn_water_wall(&mut commands, registry, &chunk, x, y);
        }
        Tile::Ice => {
            spawn_water_wall(&mut commands, registry, &chunk, x, y);
            spawn_ice_floor(&mut commands, registry, x, y);
        }
    };
}

fn spawn_ice_floor(commands: &mut Commands, registry: &Registry, x: i32, y: i32) {
    commands.spawn((
        TileSprite((x, y)),
        AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
            name: "tile_ice".to_string(),
        },
        Transform::from_xyz(
            x as f32 * TILE_SIZE,
            -y as f32 * TILE_SIZE + WATER_PLANE_OFFEST,
            WATER_MESH_DARKER_LAYER_Z,
        ),
    ));
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
        &vec![Tile::StoneTile, Tile::Wall, Tile::PermanentWall],
    ) {
        commands.spawn((
            TileSprite((x, y)),
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "stone_wall".to_string(),
            },
            Transform::from_xyz(x as f32 * TILE_SIZE, -y as f32 * TILE_SIZE, SHORE_LAYER_Z),
        ));
    }
}

fn spawn_stone_tile(commands: &mut Commands, registry: &Registry, x: i32, y: i32) {
    let r = rand::random::<u32>() % 3;
    let slice = format!("stone_tile{}", r);
    commands.spawn((
        TileSprite((x, y)),
        Name::new("stone_tile"),
        StateScoped(GameState::InGame),
        Transform::from_translation(Vec3::new(
            x as f32 * TILE_SIZE,
            y as f32 * -TILE_SIZE,
            FLOOR_LAYER_Z,
        )),
        AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
            name: slice.into(),
        },
    ));
}

fn spawn_ceil_for_blank(
    commands: &mut Commands,
    registry: &Registry,
    chunk: &LevelChunk,
    x: i32,
    y: i32,
) {
    let tx = x as f32 * TILE_SIZE;
    let ty = y as f32 * -TILE_SIZE;
    let tz = ENTITY_LAYER_Z + (ty * Z_ORDER_SCALE);

    // 壁
    if !chunk.is_wall(x as i32, y as i32 + 1) {
        commands.spawn((
            TileSprite((x, y)),
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
    let targets = vec![Tile::Wall, Tile::Blank, Tile::PermanentWall];
    if chunk.is_visible_ceil(x, y, 3, &targets) {
        spawn_autotiles(
            &vec!["roof".to_string()],
            commands,
            registry,
            &chunk,
            &targets,
            WALL_HEIGHT,
            x,
            y,
            CEIL_LAYER_Z,
            3,
            CeilmTile,
            CeilmTile,
            CeilmTile,
            CeilmTile,
        )
    }
}

fn spawn_grassland(mut commands: &mut Commands, registry: &Registry, x: i32, y: i32) {
    let left_top = Vec2::new(x as f32 * TILE_SIZE, y as f32 * -TILE_SIZE);
    commands.spawn((
        TileSprite((x, y)),
        Name::new("grassland"),
        StateScoped(GameState::InGame),
        Transform::from_translation(left_top.extend(FLOOR_LAYER_Z)),
        AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
            name: "grassland".into(),
        },
    ));

    if rand::random::<u32>() % 6 != 0 {
        let center = left_top + Vec2::new(TILE_HALF, -TILE_HALF);
        spawn_grasses(&mut commands, &registry, center);
    }
}
