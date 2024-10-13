use bevy::prelude::*;

#[derive(Component)]
pub struct Overlay {
    pub enabled: bool,
}

#[derive(Event)]
pub struct OverlayClosedEvent;

fn setup_overlay(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    commands.spawn((
        Name::new("overlay"),
        Overlay { enabled: true },
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 1.0).into(),
            z_index: ZIndex::Global(10000),
            ..Default::default()
        },
    ));
}

fn update_overlay(
    mut query: Query<(&mut Overlay, &mut BackgroundColor)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<OverlayClosedEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut overlay, _) in query.iter_mut() {
            overlay.enabled = !overlay.enabled;
        }
    }

    let (overlay, mut background) = query.single_mut();
    let a = background.0.alpha();
    let speed = 0.005;

    let updated = (1.0_f32).min((0.0_f32).max(if overlay.enabled {
        a - speed
    } else {
        a + speed
    }));

    let current = background.0.alpha();

    background.0.set_alpha(updated);

    if current < 1.0 && updated == 1.0 {
        event_writer.send(OverlayClosedEvent);
    }
}

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay);
        app.add_systems(Update, update_overlay);
        app.add_event::<OverlayClosedEvent>();
    }
}
