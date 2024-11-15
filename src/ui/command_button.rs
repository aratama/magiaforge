use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::language::Dict;
use crate::states::GameState;
use crate::ui::hover_color::HoverColor;
use bevy::prelude::*;

#[derive(Component)]
pub struct CommandButton {
    pub text: Dict,
    pub disabled: bool,
}

#[derive(Component)]
struct MenuButtonText;

const HOVERED: Color = Color::hsla(0.0, 0.0, 1.0, 0.1);
const NONE: Color = Color::hsla(0.0, 0.0, 1.0, 0.05);
const DISABLED: Color = Color::hsla(0.0, 0.0, 1.0, 0.01);

pub fn command_button<'a, T: Component>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    marker: T,
    w: f32,
    h: f32,
    disabled: bool,
    text: Dict,
) {
    parent
        .spawn((
            CommandButton { text, disabled },
            marker,
            HoverColor {
                hovered: if disabled { DISABLED } else { HOVERED },
                none: if disabled { DISABLED } else { NONE },
            },
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
                MenuButtonText,
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

fn update_text(
    config: Res<GameConfig>,
    button_query: Query<&CommandButton>,
    mut text_query: Query<(&Parent, &mut Text), With<MenuButtonText>>,
) {
    if config.is_changed() {
        for (parent, mut text) in text_query.iter_mut() {
            let button = button_query.get(parent.get()).unwrap();
            text.sections[0].value = button.text.get(config.language).to_string();
        }
    }
}

fn update_color(
    mut button_query: Query<(&CommandButton, &mut HoverColor)>,
    mut text_color: Query<(&Parent, &mut Text), With<MenuButtonText>>,
) {
    for (parent, mut text) in text_color.iter_mut() {
        let (button, mut hover) = button_query.get_mut(parent.get()).unwrap();
        text.sections[0].style.color = if button.disabled {
            Color::hsla(0.0, 0.0, 1.0, 0.2)
        } else {
            Color::WHITE
        };
        hover.none = if button.disabled { DISABLED } else { NONE };
        hover.hovered = if button.disabled { DISABLED } else { HOVERED };
    }
}

fn update_text_on_enter(mut config: ResMut<GameConfig>) {
    // ハック
    // update_textでは config が変更されたときに呼ばれるが、
    // タイトル画面に戻ってから再開すると、config が変更されていないため呼ばれず、
    // ラベルが正しく更新されない
    // これを避けるため、config をわざと上書きして強制的に呼び出す
    config.language = config.language;
}

pub struct CommandButtonPlugin;

impl Plugin for CommandButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_text, update_color).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnEnter(GameState::InGame), update_text_on_enter);
    }
}
