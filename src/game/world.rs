use super::asset::GameAssets;
use super::constant::*;
use super::enemy;
use super::entity::book_shelf::spawn_book_shelf;
use super::entity::chest::spawn_chest;
use super::overlay::OverlayNextState;
use super::states::GameState;
use super::tile::*;
use super::wall::*;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

fn setup_world(
    mut commands: Commands,
    level_aseprites: Res<Assets<Aseprite>>,
    images: Res<Assets<Image>>,
    assets: Res<GameAssets>,
) {
    let level_aseprite = level_aseprites.get(assets.level.id()).unwrap();
    let level_image = images.get(level_aseprite.atlas_image.id()).unwrap();

    let asset_aseprite = level_aseprites.get(assets.asset.id()).unwrap();
    let asset_image = images.get(asset_aseprite.atlas_image.id()).unwrap();

    let stone_tile_slice_index =
        slice_to_tile_texture_index(asset_aseprite, asset_image, "stone tile");

    let black_slice_index = slice_to_tile_texture_index(asset_aseprite, asset_image, "black");

    let map_size = TilemapSize {
        x: level_image.width(),
        y: level_image.height(),
    };

    let floor_layer_entity = commands.spawn_empty().id();
    let roof_layer_entity = commands.spawn_empty().id();

    let mut floor_layer_storage = TileStorage::empty(map_size);
    let mut roof_layer_storage = TileStorage::empty(map_size);

    for y in 0..level_image.height() {
        for x in 0..level_image.width() {
            let tile_pos = TilePos {
                x,
                y: map_size.y - y - 1,
            };

            match get_tile(level_image, x as i32, y as i32) {
                Tile::StoneTile => {
                    // タイルマップの個々のタイルの生成
                    let tile_entity = commands
                        .spawn((
                            Name::new("stone_tile"),
                            TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(floor_layer_entity),
                                texture_index: stone_tile_slice_index,
                                ..default()
                            },
                        ))
                        .id();
                    floor_layer_storage.set(&tile_pos, tile_entity);
                }
                Tile::Wall => {
                    let tx = x as f32 * TILE_SIZE;
                    let ty = y as f32 * -TILE_SIZE;
                    let tz = ENTITY_LAYER_Z + (-ty * Z_ORDER_SCALE);

                    // 壁
                    if get_tile(level_image, x as i32, y as i32 + 1) == Tile::StoneTile {
                        commands.spawn((
                            Name::new("wall"),
                            StateScoped(GameState::InGame),
                            AsepriteSliceBundle {
                                aseprite: assets.asset.clone(),
                                slice: "stone wall".into(),
                                transform: Transform::from_translation(Vec3::new(tx, ty - 4.0, tz)),
                                ..default()
                            },
                        ));
                    }

                    // 天井
                    if get_tile(level_image, x as i32, y as i32 - 1) == Tile::StoneTile {
                        let tile_entity = commands
                            .spawn((
                                Name::new("roof tile"),
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(roof_layer_entity),
                                    texture_index: black_slice_index,
                                    ..default()
                                },
                            ))
                            .id();
                        roof_layer_storage.set(&tile_pos, tile_entity);
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

    // タイルマップ本体の生成
    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(floor_layer_entity).insert((
        Name::new("floor layer tilemap"),
        StateScoped(GameState::InGame),
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: floor_layer_storage,
            texture: TilemapTexture::Single(asset_aseprite.atlas_image.clone()),
            tile_size,
            // transformを計算するのにはget_tilemap_center_transform という関数もありますが、
            // それだとそこにタイルマップの中心が来てしまうことに注意します
            // Asepriteの座標系とはYが反転していることもあり、ここでは自力でTransformを計算しています
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -TILE_SIZE * (map_size.y - 1) as f32,
                FLOOR_LAYER_Z,
            )),
            ..default()
        },
    ));

    commands.entity(roof_layer_entity).insert((
        Name::new("roof layer tilemap"),
        StateScoped(GameState::InGame),
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: roof_layer_storage,
            texture: TilemapTexture::Single(asset_aseprite.atlas_image.clone()),
            tile_size,
            transform: Transform::from_translation(Vec3::new(
                0.0,
                // 天井レイヤーは8.0だけ上にずらしていることに注意
                -TILE_SIZE * (map_size.y - 1) as f32 + WALL_HEIGHT,
                ROOF_LAYER_Z,
            )),
            ..default()
        },
    ));

    // 衝突形状の生成
    for rect in get_wall_collisions(&level_image) {
        let w = TILE_HALF * (rect.width() + 1.0);
        let h = TILE_HALF * (rect.height() + 1.0);
        let x = rect.min.x as f32 * TILE_SIZE + w - TILE_HALF;
        let y = rect.min.y as f32 * -TILE_SIZE - h + TILE_HALF;
        commands.spawn((
            Name::new("wall collider"),
            StateScoped(GameState::InGame),
            Transform::from_translation(Vec3::new(x, y, 0.0)),
            GlobalTransform::default(),
            // todo: merge colliders
            Collider::cuboid(w, h),
            RigidBody::Fixed,
            Friction::new(1.0),
            CollisionGroups::new(WALL_GROUP, PLAYER_GROUP | ENEMY_GROUP | BULLET_GROUP),
        ));
    }
}

fn update_world(
    enemy_query: Query<&enemy::Enemy>,
    mut overlay_next_state: ResMut<OverlayNextState>,
) {
    if enemy_query.is_empty() {
        *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
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

/// スライス名からタイルマップのインデックスを計算します
fn slice_to_tile_texture_index(
    asset_aseprite: &Aseprite,
    asset_atlas: &Image,
    slice: &str,
) -> TileTextureIndex {
    let asset_tile_size = asset_atlas.width() / TILE_SIZE as u32;
    let stone_tile_slice = asset_aseprite.slices.get(slice).unwrap();
    let stone_tile_slice_index = TileTextureIndex(
        asset_tile_size * (stone_tile_slice.rect.min.y / TILE_SIZE) as u32
            + (stone_tile_slice.rect.min.x / TILE_SIZE) as u32,
    );
    return stone_tile_slice_index;
}
