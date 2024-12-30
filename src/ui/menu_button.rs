use crate::asset::GameAssets;
use crate::language::{Dict, M18NTtext};
use crate::ui::hover_color::HoverColor;
use crate::ui::on_press::OnPress;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Component)]
struct MenuButtonText;

pub fn menu_button<'a>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    button_type: SystemId,
    w: f32,
    h: f32,
    text: Dict<String>,
) {
    let hovered = Color::hsla(0.0, 0.0, 1.0, 0.1).into();
    let none = Color::hsla(0.0, 0.0, 1.0, 0.05).into();

    parent
        .spawn((
            HoverColor { hovered, none },
            OnPress(button_type),
            BackgroundColor(Color::hsla(0.0, 1.0, 1.0, 0.0)),
            Button,
            Node {
                width: Val::Px(w),
                height: Val::Px(h),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                MenuButtonText,
                M18NTtext(text),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextFont {
                    font_size: 32.0,
                    font: assets.noto_sans_jp.clone(),
                    ..default()
                },
            ));
        });
}

pub struct MenuButtonPlugin;

impl Plugin for MenuButtonPlugin {
    fn build(&self, _app: &mut App) {}
}
