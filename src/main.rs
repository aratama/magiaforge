mod book_shelf;
mod camera;
mod console;
mod hud;
mod ldtk_util;
mod overlay;
mod player;
mod serialize;
mod tree;
mod wall;
mod world;

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use book_shelf::BookShelfBundle;
use book_shelf::*;
use camera::*;
use hud::*;
use iyes_perf_ui::prelude::*;
use overlay::OverlayPlugin;
use player::*;
use serialize::*;
use tree::*;
use wall::*;
use world::*;

fn main() {
    let player_data = restore_player();
    let player_data_for_camera = player_data.clone();

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
        enabled: true,
        mode: DebugRenderMode::COLLIDER_SHAPES,
        ..default()
    })
    .add_plugins(BevySprityPlugin)
    // ここではひとまず level 0 を読み込んでいますが、
    // 実際には起動直後にプレイヤーの位置に応じて読み込むレベルを切り替わります
    // 一瞬プレイヤーの周囲が真っ暗に見えるので、最初から正しいレベルを読み込むようにしたほうがいいかも
    .insert_resource(LevelSelection::index(0))
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        ..default()
    })
    .register_ldtk_entity::<TreeBundle>("Tree")
    .register_ldtk_entity::<BookShelfBundle>("Book_Shelf")
    .add_systems(Update, set_book_shelf_aseprite)
    .add_plugins(LdtkPlugin)
    .add_plugins(HudPlugin)
    .add_plugins(OverlayPlugin)
    .add_systems(Startup, setup_autosave_timer)
    .add_systems(Update, spawn_autosave_timer)
    .add_systems(Startup, move |commands: Commands| {
        setup_camera(commands, &player_data_for_camera);
    })
    .add_systems(Update, update_camera)
    .add_plugins(WorldPlugin)
    .add_plugins(TreePlugin)
    .add_systems(PostStartup, {
        move |commands: Commands, asset_server: Res<AssetServer>| {
            setup_player(commands, asset_server, &player_data);
        }
    })
    .add_systems(Update, update_player)
    .add_systems(Update, spawn_wall_collision)
    .register_ldtk_int_cell::<WallBundle>(1)
    .register_ldtk_int_cell::<WallBundle>(3);

    app.run();
}
