use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::language::Dict;
use crate::states::GameState;
use crate::ui::hover_color::HoverColor;
use bevy::prelude::*;

#[derive(Component)]
pub struct CommandButton;

#[derive(Component)]
struct MenuButtonText {
    text: Dict,
}

pub fn command_button<'a, T: Component>(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    marker: T,
    w: f32,
    h: f32,
    text: Dict,
) {
    let hovered = Color::hsla(0.0, 0.0, 1.0, 0.1).into();
    let none = Color::hsla(0.0, 0.0, 1.0, 0.05).into();

    parent
        .spawn((
            CommandButton,
            marker,
            HoverColor { hovered, none },
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
    if config.is_changed() {
        for (mut text, label) in query.iter_mut() {
            text.sections[0].value = label.text.get(config.language).to_string();
        }
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
        app.add_systems(Update, update_text.run_if(in_state(GameState::InGame)));
        app.add_systems(OnEnter(GameState::InGame), update_text_on_enter);
    }
}
