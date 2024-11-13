use crate::{
    asset::GameAssets,
    constant::WAND_EDITOR_Z_INDEX,
    states::{GameMenuState, GameState},
};
use bevy::prelude::*;

use super::{inventory::spawn_inventory, spell_information::spawn_spell_information};

#[derive(Component)]
struct WandEditorRoot;

pub fn spawn_wand_editor(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            StateScoped(GameState::InGame),
            WandEditorRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(300.0),
                    top: Val::Px(200.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                background_color: Color::hsla(0.0, 0.0, 0.0, 0.8).into(),
                z_index: ZIndex::Global(WAND_EDITOR_Z_INDEX),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_inventory(&mut parent, &assets);

            spawn_spell_information(&mut parent, &assets);
        });
}

fn handle_e_key(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameMenuState>>,
    mut next: ResMut<NextState<GameMenuState>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        match state.get() {
            GameMenuState::Closed => {
                next.set(GameMenuState::WandEditOpen);
            }
            GameMenuState::WandEditOpen => {
                next.set(GameMenuState::Closed);
            }
            _ => {}
        }
    }
}

fn apply_wand_editor_visible(
    mut query: Query<&mut Visibility, With<WandEditorRoot>>,
    state: Res<State<GameMenuState>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = match state.get() {
            GameMenuState::WandEditOpen => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

pub struct WandEditorPlugin;

impl Plugin for WandEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_e_key, apply_wand_editor_visible).run_if(in_state(GameState::InGame)),
        );
    }
}
