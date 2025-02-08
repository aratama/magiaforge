use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorState;
use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::config::GameConfig;
use crate::controller::player::Player;
use crate::language::language_to_font;
use crate::language::Dict;
use crate::language::Languages;
use crate::registry::Registry;
use crate::script::context::JavaScriptContext;
use crate::se::SEEvent;
use crate::se::KAWAII;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
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
    pub dict: Dict<String>,
    pub speech_count: usize,
}

#[derive(Component)]
pub struct SpeechBubbleText;

#[derive(Component)]
struct NextPage;

pub fn spawn_speech_bubble(parent: &mut Commands, registry: &Registry) {
    parent
        .spawn((
            StateScoped(GameState::InGame),
            SpeechBubble {
                entity: None,
                dict: Dict::default(),
                speech_count: 0,
            },
            AseUiSlice {
                aseprite: registry.assets.atlas.clone(),
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
            TextColor(Color::hsva(0.0, 0.0, 0.3, 1.0)),
            TextFont {
                font: registry.assets.noto_sans_jp.clone(),
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
                aseprite: registry.assets.next_page.clone(),
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
    speech_query: Query<&SpeechBubble>,
    mut query: Query<&mut Visibility, With<NextPage>>,
    config: Res<GameConfig>,
) {
    let speech = speech_query.single();
    let mut visibility = query.single_mut();
    let page_string = speech.dict.get(config.language);
    let chars = page_string.char_indices();
    let count = chars.count();
    let pos = speech.speech_count / DELAY;
    *visibility = if pos < count {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };
}

pub fn update_text_on_change_config(
    speech_query: Query<&SpeechBubble>,
    config: Res<GameConfig>,
    mut speech_text_query: Query<(&mut Text, &mut TextFont), With<SpeechBubbleText>>,
    assets: Res<GameAssets>,
) {
    let speech = speech_query.single();
    let text_end_position = speech.speech_count / DELAY;
    let (mut speech_text, mut font) = speech_text_query.single_mut();
    let page_string = speech.dict.get(config.language);
    speech_text.0 = page_string.chars().take(text_end_position).collect();
    font.font = language_to_font(&assets, config.language);
}

fn speech_countup(
    mut speech_query: Query<(&Visibility, &mut SpeechBubble)>,
    config: Res<GameConfig>,
    mut player_query: Query<(&mut Actor, &Player)>,
    mut se_writer: EventWriter<SEEvent>,
) {
    let (speech_visibility, mut speech) = speech_query.single_mut();

    if *speech_visibility == Visibility::Hidden {
        return;
    }

    if let Ok((mut actor, _)) = player_query.get_single_mut() {
        actor.state = ActorState::Idle;
        actor.fire_state = ActorFireState::Idle;
    }

    let text_end_position = speech.speech_count / DELAY;
    let page_string = speech.dict.get(config.language);

    if text_end_position < page_string.char_indices().count() {
        let step = match config.language {
            Languages::Ja => 1,
            Languages::ZhCn => 1,
            _ => 2,
        };

        if speech.speech_count % (DELAY * step) == 0 {
            se_writer.send(SEEvent::new(KAWAII));
        }

        speech.speech_count += step;
    }
}

fn next_page(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bubble_query: Query<(&Visibility, &mut SpeechBubble)>,
    config: Res<GameConfig>,
    state: Res<State<GameMenuState>>,
    mut script: NonSendMut<JavaScriptContext>,
) {
    let (bubble_visivility, mut bubble) = bubble_query.single_mut();
    if *bubble_visivility == Visibility::Inherited && *state != GameMenuState::PauseMenuOpen {
        if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            let page_string = bubble.dict.get(config.language);
            let chars = page_string.char_indices();
            let count = chars.count();
            let pos = bubble.speech_count / DELAY;
            if pos < count {
                bubble.speech_count = count * DELAY;
            } else {
                script.resume();
            }
        }
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
        app.add_systems(
            Update,
            (next_page, speech_countup)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
    }
}
