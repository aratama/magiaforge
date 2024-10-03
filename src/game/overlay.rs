use bevy::prelude::*;

#[derive(Component)]
pub struct Overlay {
    enabled: bool,
}

fn setup_overlay(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    commands.spawn((
        Overlay { enabled: true },
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 1.0).into(),
            ..Default::default()
        },
    ));
}

fn update_overlay(
    mut query: Query<(&mut Overlay, &mut BackgroundColor)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut overlay, _) in query.iter_mut() {
            overlay.enabled = !overlay.enabled;
        }
    }

    let (overlay, mut background) = query.single_mut();
    let a = background.0.alpha();
    let speed = 0.005;
    background
        .0
        .set_alpha((1.0_f32).min((0.0_f32).max(if overlay.enabled {
            a - speed
        } else {
            a + speed
        })));
}

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay);
        app.add_systems(Update, update_overlay);
    }
}
