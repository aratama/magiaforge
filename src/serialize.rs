use crate::Person;
use bevy::{
    prelude::*,
    time::{Time, Timer},
};
use serde::*;
use std::time::Duration;

// use https://github.com/Zeenobit/moonshine_save?

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub x: f32,
    pub y: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData { x: 20.0, y: 20.0 }
    }
}

pub fn save_player(data: &PlayerData) {
    let win = web_sys::window().unwrap();
    let local_storage = win.local_storage().unwrap().unwrap();
    let str = serde_json::to_string(data).unwrap();
    local_storage.set_item("data", str.as_str()).unwrap();
}

pub fn restore_player() -> PlayerData {
    let win = web_sys::window().unwrap();
    let local_storage = win.local_storage().unwrap().unwrap();
    if let Ok(Some(str)) = local_storage.get_item("data") {
        if let Ok(data) = serde_json::from_str::<PlayerData>(str.as_str()) {
            data
        } else {
            default()
        }
    } else {
        default()
    }
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
    player_query: Query<&Transform, (With<Person>, Without<Camera2d>)>,
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
