// https://github.com/bevyengine/bevy/blob/ca8dd061467d44da358ae116d1b6da03e917aaa6/examples/window/monitor_info.rs

use bevy::prelude::*;

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
