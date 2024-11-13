use crate::constant::POINTER_Z_INDEX;
use crate::states::GameMenuState;
use crate::{asset::GameAssets, constant::TILE_SIZE, input::MyGamepad, states::GameState};
use crate::{controller::player::Player, entity::actor::Actor};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::AsepriteSliceUiBundle;

#[derive(Component)]
struct Pointer;

fn setup_pointer(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Pointer,
        StateScoped(GameState::InGame),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Px(13.0 * 2.0),
                height: Val::Px(13.0 * 2.0),

                ..default()
            },
            z_index: ZIndex::Global(POINTER_Z_INDEX),
            ..default()
        },
        AsepriteSliceUiBundle {
            slice: "pointer".into(),
            aseprite: assets.atlas.clone(),
            ..default()
        },
    ));
}

fn update_pointer_image_by_angle(
    player_query: Query<(&Actor, &GlobalTransform), With<Player>>,
    mut pointer_query: Query<&mut Style, With<Pointer>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
) {
    if let Ok((player, player_transform)) = player_query.get_single() {
        if let Ok(mut pointer_style) = pointer_query.get_single_mut() {
            if let Ok((camera, camera_global_transform)) = camera_query.get_single() {
                let pointer_in_world = player_transform.translation().truncate() + player.pointer;

                if let Some(pointer_in_screen) =
                    camera.world_to_viewport(camera_global_transform, pointer_in_world.extend(0.0))
                {
                    // AsepriteSliceUiBundle に Aseprite のアンカーは効かないことに注意
                    // スライスのサイズは 13ピクセルで、それを２倍に拡大してその半分だけずらして中央ぞろえするので -13
                    pointer_style.left = Val::Px(pointer_in_screen.x - 13.0);
                    pointer_style.top = Val::Px(pointer_in_screen.y - 13.0);
                }
            }
        }
    }
}

fn update_pointer_by_mouse(
    mut player_query: Query<(&mut Actor, &GlobalTransform), With<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    state: Res<State<GameMenuState>>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        if let Ok(window) = q_window.get_single() {
            if let Some(cursor_in_screen) = window.cursor_position() {
                if let Ok((camera, camera_global_transform)) = camera_query.get_single() {
                    if let Some(mouse_in_world) =
                        camera.viewport_to_world(camera_global_transform, cursor_in_screen)
                    {
                        if *state.get() == GameMenuState::Closed {
                            player.pointer = mouse_in_world.origin.truncate()
                                - player_transform.translation().truncate();
                        }
                    }
                }
            }
        }
    }
}

fn update_pointer_by_gamepad(
    mut player_query: Query<&mut Actor, With<Player>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        match my_gamepad.as_deref() {
            Some(&MyGamepad(gamepad)) => {
                let axis_lx = GamepadAxis {
                    gamepad,
                    axis_type: GamepadAxisType::RightStickX,
                };
                let axis_ly = GamepadAxis {
                    gamepad,
                    axis_type: GamepadAxisType::RightStickY,
                };
                if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                    let normalized = Vec2::new(x, y).normalize_or_zero();
                    if 0.2 < normalized.length() {
                        player.pointer = normalized * TILE_SIZE * 4.0;
                    }
                }
            }
            None => {}
        }
    }
}

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_pointer)
            .add_systems(
                Update,
                (
                    update_pointer_image_by_angle,
                    update_pointer_by_mouse,
                    update_pointer_by_gamepad,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
