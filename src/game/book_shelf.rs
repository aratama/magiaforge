use crate::game::constant::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
pub struct BookShelf;

#[derive(Bundle, LdtkEntity)]
pub struct BookShelfBundle {
    book_shelf: BookShelf,
    aseprite_slice_bundle: AsepriteSliceBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    rigit_body: RigidBody,
    collider: Collider,
}

impl Default for BookShelfBundle {
    fn default() -> Self {
        Self {
            book_shelf: BookShelf,
            aseprite_slice_bundle: AsepriteSliceBundle {
                slice: "book_shelf".into(),
                sprite: Sprite {
                    // ここでanchorを設定しても反映されないことに注意
                    // Aseprite側でスライスごとに pivot を設定することができるようになっており、
                    // pivotが指定されている場合はそれが比率に変換されて anchor に設定されます
                    // pivotが指定されていない場合は Center になります
                    // https://github.com/Lommix/bevy_aseprite_ultra/blob/dc57882c8d3023e6879a29332ad42c6ddcf56380/src/loader.rs#L59
                    // anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
                // ここでは asset serverからasepriteを読み込めないことに注意
                // set_book_shelf_asepriteであとから設定します
                // aseprite: asset_server.load("asset.aseprite"),
                ..default()
            },
            grid_coords: GridCoords::default(),
            rigit_body: RigidBody::Fixed,
            collider: Collider::cuboid(16.0, 8.0),
        }
    }
}

pub fn set_book_shelf_aseprite(
    mut query: Query<(&mut Handle<Aseprite>, &mut Transform), Added<BookShelf>>,
    asset_server: Res<AssetServer>,
) {
    for (mut aseprite, mut transform) in query.iter_mut() {
        *aseprite = asset_server.load("asset.aseprite");

        // https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/anatomy-of-the-world.html
        transform.translation.z = (256.0 - transform.translation.y) * Z_ORDER_SCALE;
        // println!(
        //     "book shelf  y:{} z:{}",
        //     transform.translation.y, transform.translation.z
        // );
    }
}
