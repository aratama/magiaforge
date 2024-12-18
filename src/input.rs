use bevy::prelude::*;

pub fn get_direction(keys: Res<ButtonInput<KeyCode>>) -> Vec2 {
    let key_direction = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero();

    let merged = key_direction;
    if 1.0 < merged.length() {
        merged.normalize_or_zero()
    } else {
        merged
    }
}

pub fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}

pub fn get_fire_trigger(mouse_buttons: &Res<ButtonInput<MouseButton>>) -> bool {
    mouse_buttons.pressed(MouseButton::Left)
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, _app: &mut App) {}
}
