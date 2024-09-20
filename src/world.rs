use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let x = 400.0 * random::<f32>();
        let y = 400.0 * random::<f32>();
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(x, y, -y),
            texture: asset_server.load("Pixel Art Top Down Basic/TX Plant.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                rect: Some(Rect::new(0.0, 0.0, 8.0 * 19.0, 155.0)),
                ..default()
            },
            ..default()
        });
    }

    let texture_handle: Handle<Image> =
        asset_server.load("Pixel Art Top Down Basic/TX Tileset Grass.png");

    // 1000 x 1000 くらいにするとかなり重い
    let map_size = TilemapSize { x: 64, y: 64 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    // 0～120くらいが原っぱのタイル、それ以降は石畳など
                    // ひとまず原っぱをランダムに配置
                    texture_index: TileTextureIndex(random::<u32>() % 120),
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        // タイルマップは最も背後に配置したいので z: -1000にする
        // サンプルコードでは get_tilemap_center_transform を使って画面中央に揃えているが、
        // 座標計算が面倒になるので x:0, y:0 に配置している
        // また、アンカーがタイルの中央になっているので、タイルの大きさの半分だけずらして原点に揃える
        transform: Transform::from_xyz(tile_size.x * 0.5, tile_size.y * 0.5, -1000.0),
        // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, -1000.0),
        ..Default::default()
    });
}
