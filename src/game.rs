use super::actor::player::PlayerPlugin;
use super::asset::GameAssets;
use super::camera::*;
use super::constant::INITIAL_STATE;
use super::constant::PIXELS_PER_METER;
use super::entity::bullet::BulletPlugin;
use super::entity::witch::WitchPlugin;
use super::hud::pointer::PointerPlugin;
use super::hud::*;
use super::states::*;
use super::world::*;
use crate::entity::actor::ActorPlugin;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::Cursor;
use bevy::window::EnabledButtons;
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

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
                    level: Level::INFO,
                    ..default()
                }),
            //
        )
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
        //
        // 以下はこのゲーム本体で定義されたプラグイン
        //
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(PointerPlugin)
        .add_plugins(WitchPlugin)
        .add_plugins(ActorPlugin)
        //
        // 全体の初期化をするsystem
        // カメラなど、最初の画面に関わらず必要な初期化はここで行っています
        //
        .add_systems(OnEnter(GameState::Setup), |mut commands: Commands| {
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

    app.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            mode: DebugRenderMode::COLLIDER_SHAPES | DebugRenderMode::COLLIDER_AABBS,
            ..default()
        });

    app.run();
}
