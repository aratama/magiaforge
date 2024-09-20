mod camera;
mod hud;
mod player;
mod world;

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::*;
use hud::*;
use iyes_perf_ui::prelude::*;
use player::*;
use wasm_bindgen::JsValue;
use web_sys::console;
use world::*;

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
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(EntityCountDiagnosticsPlugin)
    .add_plugins(SystemInformationDiagnosticsPlugin)
    .add_plugins(PerfUiPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugins(RapierDebugRenderPlugin {
        enabled: false,
        mode: DebugRenderMode::COLLIDER_SHAPES,
        ..default()
    })
    .insert_resource(LevelSelection::index(0))
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
        ..default()
    })
    .register_ldtk_entity::<TreeBundle>("Tree")
    .add_plugins(LdtkPlugin)
    .add_systems(Startup, setup_hud)
    .add_systems(Startup, setup_player)
    .add_systems(Startup, setup_world)
    .add_systems(Startup, setup_camera)
    .add_systems(Update, update_player)
    .add_systems(Update, update_camera)
    .add_systems(Update, update_hud)
    .add_systems(Update, update_world);

    app.run();
}
