use super::bullet::BulletPlugin;
use super::camera::*;
use super::close_on_esc::close_on_esc;
use super::embedded::EmbeddedAssetPlugin;
use super::entity::book_shelf::*;
use super::entity::chest::*;
use super::hud::*;
use super::overlay::*;
use super::player::setup_player;
use super::player::update_player;
use super::serialize::*;
use super::tree::*;
use super::wall::spawn_wall_collision;
use super::wall::WallBundle;
use super::world::*;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::diagnostic::*;
// use bevy::log::Level;
// use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use iyes_perf_ui::prelude::*;

pub fn run_game() {
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
        //
        // .set(LogPlugin {
        //     level: Level::DEBUG,
        //     ..default()
        // })
    )
    .add_plugins(EmbeddedAssetPlugin)
    .add_plugins(WorldInspectorPlugin::new())
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
    .add_plugins(BevySprityPlugin)
    // ここではひとまず level 0 を読み込んでいますが、
    // 実際には起動直後にプレイヤーの位置に応じて読み込むレベルを切り替わります
    // 一瞬プレイヤーの周囲が真っ暗に見えるので、最初から正しいレベルを読み込むようにしたほうがいいかも
    .insert_resource(LevelSelection::index(0))
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        // デフォルトではInt LayerにもLDTKと同様の色が描画される
        // デバッグ中以外はInvisibleにしておく
        int_grid_rendering: IntGridRendering::Invisible,
        ..default()
    })
    .register_ldtk_entity::<TreeBundle>("Tree")
    .add_plugins(BookShelfPlugin)
    .add_plugins(ChestPlugin)
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
    .register_ldtk_int_cell::<WallBundle>(3)
    .add_systems(Update, close_on_esc)
    .add_plugins(BulletPlugin)
    .add_plugins(ParticleSystemPlugin);

    app.run();
}
