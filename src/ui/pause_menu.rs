use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::constant::HOME_LEVEL;
use crate::hud::overlay::OverlayEvent;
use crate::language::Dict;
use crate::language::Languages;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::message::BGM_VOLUME;
use crate::message::FULLSCREEN;
use crate::message::OFF;
use crate::message::ON;
use crate::message::PAUSED;
use crate::message::RESUME;
use crate::message::RETURN_TO_TITLE;
use crate::message::SFX_VOLUME;
use crate::se::SEEvent;
use crate::se::CLICK;
use crate::set::FixedUpdateInGameSet;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::label::spawn_label;
use crate::ui::menu_button::menu_button;
use crate::ui::range::spawn_range;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;
use bevy_simple_websocket::ClientMessage;

#[derive(Resource)]
struct ButtonShots {
    close: SystemId,
    exit: SystemId,
    bgm_volume_up: SystemId,
    bgm_volume_down: SystemId,
    se_volume_up: SystemId,
    se_volume_down: SystemId,
    ja: SystemId,
    en: SystemId,
    zh_cn: SystemId,
    zh_tw: SystemId,
    es: SystemId,
    fr: SystemId,
    pt: SystemId,
    ru: SystemId,
    de: SystemId,
    ko: SystemId,
    #[cfg(not(target_arch = "wasm32"))]
    fullscreen_on: SystemId,
    #[cfg(not(target_arch = "wasm32"))]
    fullscreen_off: SystemId,
    wait: i32,
}

#[derive(Component)]
struct PauseMenuRoot;

#[derive(Component)]
struct BGMVolumeLabel;

#[derive(Component)]
struct SEVolumeLabel;

#[derive(Component)]
pub struct BGMCredit;

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            close: world.register_system(resume),
            exit: world.register_system(exit),
            bgm_volume_up: world.register_system(volume_up),
            bgm_volume_down: world.register_system(volume_down),
            se_volume_up: world.register_system(se_volume_up),
            se_volume_down: world.register_system(se_volume_down),
            ja: world.register_system(ja),
            en: world.register_system(en),
            zh_cn: world.register_system(zh_cn),
            zh_tw: world.register_system(zh_tw),
            es: world.register_system(es),
            fr: world.register_system(fr),
            pt: world.register_system(pt),
            ru: world.register_system(ru),
            de: world.register_system(de),
            ko: world.register_system(ko),
            #[cfg(not(target_arch = "wasm32"))]
            fullscreen_on: world.register_system(fullscreen_on),
            #[cfg(not(target_arch = "wasm32"))]
            fullscreen_off: world.register_system(fullscreen_off),
            wait: 0,
        }
    }
}

fn resume(mut state: ResMut<NextState<GameMenuState>>, mut writer: EventWriter<SEEvent>) {
    state.set(GameMenuState::PauseMenuClosing);
    writer.send(SEEvent::new(CLICK));
}

fn exit(
    mut writer: EventWriter<SEEvent>,
    mut websocket: EventWriter<ClientMessage>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
    mut next: ResMut<GameWorld>,
) {
    next.next_level = GameLevel::new(HOME_LEVEL);
    overlay_event_writer.send(OverlayEvent::Close(GameState::MainMenu));
    writer.send(SEEvent::new(CLICK));
    websocket.send(ClientMessage::Close);
}

fn volume_up(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.bgm_volume = (config.bgm_volume + 0.1).min(1.0);
    writer.send(SEEvent::new(CLICK));
}

fn volume_down(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.bgm_volume = (config.bgm_volume - 0.1).max(0.0);
    writer.send(SEEvent::new(CLICK));
}

fn se_volume_up(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.se_volume = (config.se_volume + 0.1).min(1.0);
    writer.send(SEEvent::new(CLICK));
}

fn se_volume_down(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.se_volume = (config.se_volume - 0.1).max(0.0);
    writer.send(SEEvent::new(CLICK));
}

fn ja(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::Ja;
    writer.send(SEEvent::new(CLICK));
}

fn en(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::En;
    writer.send(SEEvent::new(CLICK));
}

fn zh_cn(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::ZhCn;
    writer.send(SEEvent::new(CLICK));
}

fn zh_tw(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::ZhTw;
    writer.send(SEEvent::new(CLICK));
}

fn es(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::Es;
    writer.send(SEEvent::new(CLICK));
}

fn fr(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::Fr;
    writer.send(SEEvent::new(CLICK));
}

fn pt(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::Pt;
    writer.send(SEEvent::new(CLICK));
}

fn ru(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::Ru;
    writer.send(SEEvent::new(CLICK));
}

fn de(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::De;
    writer.send(SEEvent::new(CLICK));
}

fn ko(mut config: ResMut<GameConfig>, mut writer: EventWriter<SEEvent>) {
    config.language = Languages::Ko;
    writer.send(SEEvent::new(CLICK));
}

#[cfg(not(target_arch = "wasm32"))]
fn fullscreen_on(
    mut config: ResMut<GameConfig>,
    mut writer: EventWriter<SEEvent>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    window.mode = WindowMode::SizedFullscreen(MonitorSelection::Primary);
    writer.send(SEEvent::new(CLICK));
    config.fullscreen = true;
}

#[cfg(not(target_arch = "wasm32"))]
fn fullscreen_off(
    mut config: ResMut<GameConfig>,
    mut writer: EventWriter<SEEvent>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    window.mode = WindowMode::Windowed;
    writer.send(SEEvent::new(CLICK));
    config.fullscreen = false;
}

fn setup_game_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    shots: Res<ButtonShots>,
    config: Res<GameConfig>,
) {
    commands
        .spawn((
            Name::new("Pause Menu"),
            PauseMenuRoot,
            StateScoped(GameState::InGame),
            BackgroundColor(Color::hsla(0.0, 0.0, 0.0, 0.95)),
            GlobalZIndex(GAME_MENU_Z_INDEX),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_label(parent, &assets, PAUSED.to_string());

                    parent
                        .spawn(Node {
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Start,

                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_label(parent, &assets, Dict::literal("言語/Language"));

                            menu_button(
                                parent,
                                &assets,
                                shots.ja,
                                140.0,
                                50.0,
                                Dict::literal("日本語"),
                            );
                            menu_button(
                                parent,
                                &assets,
                                shots.en,
                                140.0,
                                50.0,
                                Dict::literal("English"),
                            );
                            menu_button(
                                parent,
                                &assets,
                                shots.zh_cn,
                                180.0,
                                50.0,
                                Dict::literal("简体中文"),
                            );
                            menu_button(
                                parent,
                                &assets,
                                shots.zh_tw,
                                180.0,
                                50.0,
                                Dict::literal("繁体中文"),
                            );
                        });

                    parent
                        .spawn(Node {
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Start,

                            ..default()
                        })
                        .with_children(|parent| {
                            menu_button(
                                parent,
                                &assets,
                                shots.es,
                                140.0,
                                50.0,
                                Dict::literal("Eespañol"),
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.fr,
                                140.0,
                                50.0,
                                Dict::literal("Français"),
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.pt,
                                140.0,
                                50.0,
                                Dict::literal("Português"),
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.ru,
                                140.0,
                                50.0,
                                Dict::literal("Русский"),
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.de,
                                140.0,
                                50.0,
                                Dict::literal("Deutsch"),
                            );

                            menu_button(
                                parent,
                                &assets,
                                shots.ko,
                                140.0,
                                50.0,
                                Dict::literal("한국어"),
                            );
                        });

                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn(Node {
                            column_gap: Val::Px(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_label(parent, &assets, FULLSCREEN.to_string());
                            menu_button(
                                parent,
                                &assets,
                                shots.fullscreen_on,
                                240.0,
                                50.0,
                                ON.to_string(),
                            );
                            menu_button(
                                parent,
                                &assets,
                                shots.fullscreen_off,
                                240.0,
                                50.0,
                                OFF.to_string(),
                            );
                        });

                    spawn_range(
                        parent,
                        &assets,
                        BGMVolumeLabel,
                        (10.0 * config.bgm_volume).round() as u32,
                        shots.bgm_volume_up,
                        shots.bgm_volume_down,
                        BGM_VOLUME.to_string(),
                    );

                    spawn_range(
                        parent,
                        &assets,
                        SEVolumeLabel,
                        (10.0 * config.se_volume).round() as u32,
                        shots.se_volume_up,
                        shots.se_volume_down,
                        SFX_VOLUME.to_string(),
                    );

                    menu_button(
                        parent,
                        &assets,
                        shots.close,
                        280.0,
                        60.0,
                        RESUME.to_string(),
                    );

                    parent.spawn(Node {
                        width: Val::Px(280.0),
                        height: Val::Px(60.0),
                        ..default()
                    });

                    menu_button(
                        parent,
                        &assets,
                        shots.exit,
                        400.0,
                        60.0,
                        RETURN_TO_TITLE.to_string(),
                    );
                });

            parent.spawn((
                BGMCredit,
                Text::new(""),
                Label,
                TextFont {
                    font_size: 20.0,
                    font: assets.noto_sans_jp.clone(),
                    ..Default::default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
            ));
        });
}

fn update_game_menu(
    state: Res<State<GameMenuState>>,
    mut query: Query<&mut Visibility, With<PauseMenuRoot>>,
) {
    let mut visibility = query.single_mut();
    *visibility = match state.get() {
        GameMenuState::PauseMenuOpen => Visibility::Visible,
        _ => Visibility::Hidden,
    };
}

fn handle_escape_key(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match *state.get() {
            GameMenuState::Closed => {
                next.set(GameMenuState::PauseMenuOpen);
            }
            _ => {
                next.set(GameMenuState::Closed);
            }
        }
    }
}

fn switch_physics_activation(
    state: Res<State<GameMenuState>>,
    mut in_game_time: ResMut<NextState<TimeState>>,
) {
    if state.is_changed() {
        match *state.get() {
            GameMenuState::PauseMenuOpen => in_game_time.set(TimeState::Inactive),
            _ => in_game_time.set(TimeState::Active),
        }
    }
}

fn update_bgm_volume_label(
    config: Res<GameConfig>,
    mut query: Query<&mut Text, With<BGMVolumeLabel>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("{}", (10.0 * config.bgm_volume).round() as u32);
    }
}

fn update_se_volume_label(
    config: Res<GameConfig>,
    mut query: Query<&mut Text, With<SEVolumeLabel>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("{}", (10.0 * config.se_volume).round() as u32);
    }
}

fn closing_to_closed(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    mut res: ResMut<ButtonShots>,
) {
    match state.get() {
        GameMenuState::PauseMenuOpen => {
            res.wait = 20;
        }
        GameMenuState::PauseMenuClosing => {
            res.wait = (res.wait - 1).max(0);
            if res.wait <= 0 {
                next.set(GameMenuState::Closed);
            }
        }
        _ => {}
    }
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game_menu);
        app.add_systems(
            FixedUpdate,
            (
                update_game_menu,
                update_se_volume_label,
                update_bgm_volume_label,
                handle_escape_key,
                switch_physics_activation,
                closing_to_closed,
            )
                .in_set(FixedUpdateInGameSet),
        );
        app.init_resource::<ButtonShots>();
    }
}
