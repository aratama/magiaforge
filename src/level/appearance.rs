use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::grass::spawn_grasses;
use crate::level::biome::Biome;
use crate::level::ceil::spawn_roof_tiles;
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

/// 床や壁の外観(スプライト)を生成します
pub fn spawn_level_appearance(
    mut commands: &mut Commands,
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

    info!(
        "bounds min_x:{} max_x:{} min_y:{} max_y:{}",
        slice.rect.min.x, slice.rect.max.x, slice.rect.min.y, slice.rect.max.y
    );

    let chunk = image_to_tilemap(
        &level_image,
        slice.rect.min.x as i32,
        slice.rect.max.x as i32,
        slice.rect.min.y as i32,
        slice.rect.max.y as i32,
    );

    spawn_world_tilemap(
        &mut commands,
        &assets,
        &chunk,
        // バイオームをハードコーディングしているけどこれでいい？
        match level {
            GameLevel::Level(2) => Biome::Grassland,
            _ => Biome::StoneTile,
        },
    );

    return chunk;
}

fn spawn_world_tilemap(
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
            }
        }
    }
}

fn spawn_stone_tile(commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
    let r = rand::random::<u32>() % 3;
    let slice = format!("stone_tile{}", r);
    commands.spawn((
        WorldTile,
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
    let tz = ENTITY_LAYER_Z + (-ty * Z_ORDER_SCALE);

    // 壁
    if !chunk.equals(x as i32, y as i32 + 1, Tile::Wall) {
        commands.spawn((
            WorldTile,
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
    if chunk.is_visible_ceil(x, y) {
        spawn_roof_tiles(commands, assets, &chunk, x, y)
    }
}

fn spawn_grassland(mut commands: &mut Commands, assets: &Res<GameAssets>, x: i32, y: i32) {
    let left_top = Vec2::new(x as f32 * TILE_SIZE, y as f32 * -TILE_SIZE);
    commands.spawn((
        WorldTile,
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
