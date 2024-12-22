use crate::constant::POINTER_Z_INDEX;
use crate::states::GameMenuState;
use crate::{asset::GameAssets, states::GameState};
use crate::{controller::player::Player, entity::actor::Actor};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::AseUiSlice;

#[derive(Component)]
struct Pointer;

fn setup_pointer(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("pointer"),
        Pointer,
        GlobalZIndex(POINTER_Z_INDEX),
        AseUiSlice {
            name: "pointer".into(),
            aseprite: assets.atlas.clone(),
        },
    ));
}

fn update_pointer_image_by_angle(
    mut pointer_query: Query<&mut Node, With<Pointer>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut pointer_style) = pointer_query.get_single_mut() {
        if let Ok(window) = window_query.get_single() {
            if let Some(cursor_in_screen) = window.cursor_position() {
                // AsepriteSliceUiBundle に Aseprite のアンカーは効かないことに注意
                // スライスのサイズは 13ピクセルで、それを２倍に拡大してその半分だけずらして中央ぞろえするので -13
                pointer_style.left = Val::Px((cursor_in_screen.x - 13.0).floor());
                pointer_style.top = Val::Px((cursor_in_screen.y - 13.0).floor());
                pointer_style.display = Display::default();
            } else {
                pointer_style.display = Display::None;
            }
        }
    }
}

/// マウスポインタの位置を参照してプレイヤーアクターのポインターを設定します
/// この関数はプレイヤーのモジュールに移動する？
fn update_pointer_by_mouse(
    mut player_query: Query<(&mut Actor, &GlobalTransform), With<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    state: Res<State<GameMenuState>>,
) {
    if *state.get() != GameMenuState::Closed {
        return;
    }

    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        if let Ok(window) = q_window.get_single() {
            if let Some(cursor_in_screen) = window.cursor_position() {
                if let Ok((camera, camera_global_transform)) = camera_query.get_single() {
                    if let Ok(mouse_in_world) =
                        camera.viewport_to_world(camera_global_transform, cursor_in_screen)
                    {
                        player.pointer = mouse_in_world.origin.truncate()
                            - player_transform.translation().truncate();
                    }
                }
            }
        }
    }
}

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Setup), setup_pointer);

        app.add_systems(
            Update,
            update_pointer_by_mouse.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            Update,
            update_pointer_image_by_angle.run_if(
                in_state(GameState::InGame).or(in_state(GameState::MainMenu)
                    .or(in_state(GameState::NameInput))
                    .or(in_state(GameState::Ending))),
            ),
        );
    }
}
