use super::player::Player;
use crate::game::constant::TILE_SIZE;
use bevy::{
    prelude::*,
    time::{Time, Timer},
};
use serde::*;
use std::time::Duration;

// use https://github.com/Zeenobit/moonshine_save?

#[cfg(target_arch = "wasm32")]
const PLAYER_DATA_KEY: &str = "data";

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub x: f32,
    pub y: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            x: TILE_SIZE * 5.0,
            y: TILE_SIZE * -7.0,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn save_player(data: &PlayerData) {
    let win = web_sys::window().unwrap();
    let local_storage = win.local_storage().unwrap().unwrap();
    if let Ok(str) = serde_json::to_string(data) {
        local_storage.set_item(PLAYER_DATA_KEY, &str).unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_player(_data: &PlayerData) {
    // todo
}

#[cfg(target_arch = "wasm32")]
pub fn restore_player() -> PlayerData {
    let win = web_sys::window().unwrap();
    let local_storage = win.local_storage().unwrap().unwrap();
    if let Ok(Some(str)) = local_storage.get_item(PLAYER_DATA_KEY) {
        serde_json::from_str::<PlayerData>(&str).unwrap_or_default()
    } else {
        default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn restore_player() -> PlayerData {
    // todo
    default()
}

#[derive(Resource)]
pub struct AutoSaveTimerConfig {
    timer: Timer,
}

pub fn setup_autosave_timer(mut commands: Commands) {
    commands.insert_resource(AutoSaveTimerConfig {
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    })
}

pub fn spawn_autosave_timer(
    time: Res<Time>,
    mut config: ResMut<AutoSaveTimerConfig>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    config.timer.tick(time.delta());
    if config.timer.finished() {
        if let Ok(player) = player_query.get_single() {
            save_player(&PlayerData {
                x: player.translation.x,
                y: player.translation.y,
            });
        }
    }
}
