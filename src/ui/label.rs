use crate::asset::GameAssets;
use crate::language::{Dict, M18NTtext};
use bevy::prelude::*;

pub fn spawn_label<'a>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    text: Dict<&'static str>,
) {
    parent.spawn((
        M18NTtext(text.to_string()),
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextFont {
            font_size: 40.0,
            font: assets.dotgothic.clone(),
            ..default()
        },
    ));
}

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, _app: &mut App) {}
}
