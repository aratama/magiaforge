use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::config::GameConfig;
use crate::hud::overlay::OverlayEvent;
use crate::language::M18NTtext;
use crate::message::INPUT_YOUR_NAME;
use crate::message::START;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::states::MainMenuPhase;
use crate::ui::menu_button::menu_button;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy_simple_text_input::TextInput;
use bevy_simple_text_input::TextInputSettings;
use bevy_simple_text_input::TextInputTextColor;
use bevy_simple_text_input::TextInputTextFont;
use bevy_simple_text_input::TextInputValue;

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Resource)]
struct ButtonShots {
    start: SystemId,
}

impl FromWorld for ButtonShots {
    fn from_world(world: &mut World) -> Self {
        ButtonShots {
            start: world.register_system(start_game),
        }
    }
}

fn start_game(
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut config: ResMut<GameConfig>,
    text_input_query: Query<&TextInputValue>,
    mut writer: EventWriter<SEEvent>,
    mut next_bgm: ResMut<NextBGM>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
) {
    menu_next_state.set(MainMenuPhase::Paused);
    overlay_event_writer.send(OverlayEvent::Close(GameState::InGame));
    *next_bgm = NextBGM(None);

    let TextInputValue(text_input) = text_input_query.single();
    config.player_name = text_input.clone();

    writer.send(SEEvent::new(SE::Click));
}

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    shots: Res<ButtonShots>,
    config: Res<GameConfig>,
    mut overlay: EventWriter<OverlayEvent>,
) {
    overlay.send(OverlayEvent::SetOpen(true));

    commands.spawn((Camera2d::default(), StateScoped(GameState::NameInput)));

    commands
        .spawn((
            Name::new("name_input"),
            StateScoped(GameState::NameInput),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        M18NTtext(INPUT_YOUR_NAME.to_string()),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextFont {
                            font_size: 60.0,
                            font: assets.noto_sans_jp.clone(),
                            ..default()
                        },
                    ));

                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(200.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                BorderColor::from(BORDER_COLOR_ACTIVE),
                                BackgroundColor::from(BACKGROUND_COLOR),
                                TextInput,
                                TextInputTextFont(TextFont {
                                    font: assets.noto_sans_jp.clone(),
                                    font_size: 40.,
                                    ..default()
                                }),
                                TextInputTextColor(TEXT_COLOR.into()),
                                TextInputSettings {
                                    retain_on_submit: true,
                                    ..default()
                                },
                                TextInputValue(config.player_name.clone()),
                            ));

                            menu_button(
                                parent,
                                &assets,
                                shots.start,
                                160.0,
                                60.0,
                                START.to_string(),
                            );
                        });
                });
        });
}

pub struct NameInputPagePlugin;

impl Plugin for NameInputPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::NameInput), setup);
        app.init_resource::<ButtonShots>();
    }
}
