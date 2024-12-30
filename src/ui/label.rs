use crate::asset::GameAssets;
use crate::language::Dict;
use crate::language::M18NTtext;
use bevy::prelude::*;

pub fn spawn_label<'a>(parent: &mut ChildBuilder, assets: &Res<GameAssets>, text: Dict<String>) {
    parent.spawn((
        M18NTtext(text),
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextFont {
            font_size: 40.0,
            font: assets.noto_sans_jp.clone(),
            ..default()
        },
    ));
}

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, _app: &mut App) {}
}
