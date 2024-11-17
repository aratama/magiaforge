use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{config::GameConfig, player_state::PlayerState, states::GameState, world::NextLevel};

fn process_debug_command(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    mut next: ResMut<NextState<GameState>>,
    mut level: ResMut<NextLevel>,
    config: Res<GameConfig>,
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
    // info!("debug commands: {}", *local);

    if local.ends_with("next") {
        local.clear();
        *level = match level.as_ref() {
            NextLevel::None => NextLevel::Level(1, PlayerState::from_config(&config)),
            NextLevel::Level(n, p) => NextLevel::Level(n + 1, p.clone()),
            NextLevel::MultiPlayArena(_) => NextLevel::Level(0, PlayerState::from_config(&config)),
        };
        info!("next level: {:?}", level);
        next.set(GameState::Warp);
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
