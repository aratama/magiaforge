use crate::ui::menu_button::menu_button;
use crate::{asset::GameAssets, states::GameState};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

pub fn spawn_range<T: Component>(
    child_builder: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    marker: T,
    label: &str,
    value: u32,
    up: SystemId,
    down: SystemId,
) {
    child_builder
        .spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    border: UiRect::all(Val::Px(2.0)),
                    width: Val::Percent(100.0),
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 40.0,
                    font: assets.dotgothic.clone(),
                    color: Color::srgba(0.9, 0.9, 0.9, 0.4),
                    ..default()
                },
            ));
            menu_button(parent, &assets, down, "▼", 40.0, 40.0);
            parent.spawn((
                marker,
                TextBundle::from_section(
                    format!("{}", value).to_string(),
                    TextStyle {
                        font_size: 40.0,
                        font: assets.dotgothic.clone(),
                        color: Color::srgba(0.9, 0.9, 0.9, 0.4),
                        ..default()
                    },
                ),
            ));
            menu_button(parent, &assets, up, "▲", 40.0, 40.0);
        });
}
