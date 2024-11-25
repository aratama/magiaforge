use crate::asset::GameAssets;
use crate::audio::GameAudioPlugin;
use crate::camera::*;
use crate::command::GameCommandPlugin;
use crate::config::GameConfigPlugin;
use crate::constant::*;
use crate::controller::enemy::EnemyPlugin;
use crate::controller::player::PlayerPlugin;
use crate::controller::remote::RemotePlayerPlugin;
use crate::debug::DebugCommandPlugin;
use crate::enemy::eyeball::EyeballControlPlugin;
use crate::enemy::slime::SlimeControlPlugin;
use crate::entity::actor::ActorPlugin;
use crate::entity::book_shelf::BookshelfPlugin;
use crate::entity::breakable::BreakablePlugin;
use crate::entity::bullet::BulletPlugin;
use crate::entity::chest::ChestPlugin;
use crate::entity::dropped_item::SpellEntityPlugin;
use crate::entity::gold::GoldPlugin;
use crate::entity::magic_circle::MagicCirclePlugin;
use crate::entity::stone_lantern::StoneLanternPlugin;
use crate::entity::witch::WitchPlugin;
use crate::entity::EntityPlugin;
use crate::footsteps::FootStepsPlugin;
use crate::hud::life_bar::LifeBarPlugin;
use crate::hud::overlay::*;
use crate::hud::pointer::PointerPlugin;
use crate::hud::*;
use crate::input::GamepadPlugin;
use crate::interaction_sensor::EntityPickerPlugin;
use crate::level::*;
use crate::page::main_menu::MainMenuPlugin;
use crate::page::name_input::NameInputPagePlugin;
use crate::page::warp::WarpPagePlugin;
use crate::states::*;
use crate::ui::bar::StatusBarPlugin;
use crate::ui::command_button::CommandButtonPlugin;
use crate::ui::equipment_list::EquipmentListPlugin;
use crate::ui::floating::InventoryItemFloatingPlugin;
use crate::ui::hover_color::HoverColorPlugin;
use crate::ui::inventory::InventoryPlugin;
use crate::ui::item_information::SpellInformationPlugin;
use crate::ui::label::LabelPlugin;
use crate::ui::menu_button::MenuButtonPlugin;
use crate::ui::menu_left::MenuLeftPlugin;
use crate::ui::on_press::OnPressPlugin;
use crate::ui::pause_menu::GameMenuPlugin;
use crate::ui::player_list::PlayerListPlugin;
use crate::ui::spell_in_wand::SpellInWandPlugin;
use crate::ui::wand_editor::WandEditorPlugin;
use crate::ui::wand_list::WandListPlugin;
use crate::ui::wand_sprite::WandSpritePlugin;
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::log::*;
use bevy::prelude::*;
use bevy::window::EnabledButtons;
use bevy::window::{Cursor, WindowMode};
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_asset_loader::prelude::*;
#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
use bevy_embedded_assets::PluginMode;
use bevy_kira_audio::AudioPlugin;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_particle_systems::ParticleSystemPlugin;
#[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;
use bevy_simple_text_input::TextInputPlugin;
use bevy_simple_websocket::WebSocketPlugin;
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

    // アセットの埋め込みはリリースモードでのビルドのみで有効にしています
    #[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
    app.add_plugins(EmbeddedAssetPlugin {
        mode: PluginMode::ReplaceDefault,
    });

    // bevy_pkv を使うとセーブファイルがロックされるため、複数のインスタンスを同時に起動できなくなります
    // 開発時に不便なので、フィーチャーフラグで開発時は無効にしておきます
    #[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
    app.insert_resource(PkvStore::new(CRATE_NAME, CRATE_NAME));

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
                            visible: false,
                            ..default()
                        },
                        title: format!("{} 0.1", CRATE_NAME).to_string(),
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
        .add_plugins(ParticleSystemPlugin)
        .add_plugins(Light2dPlugin)
        .add_plugins(TextInputPlugin)
        //
        // 以下はこのゲーム本体で定義されたプラグイン
        //
        .add_systems(Update, toggle_fullscreen)
        .add_plugins(ActorPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(BookshelfPlugin)
        .add_plugins(BreakablePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ChestPlugin)
        .add_plugins(CommandButtonPlugin)
        .add_plugins(DebugCommandPlugin)
        .add_plugins(EyeballControlPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(EntityPickerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(EquipmentListPlugin)
        .add_plugins(FootStepsPlugin)
        .add_plugins(GameAudioPlugin)
        .add_plugins(GameCommandPlugin)
        .add_plugins(GameConfigPlugin)
        .add_plugins(GameMenuPlugin)
        .add_plugins(GamepadPlugin)
        .add_plugins(GoldPlugin)
        .add_plugins(HoverColorPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(InventoryItemFloatingPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(LabelPlugin)
        .add_plugins(LifeBarPlugin)
        .add_plugins(MagicCirclePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(MenuButtonPlugin)
        .add_plugins(MenuLeftPlugin)
        .add_plugins(NameInputPagePlugin)
        .add_plugins(OnPressPlugin)
        .add_plugins(OverlayPlugin)
        .add_plugins(PlayerListPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PointerPlugin)
        .add_plugins(RemotePlayerPlugin)
        .add_plugins(SlimeControlPlugin)
        .add_plugins(SpellInformationPlugin)
        .add_plugins(SpellEntityPlugin)
        .add_plugins(SpellInWandPlugin)
        .add_plugins(StatusBarPlugin)
        .add_plugins(StoneLanternPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(WandEditorPlugin)
        .add_plugins(WandListPlugin)
        .add_plugins(WandSpritePlugin)
        .add_plugins(WarpPagePlugin)
        .add_plugins(WebSocketPlugin)
        .add_plugins(WitchPlugin)
        .add_plugins(WorldPlugin)
        //
        // メインメニューやゲームプレイ画面などのシーンを定義するstate
        //
        .init_state::<GameState>()
        .add_sub_state::<MainMenuPhase>()
        .add_sub_state::<GameMenuState>()
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
    // 無くてもゲーム自体は動作します
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

fn toggle_fullscreen(mut window_query: Query<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = window_query.single_mut();
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::SizedFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            WindowMode::SizedFullscreen => WindowMode::Windowed,
            WindowMode::Fullscreen => WindowMode::Windowed,
        };
    }
}
