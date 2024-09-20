use bevy::math::Vec2;
use serde::*;

// use https://github.com/Zeenobit/moonshine_save?

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub x: f32,
    pub y: f32,
}

pub fn get_initial_position(player_data: Option<PlayerData>) -> Vec2 {
    let player_x = player_data.as_ref().map(|data| data.x).unwrap_or(20.0);
    let player_y = player_data.as_ref().map(|data| data.y).unwrap_or(20.0);
    return Vec2::new(player_x, player_y);
}

pub fn save_player(data: &PlayerData) {
    let win = web_sys::window().unwrap();
    let local_storage = win.local_storage().unwrap().unwrap();
    let str = serde_json::to_string(data).unwrap();
    local_storage.set_item("data", str.as_str()).unwrap();
}

pub fn restore_player() -> Option<PlayerData> {
    let win = web_sys::window().unwrap();
    let local_storage = win.local_storage().unwrap().unwrap();
    if let Ok(Some(str)) = local_storage.get_item("data") {
        if let Ok(data) = serde_json::from_str::<PlayerData>(str.as_str()) {
            Some(data)
        } else {
            None
        }
    } else {
        None
    }
}
