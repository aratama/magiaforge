use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*,
};

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}

pub fn get_direction(
    keys: Res<ButtonInput<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: &Option<Res<MyGamepad>>,
) -> Vec2 {
    let key_direction = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero();

    let gamepad_direction = match my_gamepad.as_deref() {
        Some(&MyGamepad(gamepad)) => {
            // The joysticks are represented using a separate axis for X and Y
            let axis_lx = GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            };
            let axis_ly = GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickY,
            };

            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                // combine X and Y into one vector
                Vec2::new(x, y)
            } else {
                Vec2::ZERO
            }
        }
        None => Vec2::ZERO,
    }
    .normalize_or_zero();

    (key_direction + gamepad_direction).normalize_or_zero()
}

pub fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}

pub fn get_fire_trigger(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: &Option<Res<MyGamepad>>,
) -> bool {
    mouse_buttons.pressed(MouseButton::Left)
        || match my_gamepad.as_deref() {
            Some(&MyGamepad(gamepad)) => {
                gamepad_buttons.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::South,
                }) || gamepad_buttons.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::RightTrigger,
                })
            }
            None => false,
        }
}

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, gamepad_connections);
    }
}
