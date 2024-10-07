use super::setup::set_aseprite_and_z;
use app::LdtkEntityAppExt;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::*;
use bevy_rapier2d::prelude::*;
use std::sync::LazyLock;

// Asepriteのファイルパス
const ASEPRITE_PATH: &str = "asset.aseprite";

// Asepriteのスライス名
// スライスの原点はAsepriteのpivotで指定します
const SLICE_NAME: &str = "book_shelf";

// LDTKでのEntity名
const ENTITY_ID: &str = "Book_Shelf";

// repierでの衝突形状
static COLLIDER: LazyLock<Collider> = LazyLock::new(|| Collider::cuboid(16.0, 8.0));

#[derive(Default, Component)]
struct BookShelf;

#[derive(Bundle, LdtkEntity)]
struct BookShelfBundle {
    name: Name,
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
            name: Name::new("book_shelf"),
            book_shelf: BookShelf,
            aseprite_slice_bundle: AsepriteSliceBundle {
                slice: SLICE_NAME.into(),
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
            collider: COLLIDER.clone(),
        }
    }
}

fn set_aseprite(
    asset_server: Res<AssetServer>,
    level_query: Query<&Transform, (With<LevelIid>, Without<BookShelf>)>,
    layer_query: Query<&Parent, With<LayerMetadata>>,
    query: Query<(&mut Handle<Aseprite>, &mut Transform, &Parent), With<BookShelf>>,
) {
    set_aseprite_and_z::<BookShelf>(ASEPRITE_PATH, asset_server, level_query, layer_query, query);
}

pub struct BookShelfPlugin;

impl Plugin for BookShelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_aseprite);
        app.register_ldtk_entity::<BookShelfBundle>(ENTITY_ID);
    }
}
