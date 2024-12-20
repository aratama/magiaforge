use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::language::Dict;
use crate::states::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct LabelText {
    text: Dict<&'static str>,
}

pub fn spawn_label<'a>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    text: Dict<&'static str>,
) {
    parent.spawn((
        LabelText { text },
        Text::new(""),
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextFont {
            font_size: 40.0,
            font: assets.dotgothic.clone(),
            ..default()
        },
    ));
}

fn update_text(config: Res<GameConfig>, mut query: Query<(&mut Text, &LabelText)>) {
    if config.is_changed() {
        for (mut text, label) in query.iter_mut() {
            text.0 = label.text.get(config.language).to_string();
        }
    }
}

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_text.run_if(in_state(GameState::InGame)));
    }
}
