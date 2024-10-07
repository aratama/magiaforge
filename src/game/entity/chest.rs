use super::setup::set_aseprite_and_z;
use app::LdtkEntityAppExt;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::*;
use bevy_rapier2d::prelude::*;
use std::sync::LazyLock;

const ASEPRITE_PATH: &str = "asset.aseprite";

const SLICE_NAME: &str = "chest";

const ENTITY_ID: &str = "Chest";

static COLLIDER: LazyLock<Collider> = LazyLock::new(|| Collider::cuboid(4.0, 4.0));

#[derive(Default, Component)]
struct Chest;

#[derive(Bundle, LdtkEntity)]
struct ChestBundle {
    name: Name,
    chest: Chest,
    aseprite_slice_bundle: AsepriteSliceBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    rigit_body: RigidBody,
    collider: Collider,
}

impl Default for ChestBundle {
    fn default() -> Self {
        Self {
            name: Name::new("chest"),
            chest: Chest,
            aseprite_slice_bundle: AsepriteSliceBundle {
                slice: SLICE_NAME.into(),
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
    level_query: Query<&Transform, (With<LevelIid>, Without<Chest>)>,
    layer_query: Query<&Parent, With<LayerMetadata>>,
    query: Query<(&mut Handle<Aseprite>, &mut Transform, &Parent), With<Chest>>,
) {
    set_aseprite_and_z::<Chest>(ASEPRITE_PATH, asset_server, level_query, layer_query, query);
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_aseprite);
        app.register_ldtk_entity::<ChestBundle>(ENTITY_ID);
    }
}
