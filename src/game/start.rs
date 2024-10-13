use super::{
    overlay::{Overlay, OverlayClosedEvent},
    states::GameState,
};
use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct StartPage;

#[derive(Bundle)]
pub struct StartPageBundle {
    start: StartPage,
}

pub fn setup_start_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
) {
    commands
        .spawn((
            StateScoped(GameState::MainMenu),
            StartPage,
            Name::new("start page"),
            NodeBundle {
                style: Style {
                    width: Val::Px(1280.0),
                    height: Val::Px(720.0),
                    ..Default::default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 1.0).into(),
                visibility: match state.get() {
                    GameState::MainMenu => Visibility::Visible,
                    _ => Visibility::Hidden,
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("title.png")),
                ..default()
            });
        });
}

pub fn update_start_page(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Visibility), With<StartPage>>,
    mut overlay_query: Query<&mut Overlay>,
    mut event_overlay_closed: EventReader<OverlayClosedEvent>,

    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut overlay) = overlay_query.get_single_mut() {
        if let Ok((_, mut visibility)) = query.get_single_mut() {
            if *visibility == Visibility::Visible {
                if keys.just_pressed(KeyCode::Enter) || buttons.just_pressed(MouseButton::Left) {
                    overlay.enabled = false;
                }
            }
            for _ in event_overlay_closed.read() {
                *visibility = Visibility::Hidden;

                overlay.enabled = true;

                next_state.set(GameState::InGame);
            }
        }
    }
}

pub struct StartPagePlugin;

impl Plugin for StartPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_start_page);
        app.add_systems(
            FixedUpdate,
            update_start_page.run_if(in_state(GameState::MainMenu)),
        );
    }
}
