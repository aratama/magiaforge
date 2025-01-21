use crate::component::life::Life;
use crate::constant::LAST_BOSS_LEVEL;
use crate::constant::LEVELS;
use crate::controller::player::Player;
use crate::actor::Actor;
use crate::hud::overlay::OverlayEvent;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::player_state::PlayerState;
use crate::set::FixedUpdateInGameSet;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

fn process_debug_command(
    mut commands: Commands,
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match ev.logical_key {
            Key::Character(ref c) => {
                local.push_str(c);
            }
            _ => {}
        }
    }

    if local.ends_with("@next") {
        local.clear();
        commands.run_system_cached(debug_next);
    } else if local.ends_with("@home") {
        local.clear();
        commands.run_system_cached(debug_home);
    } else if local.ends_with("@arena") {
        local.clear();
        commands.run_system_cached(debug_arena);
    } else if local.ends_with("@boss") {
        local.clear();
        commands.run_system_cached(debug_boss);
    } else if local.ends_with("@ending") {
        local.clear();
        commands.run_system_cached(debug_ending);
    } else if local.ends_with("@pause") {
        local.clear();
        commands.run_system_cached(debug_pause);
    } else if local.ends_with("@resume") {
        local.clear();
        commands.run_system_cached(debug_resume);
    } else if local.ends_with("@level1") {
        local.clear();
        commands.run_system_cached(debug_level_1);
    } else if local.ends_with("@level2") {
        local.clear();
        commands.run_system_cached(debug_level_2);
    } else if local.ends_with("@level3") {
        local.clear();
        commands.run_system_cached(debug_level_3);
    } else if local.ends_with("@level4") {
        local.clear();
        commands.run_system_cached(debug_level_4);
    } else if local.ends_with("@level5") {
        local.clear();
        commands.run_system_cached(debug_level_5);
    }
}

fn debug_next(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    match level.next_level {
        GameLevel::Level(n) => {
            level.next_level = GameLevel::Level((n + 1) % LEVELS);
            level.next_state = Some(PlayerState::from_query(&player_query));
            writer.send(OverlayEvent::Close(GameState::Warp));
        }
        GameLevel::MultiPlayArena => {
            level.next_level = GameLevel::Level(0);
            writer.send(OverlayEvent::Close(GameState::Warp));
        }
    };
}

fn debug_home(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(0);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_level_1(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(1);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_level_2(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(2);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_level_3(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(3);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_level_4(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(4);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_level_5(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(5);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_arena(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::MultiPlayArena;
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_boss(
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(LAST_BOSS_LEVEL);
    level.next_state = Some(PlayerState::from_query(&player_query));
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_ending(mut writer: EventWriter<OverlayEvent>) {
    writer.send(OverlayEvent::Close(GameState::Ending));
}

fn debug_pause(mut in_game_time: ResMut<NextState<TimeState>>) {
    in_game_time.set(TimeState::Inactive);
}

fn debug_resume(mut in_game_time: ResMut<NextState<TimeState>>) {
    in_game_time.set(TimeState::Active);
}

pub struct DebugCommandPlugin;

impl Plugin for DebugCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            process_debug_command.in_set(FixedUpdateInGameSet),
        );
    }
}
