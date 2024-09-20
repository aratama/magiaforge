use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let x = 400.0 * random::<f32>();
        let y = 400.0 * random::<f32>();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, -y),
                texture: asset_server.load("Pixel Art Top Down Basic/TX Plant.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    rect: Some(Rect::new(0.0, 0.0, 8.0 * 19.0, 155.0)),
                    ..default()
                },
                ..default()
            },
            RigidBody::Fixed,
            Collider::ball(5.0),
            Restitution::coefficient(0.7),
            GravityScale(0.0),
        ));
    }

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Pixel Art Top Down Basic/basic.ldtk"),
        // タイルマップは最も背後に配置したいので z: -1000にする
        // サンプルコードでは get_tilemap_center_transform を使って画面中央に揃えているが、
        // 座標計算が面倒になるので x:0, y:0 に配置している
        // また、アンカーがタイルの中央になっているので、タイルの大きさの半分だけずらして原点に揃える
        transform: Transform::from_xyz(8.0, 8.0, -1000.0),
        ..Default::default()
    });

    commands.spawn((
        Collider::cuboid(400.0, 20.0),
        TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)),
    ));
}
