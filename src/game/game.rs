use super::actor::enemy::EnemyPlugin;
use super::actor::player::PlayerPlugin;
use super::actor::remote::RemotePlayerPlugin;
use super::asset::GameAssets;
use super::bgm::BGMPlugin;
use super::camera::*;
use super::close_on_esc::close_on_esc;
use super::config::GameConfigPlugin;
use super::constant::INITIAL_STATE;
use super::constant::PIXELS_PER_METER;
use super::embedded::EmbeddedAssetPlugin;
use super::entity::book_shelf::BookshelfPlugin;
use super::entity::bullet::BulletPlugin;
use super::entity::witch::WitchPlugin;
use super::gamepad::GamepadPlugin;
use super::hud::life_bar::LifeBarPlugin;
use super::hud::overlay::*;
use super::hud::pointer::PointerPlugin;
use super::hud::*;
use super::main_menu::*;
use super::serialize::*;
use super::states::*;
use super::world::*;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::Cursor;
use bevy::window::EnabledButtons;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_asset_loader::prelude::*;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use bevy_websocket_sync::WebSocketPlugin;
use wall::WallPlugin;

#[cfg(feature = "debug")]
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
#[cfg(feature = "debug")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "debug")]
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "debug")]
use iyes_perf_ui::PerfUiPlugin;

pub fn run_game() {
    let mut app = App::new();

    println!("main");

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
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        cursor: Cursor {
                            // visible: false,
                            ..default()
                        },
                        title: "Magia Boost 0.1".to_string(),
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
                })
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                }),
            //
        )
        .add_plugins(EmbeddedAssetPlugin)
        // RapierConfiguration は RapierPhysicsPlugin の初期化の前に設定する必要があるらしい
        // そうしないと警告が出る
        .insert_resource(RapierConfiguration {
            gravity: Vect::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Fixed {
                dt: 0.016666668,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                // .with_length_unit(PIXELS_PER_METER)
                .in_fixed_schedule(),
        )
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
        .add_plugins(GamepadPlugin)
        .add_plugins(PointerPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(WebSocketPlugin)
        .add_plugins(GameConfigPlugin)
        .add_plugins(RemotePlayerPlugin)
        .add_plugins(WitchPlugin)
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
