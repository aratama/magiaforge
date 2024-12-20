use super::label::spawn_label;
use crate::asset::GameAssets;
use crate::language::Dict;
use crate::ui::menu_button::menu_button;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

pub fn spawn_range<T: Component>(
    child_builder: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    marker: T,
    value: u32,
    up: SystemId,
    down: SystemId,
    label: Dict<&'static str>,
) {
    child_builder
        .spawn((Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(100.0),
            column_gap: Val::Px(10.0),
            ..default()
        },))
        .with_children(|parent| {
            spawn_label(parent, assets, label);

            menu_button(
                parent,
                &assets,
                down,
                40.0,
                40.0,
                Dict {
                    ja: "▼", en: "▼"
                },
            );
            parent.spawn((
                marker,
                Text::new(format!("{}", value)),
                TextColor(Color::srgba(0.9, 0.9, 0.9, 0.4)),
                TextFont {
                    font_size: 40.0,
                    font: assets.dotgothic.clone(),
                    ..default()
                },
            ));
            menu_button(
                parent,
                &assets,
                up,
                40.0,
                40.0,
                Dict {
                    ja: "▲", en: "▲"
                },
            );
        });
}
