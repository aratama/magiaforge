use super::hover_color::HoverColor;
use super::on_press::OnPress;
use crate::game::asset::GameAssets;
use crate::game::states::GameState;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

const SCALE: f32 = 4.0;

pub fn button<'a>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    button_type: SystemId,
    scope: GameState,
    text: impl Into<String>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let hovered = Color::hsla(0.0, 0.0, 1.0, 0.1).into();
    let none = Color::hsla(0.0, 0.0, 1.0, 0.0).into();

    parent
        .spawn((
            HoverColor { hovered, none },
            OnPress(button_type),
            StateScoped(scope),
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(SCALE * w),
                    height: Val::Px(SCALE * h),
                    left: Val::Px(SCALE * x),
                    top: Val::Px(SCALE * y),
                    border: UiRect::all(Val::Px(4.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: Color::hsla(0.0, 1.0, 1.0, 0.1).into(),
                border_radius: BorderRadius::DEFAULT,
                background_color: Color::hsla(0.0, 1.0, 1.0, 0.0).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 60.0,
                    font: assets.dotgothic.clone(),
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}
