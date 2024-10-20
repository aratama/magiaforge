use super::asset::GameAssets;
use super::bgm::BGMPlugin;
use super::bullet::BulletPlugin;
use super::camera::*;
use super::close_on_esc::close_on_esc;
use super::constant::INITIAL_STATE;
use super::constant::PIXELS_PER_METER;
use super::embedded::EmbeddedAssetPlugin;
use super::enemy::EnemyPlugin;
use super::entity::book_shelf::BookshelfPlugin;
use super::hud::*;
use super::life_bar::LifeBarPlugin;
use super::main_menu::*;
use super::overlay::*;
use super::player::*;
use super::serialize::*;
use super::states::*;
use super::world::*;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
#[cfg(feature = "debug")]
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
#[cfg(feature = "debug")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "debug")]
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::Cursor;
use bevy::window::EnabledButtons;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
#[cfg(feature = "debug")]
use iyes_perf_ui::PerfUiPlugin;

pub fn run_game() {
    let mut app = App::new();

    app
        //
        // デフォルトのプラグインや依存しているサードパーティのプラグインなど
        //
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // https://github.com/bevyengine/bevy/issues/10157
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        cursor: Cursor {
                            icon: CursorIcon::Crosshair,
                            ..default()
                        },
                        title: "Magia Gene 0.1".to_string(),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            close: true,
                            maximize: false,
                            minimize: true,
                        },
                        focused: true,
                        ..default()
                    }),
                    ..default()
                }),
            //
            // .set(LogPlugin {
            //     level: Level::DEBUG,
            //     ..default()
            // })
        )
        .add_plugins(EmbeddedAssetPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(BevySprityPlugin)
        .add_plugins(ParticleSystemPlugin)
        .add_plugins(Light2dPlugin)
        //
        // 以下はこのゲーム本体で定義されたプラグイン
        //
        .add_plugins(HudPlugin)
        .add_plugins(OverlayPlugin)
        .add_systems(Startup, setup_autosave_timer)
        .add_systems(FixedUpdate, spawn_autosave_timer)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(FixedUpdate, close_on_esc)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StartPagePlugin)
        .add_plugins(BGMPlugin)
        .add_plugins(LifeBarPlugin)
        .add_plugins(BookshelfPlugin)
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
        .add_sub_state::<MainMenuPhase>()
        .add_loading_state(
            LoadingState::new(GameState::Setup)
                .continue_to_state(INITIAL_STATE)
                .load_collection::<GameAssets>(),
        )
        // State Scoped Entities をオンにすることで、
        // stateを変更したときに自動的にエンティティを削除できます
        // https://bevyengine.org/news/bevy-0-14/#state-scoped-entities
        .enable_state_scoped_entities::<GameState>();

    // bevy_light_2d のプラグインはwasm32向けには対応していません
    // https://github.com/jgayfer/bevy_light_2d/issues/5
    // https://github.com/jgayfer/bevy_light_2d/issues/6
    // https://github.com/jgayfer/bevy_light_2d/pull/7

    //
    // 以下はデバッグ用のプラグインなど
    // 無くてもゲーム事態は動作します
    //
    #[cfg(feature = "debug")]
    app.add_plugins(PerfUiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            mode: DebugRenderMode::COLLIDER_SHAPES,
            ..default()
        });

    app.run();
}
