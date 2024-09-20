use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Default, Component)]
pub struct Tree;

#[derive(Default, Bundle, LdtkEntity)]
pub struct TreeBundle {
    tree: Tree,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    rigit_body: RigidBody,
    collider: Collider,
}

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkEntity)]
pub struct WallBundle {
    wall: Wall,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    rigit_body: RigidBody,
    collider: Collider,
}

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Pixel Art Top Down Basic/basic.ldtk"),
        // タイルマップは最も背後に配置したいので z: -1000にする
        // サンプルコードでは get_tilemap_center_transform を使って画面中央に揃えているが、
        // 座標計算が面倒になるので x:0, y:0 に配置している
        // また、アンカーがタイルの中央になっているので、タイルの大きさの半分だけずらして原点に揃える
        transform: Transform::from_xyz(0.0, 0.0, -1000.0),
        ..Default::default()
    });
    commands.spawn((
        Collider::cuboid(400.0, 20.0),
        TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)),
    ));
}

pub fn update_world(
    mut tree_query: Query<
        (&mut Sprite, &mut RigidBody, &mut Collider, &mut Transform),
        (With<Tree>, Without<Wall>),
    >,
    mut wall_query: Query<(&mut RigidBody, &mut Collider), With<Wall>>,
    // layer_query: Query<&LayerMetadata>,
    // tile_query: Query<&Parent>,
) {
    for s in &mut tree_query {
        let (mut sprite, mut body, mut collider, mut transform) = s;
        sprite.anchor = bevy::sprite::Anchor::Custom(Vec2::new(0.12, -0.42));
        *body = RigidBody::Fixed;
        *collider = Collider::ball(5.0);

        // TreeはLdtkWorldBundleの子であり、LdtkWorldBundleのzが-1000になっているので、
        // 1000を足したところが座標になる
        transform.translation.z = 1000.0 - transform.translation.y;
    }

    for s in &mut wall_query {
        let (mut body, mut collider) = s;
        *body = RigidBody::Fixed;

        // (8.0, 8.0)でエンティティの大きさにちょうど合致する
        *collider = Collider::cuboid(8.0, 8.0);
    }

    // for layer in layer_query.iter() {
    //     //
    //     console::log_1(&JsValue::from(layer.identifier.clone()));
    // }

    // for my_tile_parent in tile_query.iter() {
    //     let layer_for_this_tile = layer_query.get(my_tile_parent.get());
    //     match layer_for_this_tile {
    //         Ok(layer) => {
    //             console::log_1(&JsValue::from(layer.identifier.clone()));
    //         }
    //         Err(e) => {
    //             // console::log_1(&JsValue::from("error"));
    //         }
    //     }
    // }
}
