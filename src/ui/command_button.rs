use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::language::Dict;
use crate::language::M18NTtext;
use crate::states::GameState;
use crate::ui::hover_color::HoverColor;
use bevy::prelude::*;

#[derive(Component)]
pub struct CommandButton {
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
    text: Dict<String>,
) {
    parent
        .spawn((
            CommandButton { disabled },
            marker,
            HoverColor {
                hovered: if disabled { DISABLED } else { HOVERED },
                none: if disabled { DISABLED } else { NONE },
            },
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

fn update_color(
    mut button_query: Query<(&CommandButton, &mut HoverColor)>,
    mut text_color: Query<(&Parent, &mut TextColor), With<MenuButtonText>>,
) {
    for (parent, mut text_color) in text_color.iter_mut() {
        let (button, mut hover) = button_query.get_mut(parent.get()).unwrap();
        text_color.0 = if button.disabled {
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
        app.add_systems(Update, (update_color).run_if(in_state(GameState::InGame)));
        app.add_systems(OnEnter(GameState::InGame), update_text_on_enter);
    }
}
