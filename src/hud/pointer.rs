use crate::asset::GameAssets;
use crate::constant::POINTER_Z_INDEX;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Setup), setup_pointer);

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
