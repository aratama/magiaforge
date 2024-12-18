use crate::{
    config::GameConfig,
    controller::player::Player,
    entity::{actor::Actor, life::Life},
    hud::overlay::OverlayEvent,
    level::{CurrentLevel, GameLevel},
    physics::GamePhysics,
    player_state::PlayerState,
    states::GameState,
};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_rapier2d::plugin::PhysicsSet;

fn process_debug_command(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    mut level: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
    mut physics: ResMut<GamePhysics>,
    player_query: Query<(&Player, &Actor, &Life)>,
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

    if local.ends_with("next") {
        local.clear();
        match level.next_level {
            GameLevel::Level(n) => {
                level.next_level = GameLevel::Level((n + 1) % 4);
                level.next_state = PlayerState::from(player_query.get_single(), &config);
            }
            GameLevel::MultiPlayArena => {
                level.next_level = GameLevel::Level(0);
                level.next_state = PlayerState::from(player_query.get_single(), &config);
            }
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("home") {
        local.clear();
        level.next_level = GameLevel::Level(0);
        level.next_state = PlayerState::from(player_query.get_single(), &config);
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("arena") {
        local.clear();
        level.next_level = GameLevel::MultiPlayArena;
        level.next_state = PlayerState::from(player_query.get_single(), &config);
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("boss") {
        local.clear();
        level.next_level = GameLevel::Level(3);
        level.next_state = PlayerState::from(player_query.get_single(), &config);
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("ending") {
        local.clear();
        writer.send(OverlayEvent::Close(GameState::Ending));
    } else if local.ends_with("pause") {
        local.clear();
        physics.active = false;
    } else if local.ends_with("resume") {
        local.clear();
        physics.active = true;
    }
}

pub struct DebugCommandPlugin;

impl Plugin for DebugCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            process_debug_command.before(PhysicsSet::SyncBackend),
        );
    }
}
