use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{
    config::GameConfig,
    hud::overlay::OverlayEvent,
    level::{GameLevel, NextLevel},
    physics::GamePhysics,
    player_state::PlayerState,
    states::GameState,
};

fn process_debug_command(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    mut level: ResMut<NextLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
    mut physics: ResMut<GamePhysics>,
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
        let next = level.as_ref();
        *level = match next.level {
            GameLevel::Level(n) => NextLevel {
                level: GameLevel::Level((n + 1) % 4),
                state: next.state.clone(),
            },
            GameLevel::MultiPlayArena => NextLevel {
                level: GameLevel::Level(0),
                state: PlayerState::from_config(&config),
            },
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("home") {
        local.clear();
        *level = NextLevel {
            level: GameLevel::Level(0),
            state: PlayerState::from_config(&config),
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("arena") {
        local.clear();
        *level = NextLevel {
            level: GameLevel::MultiPlayArena,
            state: PlayerState::from_config(&config),
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("boss") {
        local.clear();
        *level = NextLevel {
            level: GameLevel::Level(3),
            state: PlayerState::from_config(&config),
        };
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
