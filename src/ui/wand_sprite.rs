use crate::asset::GameAssets;
use bevy::prelude::*;
use bevy::ui::Node;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct WandSprite;

pub fn spawn_wand_sprite_in_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent.spawn((
        WandSprite,
        Interaction::default(),
        BackgroundColor(Color::hsla(0.0, 0.3, 0.5, 0.1)),
        Node {
            width: Val::Px(64.),
            height: Val::Px(32.),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "empty".into(),
        },
    ));
}

pub struct WandSpritePlugin;

impl Plugin for WandSpritePlugin {
    fn build(&self, _app: &mut App) {}
}
