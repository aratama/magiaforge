use bevy::math::{Rect, Vec2};
use bevy_ecs_ldtk::prelude::*;

pub fn find_level(ldtk_project: &LdtkProject, point_in_pixel: Vec2) -> Option<LevelIid> {
    let point = Vec2::new(point_in_pixel.x / 16.0, -point_in_pixel.y / 16.0); // ldtkではy軸が逆

    for level in ldtk_project.iter_raw_levels() {
        let x_by_tiles = level.world_x / 16;
        let y_by_tiles = level.world_y / 16;

        if let Some(ref layers) = level.layer_instances {
            for layer in layers.iter() {
                let bounds = Rect::new(
                    x_by_tiles as f32,
                    y_by_tiles as f32,
                    x_by_tiles as f32 + layer.c_hei as f32,
                    y_by_tiles as f32 + layer.c_wid as f32,
                );
                if bounds.contains(point) {
                    return Some(LevelIid::new(level.iid.clone()));
                }
            }
        }
    }

    None
}
