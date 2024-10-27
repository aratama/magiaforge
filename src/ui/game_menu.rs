use crate::audio::play_se;
use crate::constant::GAME_MENU_Z_INDEX;
use crate::gamepad::MyGamepad;
use crate::hud::overlay::OverlayNextState;
use crate::states::GameMenuState;
use crate::ui::button::button;
use crate::{asset::GameAssets, states::GameState};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

#[derive(Resource)]
struct ButtonShots {
    close: SystemId,
    exit: SystemId,
}

#[derive(Component)]
struct GameMenuRoot;

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            close: world.register_system(close),
            exit: world.register_system(exit),
        }
    }
}

fn close(mut state: ResMut<NextState<GameMenuState>>, assets: Res<GameAssets>, audio: Res<Audio>) {
    state.set(GameMenuState::Close);
    play_se(assets.kettei.clone(), &audio);
}

fn exit(
    mut overlay_next_state: ResMut<OverlayNextState>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
) {
    *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
    play_se(assets.kettei.clone(), &audio);
}

fn setup_game_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    shots: Res<ButtonShots>,
) {
    let mut camera = camera_query.single_mut();
    camera.translation.x = 0.0;
    camera.translation.y = 0.0;

    const MARGIN: f32 = 150.0;

    commands
        .spawn((
            GameMenuRoot,
            StateScoped(GameState::InGame),
            Name::new("main menu"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(MARGIN),
                    top: Val::Px(MARGIN),
                    width: Val::Px(1280.0 - MARGIN * 2.0),
                    height: Val::Px(720.0 - MARGIN * 2.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                background_color: Color::hsla(219.0, 0.5, 0.1, 0.9).into(),
                border_color: Color::hsla(1.0, 1.0, 1.0, 0.5).into(),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                z_index: ZIndex::Global(GAME_MENU_Z_INDEX),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            button(
                parent,
                &assets,
                shots.exit,
                GameState::InGame,
                "Back to Main Menu",
                10.0,
                10.0,
                124.0,
                16.0,
            );

            button(
                parent,
                &assets,
                shots.close,
                GameState::InGame,
                "Close",
                200.0,
                10.0,
                40.0,
                16.0,
            );
        });
}

fn update_game_menu(
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
    mut query: Query<&mut Visibility, With<GameMenuRoot>>,

    keys: Res<ButtonInput<KeyCode>>,

    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let mut visibility = query.single_mut();
    *visibility = match state.get() {
        GameMenuState::Close => Visibility::Hidden,
        GameMenuState::Open => Visibility::Visible,
    };

    if keys.just_pressed(KeyCode::Escape) {
        next.set(match state.get() {
            GameMenuState::Close => GameMenuState::Open,
            GameMenuState::Open => GameMenuState::Close,
        });
    }

    if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
        if gamepad_buttons.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        }) {
            next.set(match state.get() {
                GameMenuState::Close => GameMenuState::Open,
                GameMenuState::Open => GameMenuState::Close,
            });
        }
    }
}

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game_menu);
        app.add_systems(Update, update_game_menu.run_if(in_state(GameState::InGame)));
        app.init_resource::<ButtonShots>();
    }
}
