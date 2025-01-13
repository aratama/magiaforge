use crate::asset::GameAssets;
use crate::audio::GameAudioPlugin;
use crate::camera::*;
use crate::component::animated_slice::AnimatedSlicePlugin;
use crate::component::counter::CounterPlugin;
use crate::component::entity_depth::EntityDepthPlugin;
use crate::component::falling::FallingPlugin;
use crate::component::flip::FlipPlugin;
use crate::component::life::LifePlugin;
use crate::component::point_light::EntityPointLightPlugin;
use crate::config::GameConfigPlugin;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGoldPlugin;
use crate::controller::message_rabbit::MessageRabbitPlugin;
use crate::controller::player::PlayerPlugin;
use crate::controller::remote::RemotePlayerPlugin;
use crate::controller::servant::ServantPlugin;
use crate::controller::shop_rabbit::ShopRabbitPlugin;
use crate::controller::training_dummy::TrainingDummyPlugin;
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;
#[cfg(feature = "debug_command")]
use crate::debug_command::DebugCommandPlugin;
use crate::enemy::basic::BasicEnemyPlugin;
use crate::enemy::chicken::ChikenControlPlugin;
use crate::enemy::eyeball::EyeballControlPlugin;
use crate::enemy::huge_slime::HugeSlimePlugin;
use crate::enemy::salamander::SalamanderPlugin;
use crate::enemy::sandbug::SandbagPlugin;
use crate::enemy::shadow::ShadowPlugin;
use crate::enemy::slime::SlimeControlPlugin;
use crate::enemy::spider::SpiderPlugin;
use crate::entity::actor::ActorPlugin;
use crate::entity::bgm::BGMSwitchPlugin;
use crate::entity::bomb::BombPlugin;
use crate::entity::book_shelf::BookshelfPlugin;
use crate::entity::bullet::BulletPlugin;
use crate::entity::bullet_particle::BulletParticlePlugin;
use crate::entity::chest::ChestPlugin;
use crate::entity::damege::DamagePlugin;
use crate::entity::dropped_item::SpellEntityPlugin;
use crate::entity::explosion::ExplosionPlugin;
use crate::entity::fire::FirePlugin;
use crate::entity::fireball::FireballPlugin;
use crate::entity::gold::GoldPlugin;
use crate::entity::grass::GrassPlugin;
use crate::entity::impact::ImpactPlugin;
use crate::entity::magic_circle::MagicCirclePlugin;
use crate::entity::rabbit::RabbitPlugin;
use crate::entity::rock::RockPlugin;
use crate::entity::servant_seed::ServantSeedPlugin;
use crate::entity::shop::ShopPlugin;
use crate::entity::stone_lantern::StoneLanternPlugin;
use crate::entity::web::WebPlugin;
use crate::entity::witch::WitchPlugin;
use crate::footsteps::FootStepsPlugin;
use crate::hud::life_bar::LifeBarPlugin;
use crate::hud::overlay::*;
use crate::hud::pointer::PointerPlugin;
use crate::hud::*;
use crate::input::GameInputPlugin;
use crate::language::LanguagePlugin;
use crate::page::ending::EndingPlugin;
use crate::page::in_game::WorldPlugin;
use crate::page::main_menu::MainMenuPlugin;
use crate::page::name_input::NameInputPagePlugin;
use crate::page::opening::OpeningPlugin;
use crate::page::setup::SetupPlugin;
use crate::page::warp::WarpPagePlugin;
use crate::physics::GamePhysicsPlugin;
use crate::player_state::PlayerStatePlugin;
#[cfg(feature = "save")]
use crate::save::SavePlugin;
use crate::se::SECommandPlugin;
use crate::set::GameSetPlugin;
use crate::states::*;
use crate::theater::SenarioPlugin;
use crate::ui::bar::StatusBarPlugin;
use crate::ui::boss_hitpoint_bar::BossHitpointBarPlugin;
use crate::ui::command_button::CommandButtonPlugin;
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
use crate::ui::speech_bubble::SpeechBubblePlugin;
use crate::ui::spell_in_wand::SpellInWandPlugin;
use crate::ui::spell_list::SpellListPlugin;
use crate::ui::wand_editor::WandEditorPlugin;
use crate::ui::wand_list::WandListPlugin;
use bevy::asset::AssetMetaCheck;
// use bevy::audio::AudioPlugin;
// use bevy::audio::SpatialScale;
use bevy::log::*;
use bevy::prelude::*;
use bevy::window::CursorOptions;
use bevy::window::EnabledButtons;
use bevy::window::WindowMode;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_asset_loader::prelude::*;
#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
use bevy_embedded_assets::PluginMode;
use bevy_light_2d::plugin::Light2dPlugin;
use bevy_rapier2d::prelude::*;
use bevy_simple_text_input::TextInputPlugin;
use bevy_simple_websocket::WebSocketPlugin;
use gameover::GameoverPlugin;

// const AUDIO_SCALE: f32 = 1. / 1000.0;

pub fn run_game() {
    let mut app = App::new();

    // アセットの埋め込みはリリースモードでのビルドのみ、
    // つまりWindowsなどのネイティブビルドのみで有効にしています
    // これはDefaultPluginsの前に呼び出す必要があります
    #[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
    app.add_plugins(EmbeddedAssetPlugin {
        mode: PluginMode::ReplaceDefault,
    });

    app
        //
        // デフォルトのプラグインや依存しているサードパーティのプラグインなど
        //
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                // .set(AudioPlugin {
                //     default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                //     ..default()
                // })
                .set(AssetPlugin {
                    // https://github.com/bevyengine/bevy/issues/10157
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
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
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        })
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
        .add_plugins(AnimatedSlicePlugin)
        .add_plugins(BasicEnemyPlugin)
        .add_plugins(BGMSwitchPlugin)
        .add_plugins(BookshelfPlugin)
        .add_plugins(BombPlugin)
        .add_plugins(BossHitpointBarPlugin)
        .add_plugins(LifePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(BulletParticlePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ChestPlugin)
        .add_plugins(ChikenControlPlugin)
        .add_plugins(CommandButtonPlugin)
        .add_plugins(CounterPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(DespawnWithGoldPlugin)
        .add_plugins(EndingPlugin)
        .add_plugins(EyeballControlPlugin)
        .add_plugins(EntityDepthPlugin)
        .add_plugins(EntityPointLightPlugin)
        .add_plugins(ExplosionPlugin)
        .add_plugins(FallingPlugin)
        .add_plugins(FirePlugin)
        .add_plugins(FireballPlugin)
        .add_plugins(FootStepsPlugin)
        .add_plugins(GameAudioPlugin)
        .add_plugins(GameoverPlugin)
        .add_plugins(GrassPlugin)
        .add_plugins(SECommandPlugin)
        .add_plugins(FlipPlugin)
        .add_plugins(GameConfigPlugin)
        .add_plugins(GameMenuPlugin)
        .add_plugins(GameInputPlugin)
        .add_plugins(GamePhysicsPlugin)
        .add_plugins(GameSetPlugin)
        .add_plugins(GoldPlugin)
        .add_plugins(HoverColorPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(HugeSlimePlugin)
        .add_plugins(ImpactPlugin)
        .add_plugins(InventoryItemFloatingPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(ItemPanelPlugin)
        .add_plugins(LanguagePlugin)
        .add_plugins(LabelPlugin)
        .add_plugins(LifeBarPlugin)
        .add_plugins(MagicCirclePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(MenuButtonPlugin)
        .add_plugins(MenuLeftPlugin)
        .add_plugins(NameInputPagePlugin)
        .add_plugins(OnPressPlugin)
        .add_plugins(OverlayPlugin)
        .add_plugins(OpeningPlugin)
        .add_plugins(PlayerListPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PlayerStatePlugin)
        .add_plugins(PointerPlugin)
        .add_plugins(PopUpPlugin)
        .add_plugins(RabbitPlugin)
        .add_plugins(RemotePlayerPlugin)
        .add_plugins(RockPlugin)
        .add_plugins(SalamanderPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(ServantPlugin)
        .add_plugins(ShopPlugin)
        .add_plugins(ShadowPlugin)
        .add_plugins(SlimeControlPlugin)
        .add_plugins(ServantSeedPlugin)
        .add_plugins(SpeechBubblePlugin)
        .add_plugins(SandbagPlugin)
        .add_plugins(SenarioPlugin)
        .add_plugins(ShopRabbitPlugin)
        .add_plugins(SpellEntityPlugin)
        .add_plugins(SpellInWandPlugin)
        .add_plugins(SpellListPlugin)
        .add_plugins(SpiderPlugin)
        .add_plugins(StatusBarPlugin)
        .add_plugins(StoneLanternPlugin)
        .add_plugins(TrainingDummyPlugin)
        .add_plugins(MessageRabbitPlugin)
        .add_plugins(WandEditorPlugin)
        .add_plugins(WandListPlugin)
        .add_plugins(WarpPagePlugin)
        .add_plugins(WebPlugin)
        .add_plugins(WebSocketPlugin)
        .add_plugins(WitchPlugin)
        .add_plugins(WorldPlugin)
        //
        // メインメニューやゲームプレイ画面などのシーンを定義するstate
        //
        .init_state::<GameState>()
        .add_sub_state::<MainMenuPhase>()
        .add_sub_state::<GameMenuState>()
        .add_sub_state::<TimeState>()
        .add_loading_state(
            LoadingState::new(GameState::Setup)
                .continue_to_state(
                    // Setupステートでの初期化が完了した直後に遷移する先のステート
                    if cfg!(feature = "ingame") {
                        GameState::InGame
                    } else if cfg!(feature = "opening") {
                        GameState::Opening
                    } else {
                        GameState::MainMenu
                    },
                )
                .load_collection::<GameAssets>(),
        )
        // State Scoped Entities をオンにすることで、
        // stateを変更したときに自動的にエンティティを削除できます
        // https://bevyengine.org/news/bevy-0-14/#state-scoped-entities
        .enable_state_scoped_entities::<GameState>();

    #[cfg(feature = "debug_command")]
    app.add_plugins(DebugCommandPlugin);

    // bevy_pkv を使うとセーブファイルがロックされるため、複数のインスタンスを同時に起動できなくなります
    // 開発時に不便なので、フィーチャーフラグで開発時は無効にしておきます
    #[cfg(feature = "save")]
    app.add_plugins(SavePlugin);

    // デバッグ用のプラグインなど
    // 無くてもゲーム自体は動作します
    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

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
