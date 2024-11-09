use crate::asset::GameAssets;
use crate::ui::hover_color::HoverColor;
use crate::ui::on_press::OnPress;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

pub fn menu_button<'a>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    button_type: SystemId,
    text: impl Into<String>,
    w: f32,
    h: f32,
) {
    let hovered = Color::hsla(0.0, 0.0, 1.0, 0.1).into();
    let none = Color::hsla(0.0, 0.0, 1.0, 0.05).into();

    parent
        .spawn((
            HoverColor { hovered, none },
            OnPress(button_type),
            ButtonBundle {
                style: Style {
                    width: Val::Px(w),
                    height: Val::Px(h),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::hsla(0.0, 1.0, 1.0, 0.0).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    font: assets.dotgothic.clone(),
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}
