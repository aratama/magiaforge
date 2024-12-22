use crate::asset::GameAssets;
use crate::audio::GameAudioPlugin;
use crate::camera::*;
use crate::config::GameConfigPlugin;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGoldPlugin;
use crate::controller::message_rabbit::MessageRabbitPlugin;
use crate::controller::player::PlayerPlugin;
use crate::controller::remote::RemotePlayerPlugin;
use crate::controller::servant::ServantPlugin;
use crate::controller::shop_rabbit::ShopRabbitPlugin;
use crate::controller::training_dummy::TrainingDummyPlugin;
use crate::debug::DebugCommandPlugin;
use crate::enemy::eyeball::EyeballControlPlugin;
use crate::enemy::huge_slime::HugeSlimePlugin;
use crate::enemy::slime::SlimeControlPlugin;
use crate::entity::actor::ActorPlugin;
use crate::entity::bgm::BGMSwitchPlugin;
use crate::entity::book_shelf::BookshelfPlugin;
use crate::entity::bullet::BulletPlugin;
use crate::entity::bullet_particle::BulletParticlePlugin;
use crate::entity::chest::ChestPlugin;
use crate::entity::damege::DamagePlugin;
use crate::entity::dropped_item::SpellEntityPlugin;
use crate::entity::gold::GoldPlugin;
use crate::entity::impact::ImpactPlugin;
use crate::entity::life::LifePlugin;
use crate::entity::magic_circle::MagicCirclePlugin;
use crate::entity::rabbit::RabbitPlugin;
use crate::entity::servant_seed::ServantSeedPlugin;
use crate::entity::shop::ShopPlugin;
use crate::entity::stone_lantern::StoneLanternPlugin;
use crate::entity::witch::WitchPlugin;
use crate::entity::EntityPlugin;
use crate::footsteps::FootStepsPlugin;
use crate::hud::life_bar::LifeBarPlugin;
use crate::hud::overlay::*;
use crate::hud::pointer::PointerPlugin;
use crate::hud::*;
use crate::input::GameInputPlugin;
use crate::level::*;
use crate::page::ending::EndingPlugin;
use crate::page::main_menu::MainMenuPlugin;
use crate::page::name_input::NameInputPagePlugin;
use crate::page::setup::SetupPlugin;
use crate::page::warp::WarpPagePlugin;
use crate::physics::GamePhysicsPlugin;
use crate::se::SECommandPlugin;
use crate::speech_bubble::SpeechBubblePlugin;
use crate::states::*;
use crate::ui::bar::StatusBarPlugin;
use crate::ui::boss_hitpoint_bar::BossHitpointBarPlugin;
use crate::ui::command_button::CommandButtonPlugin;
use crate::ui::equipment_list::EquipmentListPlugin;
use crate::ui::floating::InventoryItemFloatingPlugin;
use crate::ui::hover_color::HoverColorPlugin;
use crate::ui::inventory::InventoryPlugin;
use crate::ui::item_panel::ItemPanelPlugin;
use crate::ui::label::LabelPlugin;
use crate::ui::menu_button::MenuButtonPlugin;
use crate::ui::menu_left::MenuLeftPlugin;
use crate::ui::on_press::OnPressPlugin;
use crate::ui::pause_menu::GameMenuPlugin;
use crate::ui::player_list::PlayerListPlugin;
use crate::ui::popup::PopUpPlugin;
use crate::ui::spell_in_wand::SpellInWandPlugin;
use crate::ui::wand_editor::WandEditorPlugin;
use crate::ui::wand_list::WandListPlugin;
use crate::ui::wand_sprite::WandSpritePlugin;
use bevy::asset::AssetMetaCheck;
use bevy::log::*;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::window::{CursorOptions, EnabledButtons};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_asset_loader::prelude::*;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_rapier2d::prelude::*;
use bevy_simple_text_input::TextInputPlugin;
use bevy_simple_websocket::WebSocketPlugin;
use gameover::GameoverPlugin;
use wall::WallPlugin;

#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
use bevy_embedded_assets::PluginMode;
#[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
use bevy_pkv::PkvStore;

#[cfg(feature = "debug")]
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
#[cfg(feature = "debug")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(all(feature = "debug", not(target_arch = "wasm32")))]
use bevy_remote_inspector::RemoteInspectorPlugins;

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
                        cursor_options: CursorOptions {
                            visible: cfg!(feature = "debug"),
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
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                .in_fixed_schedule()
                .with_custom_initialization(RapierContextInitialization::NoAutomaticRapierContext),
        )
        .add_systems(Startup, setup_rapier_context)
        .add_plugins(Light2dPlugin)
        .add_plugins(TextInputPlugin)
        //
        // 以下はこのゲーム本体で定義されたプラグイン
        //
        .add_systems(Update, toggle_fullscreen)
        .add_plugins(ActorPlugin)
        .add_plugins(BGMSwitchPlugin)
        .add_plugins(BookshelfPlugin)
        .add_plugins(BossHitpointBarPlugin)
        .add_plugins(LifePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(BulletParticlePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ChestPlugin)
        .add_plugins(CommandButtonPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(DebugCommandPlugin)
        .add_plugins(DespawnWithGoldPlugin)
        .add_plugins(EndingPlugin)
        .add_plugins(EyeballControlPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(EquipmentListPlugin)
        .add_plugins(FootStepsPlugin)
        .add_plugins(GameAudioPlugin)
        .add_plugins(GameoverPlugin)
        .add_plugins(SECommandPlugin)
        .add_plugins(GameConfigPlugin)
        .add_plugins(GameMenuPlugin)
        .add_plugins(GameInputPlugin)
        .add_plugins(GamePhysicsPlugin)
        .add_plugins(GoldPlugin)
        .add_plugins(HoverColorPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(HugeSlimePlugin)
        .add_plugins(ImpactPlugin)
        .add_plugins(InventoryItemFloatingPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(ItemPanelPlugin)
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
        .add_plugins(RabbitPlugin)
        .add_plugins(RemotePlayerPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(ServantPlugin)
        .add_plugins(ShopPlugin)
        .add_plugins(SlimeControlPlugin)
        .add_plugins(ServantSeedPlugin)
        .add_plugins(SpeechBubblePlugin)
        .add_plugins(PopUpPlugin)
        .add_plugins(ShopRabbitPlugin)
        .add_plugins(SpellEntityPlugin)
        .add_plugins(SpellInWandPlugin)
        .add_plugins(StatusBarPlugin)
        .add_plugins(StoneLanternPlugin)
        .add_plugins(TrainingDummyPlugin)
        .add_plugins(MessageRabbitPlugin)
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
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin)
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            mode: DebugRenderMode::COLLIDER_SHAPES,
            ..default()
        });

    #[cfg(all(feature = "debug", not(target_arch = "wasm32")))]
    app.add_plugins(RemoteInspectorPlugins);

    app.run();
}

fn setup_rapier_context(mut commands: Commands) {
    commands.spawn((
        Name::new("default rapier context"),
        DefaultRapierContext,
        RapierContext::default(),
        RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        },
    ));
}

fn toggle_fullscreen(mut window_query: Query<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = window_query.single_mut();
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::SizedFullscreen(MonitorSelection::Current),
            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
            WindowMode::SizedFullscreen(_) => WindowMode::Windowed,
            WindowMode::Fullscreen(_) => WindowMode::Windowed,
        };
    }
}
