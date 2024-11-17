use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{
    config::GameConfig, hud::overlay::OverlayEvent, player_state::PlayerState, states::GameState,
    world::NextLevel,
};

fn process_debug_command(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    mut level: ResMut<NextLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
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
        info!("debug command: next");
        local.clear();
        *level = match level.as_ref() {
            NextLevel::None => NextLevel::Level(1, PlayerState::from_config(&config)),
            NextLevel::Level(n, p) => NextLevel::Level(n + 1, p.clone()),
            NextLevel::MultiPlayArena(_) => NextLevel::Level(0, PlayerState::from_config(&config)),
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("home") {
        info!("debug command: home");
        local.clear();
        *level = match level.as_ref() {
            NextLevel::None => NextLevel::Level(0, PlayerState::from_config(&config)),
            NextLevel::Level(_, p) => NextLevel::Level(0, p.clone()),
            NextLevel::MultiPlayArena(_) => NextLevel::Level(0, PlayerState::from_config(&config)),
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
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
