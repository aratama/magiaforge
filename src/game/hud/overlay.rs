use super::super::{constant::OVERLAY_Z_INDEX, states::GameState};
use bevy::prelude::*;

#[derive(Component)]
struct Overlay;

#[derive(Resource, Clone, PartialEq, Eq, Debug)]
pub struct OverlayNextState(pub Option<GameState>);

fn setup_overlay(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    commands.spawn((
        Name::new("overlay"),
        Overlay,
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 1.0).into(),
            z_index: ZIndex::Global(OVERLAY_Z_INDEX),
            ..Default::default()
        },
    ));
}

fn update_overlay(
    mut query: Query<&mut BackgroundColor, With<Overlay>>,
    mut state: ResMut<NextState<GameState>>,
    mut overlay_next_state: ResMut<OverlayNextState>,
) {
    let mut background = query.single_mut();
    let a = background.0.alpha();
    let speed = 0.005;

    let next = overlay_next_state.0.clone();
    let updated = (1.0_f32).min((0.0_f32).max(match next {
        Option::None => a - speed,
        _ => a + speed,
    }));

    let current = background.0.alpha();

    background.0.set_alpha(updated);

    if current < 1.0 && updated == 1.0 {
        match next {
            Option::Some(next_state) => {
                state.set(next_state);
                *overlay_next_state = OverlayNextState(Option::None);
            }
            _ => {}
        }
    }
}

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_overlay);
        app.add_systems(Update, update_overlay);
        app.insert_resource::<OverlayNextState>(OverlayNextState(Option::None));
    }
}
