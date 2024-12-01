use bevy::prelude::*;

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

pub fn get_direction(keys: Res<ButtonInput<KeyCode>>, gamepads: &Query<&Gamepad>) -> Vec2 {
    let key_direction = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero();

    let gamepad_direction = match gamepads.get_single() {
        Ok(gamepad) => {
            // The joysticks are represented using a separate axis for X and Y
            if let (Some(x), Some(y)) = (
                gamepad.get(GamepadAxis::LeftStickX),
                gamepad.get(GamepadAxis::LeftStickY),
            ) {
                // combine X and Y into one vector
                Vec2::new(x, y)
            } else {
                Vec2::ZERO
            }
        }
        _ => Vec2::ZERO,
    };

    let merged = key_direction
        + if 0.2 < gamepad_direction.length() {
            gamepad_direction
        } else {
            gamepad_direction.normalize_or_zero()
        };
    if 1.0 < merged.length() {
        merged.normalize_or_zero()
    } else {
        merged
    }
}

pub fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}

pub fn get_fire_trigger(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    gamepads: &Query<&Gamepad>,
) -> bool {
    mouse_buttons.pressed(MouseButton::Left)
        || match gamepads.get_single() {
            Ok(gamepad) => {
                gamepad.pressed(GamepadButton::South)
                    || gamepad.pressed(GamepadButton::RightTrigger)
            }
            _ => false,
        }
}

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {}
}
