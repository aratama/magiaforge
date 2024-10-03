use bevy::prelude::*;
use bevy_ecs_ldtk::{LayerMetadata, LdtkWorldBundle};

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk"),
        // タイルマップは最も背後に配置したいので z: -1000にする
        // サンプルコードでは get_tilemap_center_transform を使って画面中央に揃えているが、
        // 座標計算が面倒になるので x:0, y:0 に配置している
        // また、アンカーがタイルの中央になっているので、タイルの大きさの半分だけずらして原点に揃える
        // transform: Transform::from_xyz(0.0, 0.0, -1000.0),
        ..Default::default()
    });
    // commands.spawn((
    //     Collider::cuboid(400.0, 20.0),
    //     TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)),
    // ));
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}
