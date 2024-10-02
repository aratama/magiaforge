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
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..default()
                },
                // ここでは asepriteを読み込めないことに注意
                // set_book_shelf_asepriteであとから設定する
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
    mut query: Query<&mut Handle<Aseprite>, Added<BookShelf>>,
    asset_server: Res<AssetServer>,
) {
    for mut aseprite in query.iter_mut() {
        *aseprite = asset_server.load("asset.aseprite");
    }
}
