use crate::constant::OVERLAY_Z_INDEX;
use crate::states::GameState;
use bevy::prelude::*;

const SPEED: f32 = 0.02;

#[derive(Event)]
pub enum OverlayEvent {
    Close(GameState),
}

#[derive(Component)]
struct Overlay {
    open: bool,
    alpha: f32,
    next: Option<GameState>,
    wait: i32,
}

fn setup_overlay(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    commands.spawn((
        Name::new("overlay"),
        Overlay {
            open: true,
            alpha: 1.0,
            next: None,
            wait: 0,
        },
        GlobalZIndex(OVERLAY_Z_INDEX),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
    ));
}

fn read_overlay_event(mut query: Query<&mut Overlay>, mut reader: EventReader<OverlayEvent>) {
    let mut overlay = query.single_mut();

    for event in reader.read() {
        match event {
            OverlayEvent::Close(next) => {
                overlay.open = false;
                overlay.next = Some(*next);
                overlay.wait = 30;
            }
        }
    }
}

fn update_overlay(
    mut query: Query<(&mut Overlay, &mut BackgroundColor)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut overlay, mut background) = query.single_mut();

    let current = overlay.alpha;

    let updated = (1.0_f32).min((0.0_f32).max(if overlay.open {
        current - SPEED
    } else {
        current + SPEED
    }));

    overlay.alpha = updated;

    background.0.set_alpha(updated);

    if updated == 1.0 {
        overlay.wait -= 1;
    }

    if overlay.wait == 0 {
        match overlay.next {
            Some(next) => {
                next_state.set(next);
                overlay.open = true;
            }
            _ => {}
        }
    }
}

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay);
        app.add_systems(
            Update,
            (
                read_overlay_event,
                update_overlay.after(read_overlay_event).run_if(
                    in_state(GameState::InGame)
                        .or(in_state(GameState::MainMenu))
                        .or(in_state(GameState::NameInput))
                        .or(in_state(GameState::Warp))
                        .or(in_state(GameState::Ending)),
                ),
            ),
        );
        app.add_event::<OverlayEvent>();
    }
}
