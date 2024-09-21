use crate::player::*;
use bevy::{prelude::*, window::PrimaryWindow};
use iyes_perf_ui::entries::PerfUiBundle;

#[derive(Component)]
pub struct HUD;

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("Test", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
        HUD,
    ));
    commands.spawn(PerfUiBundle::default());
}

fn update_hud(
    q_window: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, (With<Person>, Without<Camera2d>)>,
    camera_query: Query<(&Camera, &Transform, &GlobalTransform), (With<Camera2d>, Without<Person>)>,
    mut hud_query: Query<&mut Text, With<HUD>>,
) {
    let window = q_window.single();
    let player = player_query.single();
    let (camera, camera_transform, camera_global_transform) = camera_query.single();
    let mut hud = hud_query.single_mut();

    let cursor = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor))
        .map(|ray| ray.origin.truncate())
        .unwrap_or(Vec2::ZERO);

    let text = format!(
        "Player: ({:.2}, {:.2})\nCamera: ({:.2}, {:.2})\nCursor: ({:.2}, {:.2})",
        player.translation.x,
        player.translation.y,
        camera_transform.translation.x,
        camera_transform.translation.y,
        cursor.x,
        cursor.y
    );
    hud.sections = vec![TextSection::from(text)];
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud);
        app.add_systems(Update, update_hud);
    }
}
