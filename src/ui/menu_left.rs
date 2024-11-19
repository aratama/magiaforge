use bevy::prelude::*;

use crate::states::{GameMenuState, GameState};

#[derive(Component)]
pub struct MenuLeft {
    left: f32,
    open: f32,
    close: f32,
}

impl MenuLeft {
    pub fn new(open: f32, close: f32) -> Self {
        Self {
            left: close,
            open,
            close,
        }
    }
}

fn apply_wand_editor_visible(
    mut query: Query<(&mut MenuLeft, &mut Style)>,
    state: Res<State<GameMenuState>>,
) {
    for (mut root, mut style) in query.iter_mut() {
        let dest = match state.get() {
            GameMenuState::WandEditOpen => root.open,
            _ => root.close,
        };
        let left = root.left + (dest - root.left) * 0.2;
        style.left = Val::Px(left);
        root.left = left;
    }
}

pub struct MenuLeftPlugin;

impl Plugin for MenuLeftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_wand_editor_visible,).run_if(in_state(GameState::InGame)),
        );
    }
}
