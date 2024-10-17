use super::bullet::BulletPlugin;
use super::camera::*;
use super::close_on_esc::close_on_esc;
use super::constant::INITIAL_STATE;
use super::embedded::EmbeddedAssetPlugin;
use super::enemy::EnemyPlugin;
use super::entity::book_shelf::*;
use super::entity::chest::*;
use super::hud::*;
use super::overlay::*;
use super::player::*;
use super::serialize::*;
use super::start::*;
use super::states::*;
use super::wall::spawn_wall_collision;
use super::wall::WallBundle;
use super::world::*;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use iyes_perf_ui::prelude::*;

pub fn run_game() {
    let mut app = App::new();

    app
        //
        // デフォルトのプラグインや依存しているサードパーティのプラグインなど
        //
        .add_plugins(
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
        .add_plugins(TilemapPlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            mode: DebugRenderMode::COLLIDER_SHAPES,
            ..default()
        })
        .add_plugins(BevySprityPlugin)
        .add_plugins(ParticleSystemPlugin)
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
        //
        // 以下はデバッグ用のプラグインなど
        // 無くてもゲーム事態は動作します
        //
        .add_plugins(PerfUiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(Light2dPlugin)
        //
        // 以下はこのゲーム本体で定義されたプラグイン
        //
        .add_plugins(BookShelfPlugin)
        .add_plugins(ChestPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(OverlayPlugin)
        .add_systems(Startup, setup_autosave_timer)
        .add_systems(FixedUpdate, spawn_autosave_timer)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(FixedUpdate, spawn_wall_collision)
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<WallBundle>(3)
        .add_systems(FixedUpdate, close_on_esc)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StartPagePlugin)
        //
        // 全体の初期化をするsystem
        // カメラなど、最初の画面に関わらず必要な初期化はここで行っています
        //
        .add_systems(OnEnter(GameState::Setup), |mut commands: Commands| {
            // セーブデータの選択
            // 現在はデバッグ用
            let player_data = restore_player();
            commands.insert_resource(player_data);

            // カメラの初期化
            setup_camera(commands);
        })
        //
        // メインメニューやゲームプレイ画面などのシーンを定義するstate
        //
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Setup)
                .continue_to_state(INITIAL_STATE)
                .load_collection::<AsepriteAssets>(),
        )
        // State Scoped Entities をオンにすることで、
        // stateを変更したときに自動的にエンティティを削除できます
        // https://bevyengine.org/news/bevy-0-14/#state-scoped-entities
        .enable_state_scoped_entities::<GameState>()
        //
        // 開始
        //
        .run();
}
