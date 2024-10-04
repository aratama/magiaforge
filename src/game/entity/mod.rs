pub mod book_shelf;
pub mod chest;

use crate::game::constant::*;
use bevy::{asset::AssetPath, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::*;

pub fn set_aseprite_and_z<'a, T: Component>(
    aseprite_path: impl Into<AssetPath<'a>>,
    asset_server: Res<AssetServer>,
    level_query: Query<&Transform, (With<LevelIid>, Without<T>)>,
    layer_query: Query<&Parent, With<LayerMetadata>>,
    mut query: Query<(&mut Handle<Aseprite>, &mut Transform, &Parent), With<T>>,
) {
    let asset = asset_server.load(aseprite_path);
    for (mut aseprite, mut transform, parent) in query.iter_mut() {
        *aseprite = asset.clone();

        // Entity階層の階層は以下のようになっています
        //
        // ldtkproject
        //     level0 (ldtk_level)
        //         Ground (ldtk_layer)
        //         Entities (ldtk_layer)
        //             book_shelf0 (ldtk_entity)
        //             book_shelf1 (ldtk_entity)
        //
        // ここで、ldtk_level に Transform が設定されており、
        // ldtk_levelのTransformとldtk_entityのTransformを加算したところが、
        // ワールドでのldtk_entityの位置になります
        // 重ね合わせ順序を設定するため、このワールド上のy座標を計算してからそれを利用してz座標を設定しています
        // ここで、ldtk_entityからldtk_levelをたどるには２回親をたどる必要があることに注意します
        // https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/anatomy-of-the-world.html
        if let Ok(layer_parent) = layer_query.get(parent.get()) {
            if let Ok(level_transform) = level_query.get(layer_parent.get()) {
                // ldtk_entityのワールド上のy座標
                let y = level_transform.translation.y + transform.translation.y;
                // zを設定します
                // 前後のレイヤーは1づつ増加するので、前後のレイヤーのzを超えてしまわないように Z_ORDER_SCALE を乗算しています
                transform.translation.z = -y * Z_ORDER_SCALE;
            }
        }
    }
}
