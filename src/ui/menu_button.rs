use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::language::Dict;
use crate::states::GameState;
use crate::ui::hover_color::HoverColor;
use crate::ui::on_press::OnPress;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Component)]
struct MenuButtonText {
    text: Dict,
}

pub fn menu_button<'a>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    button_type: SystemId,
    w: f32,
    h: f32,
    text: Dict,
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
            parent.spawn((
                MenuButtonText { text },
                TextBundle::from_section(
                    "".to_string(),
                    TextStyle {
                        font_size: 40.0,
                        font: assets.dotgothic.clone(),
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ));
        });
}

fn update_text(config: Res<GameConfig>, mut query: Query<(&mut Text, &MenuButtonText)>) {
    for (mut text, label) in query.iter_mut() {
        text.sections[0].value = label.text.get(config.language).to_string();
    }
}

pub struct MenuButtonPlugin;

impl Plugin for MenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_text.run_if(in_state(GameState::InGame).or_else(in_state(GameState::NameInput))),
        );
    }
}
