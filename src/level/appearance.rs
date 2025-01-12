use crate::asset::GameAssets;
use crate::component::animated_slice::AnimatedSlice;
use crate::constant::*;
use crate::entity::grass::spawn_grasses;
use crate::level::biome::Biome;
use crate::level::ceil::spawn_autotiles;
use crate::level::ceil::WALL_HEIGHT_IN_TILES;
use crate::level::map::image_to_tilemap;
use crate::level::map::LevelChunk;
use crate::level::tile::*;
use crate::page::in_game::GameLevel;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

#[derive(Component)]
pub struct TileSprite(pub (i32, i32));

/// レベルの番号を指定して、画像データからレベル情報を取得します
/// このとき、該当するレベルの複数のスライスからランダムにひとつが選択されます、
pub fn read_level_chunk_data(
    level_aseprites: &Res<Assets<Aseprite>>,
    images: &Res<Assets<Image>>,
    assets: &Res<GameAssets>,
    level: GameLevel,
    mut rng: &mut StdRng,
) -> LevelChunk {
    let level_aseprite = level_aseprites.get(assets.level.id()).unwrap();
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
pub fn spawn_world_tilemap(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk: &LevelChunk,
    biome: Biome,
) {
    // 床と壁の生成
    for y in chunk.min_y..(chunk.max_y + WALL_HEIGHT_IN_TILES as i32) {
        for x in chunk.min_x..chunk.max_x {
            match chunk.get_tile(x, y) {
                Tile::Biome => match biome {
                    Biome::StoneTile => {
                        spawn_stone_tile(commands, assets, x, y);
                    }
                    Biome::Grassland => {
                        spawn_grassland(commands, &assets, x, y);
                    }
                },
                Tile::StoneTile => {
                    spawn_stone_tile(commands, assets, x, y);
                }
                Tile::Wall => {
                    spawn_ceil_for_blank(commands, assets, chunk, x, y);
                }
                Tile::Grassland => {
                    spawn_grassland(commands, &assets, x, y);
                }
                Tile::Blank => {
                    spawn_ceil_for_blank(commands, assets, chunk, x, y);
                }
                Tile::Water => {
                    // 水辺の岸の壁
                    if chunk.is_visible_ceil(x, y - 1, 1, Tile::StoneTile, Tile::Biome)
                        || chunk.is_visible_ceil(x, y - 1, 1, Tile::Wall, Tile::Wall)
                    {
                        commands.spawn((
                            TileSprite((x, y)),
                            AseSpriteSlice {
                                aseprite: assets.atlas.clone(),
                                name: "stone_wall".to_string(),
                            },
                            Transform::from_xyz(
                                x as f32 * TILE_SIZE,
                                -y as f32 * TILE_SIZE,
                                SHORE_LAYER_Z,
                            ),
                        ));
                    }

                    const WATER_PLANE_OFFEST: f32 = -4.0;

                    // 岸にできる泡
                    spawn_autotiles(
                        &vec!["water_form_0".to_string(), "water_form_1".to_string()],
                        commands,
                        assets,
                        &chunk,
                        Tile::Water,
                        Tile::Water,
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
                            aseprite: assets.atlas.clone(),
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
                            aseprite: assets.atlas.clone(),
                            name: format!("water_mesh_{}", rand::random::<u32>() % 2).to_string(),
                        },
                        Transform::from_xyz(
                            x as f32 * TILE_SIZE,
                            -y as f32 * TILE_SIZE + WATER_PLANE_OFFEST,
                            WATER_MESH_DARKER_LAYER_Z,
                        ),
                    ));
                }
            }
        }
    }
}

fn spawn_stone_tile(commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
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
            aseprite: assets.atlas.clone(),
            name: slice.into(),
        },
    ));
}

fn spawn_ceil_for_blank(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk: &LevelChunk,
    x: i32,
    y: i32,
) {
    let tx = x as f32 * TILE_SIZE;
    let ty = y as f32 * -TILE_SIZE;
    let tz = ENTITY_LAYER_Z + (ty * Z_ORDER_SCALE);

    // 壁
    if !chunk.equals(x as i32, y as i32 + 1, Tile::Wall) {
        commands.spawn((
            TileSprite((x, y)),
            Name::new("wall"),
            StateScoped(GameState::InGame),
            Transform::from_translation(Vec3::new(tx, ty, tz)),
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "high_wall_0".into(),
            },
        ));
    }

    // // 天井
    if chunk.is_visible_ceil(x, y, 3, Tile::Wall, Tile::Blank) {
        spawn_autotiles(
            &vec!["roof".to_string()],
            commands,
            assets,
            &chunk,
            Tile::Wall,
            Tile::Blank,
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

fn spawn_grassland(mut commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
    let left_top = Vec2::new(x as f32 * TILE_SIZE, y as f32 * -TILE_SIZE);
    commands.spawn((
        TileSprite((x, y)),
        Name::new("grassland"),
        StateScoped(GameState::InGame),
        Transform::from_translation(left_top.extend(FLOOR_LAYER_Z)),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "grassland".into(),
        },
    ));

    if rand::random::<u32>() % 6 != 0 {
        let center = left_top + Vec2::new(TILE_HALF, -TILE_HALF);
        spawn_grasses(&mut commands, &assets, center);
    }
}
