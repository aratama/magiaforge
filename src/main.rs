use bevy::{
    asset::{AssetMetaCheck, AssetPlugin},
    prelude::*,
    render::camera::CameraRenderGraph,
};

use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct HUD;

fn main() {
    console::log_1(&JsValue::from("start"));

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // https://github.com/bevyengine/bevy/issues/10157
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(TilemapPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, update);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // デフォルトでは far: 1000, near: -1000でカメラが作成される
    // この範囲を超えるとクリップされることに注意
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;

    commands.spawn(camera_bundle);

    commands.spawn((
        TextBundle::from_section("Test", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
        HUD,
    ));

    for _ in 0..10 {
        let x = 400.0 * rand::random::<f32>();
        let y = 400.0 * rand::random::<f32>();
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

    commands.spawn((
        Person,
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture: asset_server.load("Pixel Art Top Down Basic/TX Player.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                rect: Some(Rect::new(0.0, 0.0, 32.0, 58.0)),
                ..default()
            },
            ..default()
        },
    ));

    let texture_handle: Handle<Image> =
        asset_server.load("Pixel Art Top Down Basic/TX Tileset Grass.png");

    let map_size = TilemapSize { x: 256, y: 256 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    // 0～120くらいが原っぱのタイル、それ以降は石畳など
                    // ひとまず原っぱをランダムに配置
                    texture_index: TileTextureIndex(rand::random::<u32>() % 120),
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

    // fill_tilemap_rect_color(
    //     TileTextureIndex(0),
    //     TilePos { x: 0, y: 0 },
    //     TilemapSize { x: 1, y: 1 },
    //     Color::srgba(1.0, 0.0, 0.0, 1.0),
    //     TilemapId(tilemap_entity),
    //     &mut commands,
    //     &mut tile_storage,
    // );

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
        // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, -1000.0),
        transform: Transform::from_xyz(0.0, 0.0, -1000.0),
        ..Default::default()
    });
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Person>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Person>)>,
    mut hud_query: Query<&mut Text, With<HUD>>,
) {
    let speed = 3.0;

    let velocity = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero()
        * speed;

    let mut player = player_query.single_mut();

    player.translation.x += velocity.x;
    player.translation.y += velocity.y;
    player.translation.z = -player.translation.y;

    let mut camera = camera_query.single_mut();

    camera.translation.x += (player.translation.x - camera.translation.x) * 0.1;
    camera.translation.y += (player.translation.y - camera.translation.y) * 0.1;

    let mut hud = hud_query.single_mut();

    let text = format!(
        "Player: ({:.2}, {:.2})\nCamera: ({:.2}, {:.2})",
        player.translation.x, player.translation.y, camera.translation.x, camera.translation.y
    );

    hud.sections = vec![TextSection::from(text)];
}

fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}
