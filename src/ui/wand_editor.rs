use crate::{asset::GameAssets, constant::WAND_EDITOR_Z_INDEX, states::GameState};
use bevy::prelude::*;

use super::inventory::spawn_inventory;

#[derive(Component)]
pub struct WandEditorRoot;

pub fn spawn_wand_editor(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            StateScoped(GameState::InGame),
            WandEditorRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    left: Val::Px(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                z_index: ZIndex::Global(WAND_EDITOR_Z_INDEX),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_inventory(&mut parent, &assets);
        });
}

fn toggle_wand_editor_visible(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<WandEditorRoot>>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        for mut visibility in query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Inherited => Visibility::Visible,
            };
        }
    }
}

pub struct WandEditorPlugin;

impl Plugin for WandEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_wand_editor_visible.run_if(in_state(GameState::InGame)),
        );
    }
}
