use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::component::counter::CounterAnimated;
use crate::language::language_to_font;
use crate::states::GameState;
use crate::theater::Act;
use crate::theater::Theater;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseUiAnimation;
use bevy_aseprite_ultra::prelude::AseUiSlice;

const SCALE: f32 = 3.0;

const SPEECH_BUBBLE_WIDTH: f32 = 160.0;

const SPEECH_BUBBLE_HEIGHT: f32 = 64.0;

const DELAY: usize = 4;

#[derive(Component)]
pub struct SpeechBubble {
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct SpeechBubbleText;

#[derive(Component)]
struct NextPage;

pub fn spawn_speech_bubble(parent: &mut Commands, assets: &Res<GameAssets>) {
    parent
        .spawn((
            StateScoped(GameState::InGame),
            SpeechBubble { entity: None },
            AseUiSlice {
                aseprite: assets.atlas.clone(),
                name: "speech_bubble".into(),
            },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(SPEECH_BUBBLE_WIDTH * SCALE),
                height: Val::Px(SPEECH_BUBBLE_HEIGHT * SCALE),
                ..default()
            },
            ZIndex(100),
            Visibility::Hidden,
        ))
        .with_child((
            SpeechBubbleText,
            Text::new(""),
            TextColor(Color::hsva(0.0, 0.0, 0.1, 1.0)),
            TextFont {
                font: assets.noto_sans_jp.clone(),
                font_size: 24.0,
                ..default()
            },
            BorderColor(Color::hsv(0.0, 1.0, 1.0)),
            Node {
                left: Val::Px(16.0),
                top: Val::Px(16.0),
                width: Val::Px(150.0 * SCALE),
                height: Val::Px(46.0 * SCALE),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
        ))
        .with_child((
            NextPage,
            CounterAnimated,
            AseUiAnimation {
                aseprite: assets.next_page.clone(),
                animation: "default".into(),
            },
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(16.0),
                bottom: Val::Px(16.0),
                width: Val::Px(16.0 * SCALE),
                height: Val::Px(16.0 * SCALE),
                ..default()
            },
        ));
}

pub fn update_speech_bubble_position(
    mut speech_query: Query<(&mut Node, &SpeechBubble)>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<SpeechBubble>>,
    rabbit_query: Query<&GlobalTransform, (Without<SpeechBubble>, Without<Camera>)>,
) {
    if let Ok((mut speech_node, speech)) = speech_query.get_single_mut() {
        if let Some(entity) = speech.entity {
            if let Ok(rabbit) = rabbit_query.get(entity) {
                let (camera, camera_transform) = camera_query.single();
                if let Ok(p) = camera.world_to_viewport(
                    camera_transform,
                    rabbit.translation() + Vec3::new(0.0, 20.0, 0.0),
                ) {
                    speech_node.left = Val::Px(p.x - SPEECH_BUBBLE_WIDTH * 0.5 * SCALE);
                    speech_node.top = Val::Px(p.y - 128.0 * 0.5 * SCALE);
                }
            }
        }
    }
}

fn next_page_visibility(
    mut query: Query<&mut Visibility, With<NextPage>>,
    config: Res<GameConfig>,
    theater: Res<Theater>,
) {
    let mut visibility = query.single_mut();
    match theater.current_act() {
        Some(Act::Speech(dict)) => {
            let page_string = dict.get(config.language);
            let chars = page_string.char_indices();
            let count = chars.count();
            let pos = theater.speech_count / DELAY;
            *visibility = if pos < count {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
        _ => {}
    }
}

pub fn update_text_on_change_config(
    config: Res<GameConfig>,
    mut speech_text_query: Query<(&mut Text, &mut TextFont), With<SpeechBubbleText>>,
    theater: Res<Theater>,
    assets: Res<GameAssets>,
) {
    match theater.senario.get(theater.act_index) {
        Some(Act::Speech(dict)) => {
            let text_end_position = theater.speech_count / DELAY;
            let (mut speech_text, mut font) = speech_text_query.single_mut();
            let page_string = dict.get(config.language);
            speech_text.0 = page_string.chars().take(text_end_position).collect();
            font.font = language_to_font(&assets, config.language);
        }
        _ => {}
    }
}

pub struct SpeechBubblePlugin;

impl Plugin for SpeechBubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (update_speech_bubble_position).chain(),
                next_page_visibility,
                update_text_on_change_config,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
