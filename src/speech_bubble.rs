use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::entity::rabbit::Rabbit;
use crate::se::{SECommand, SE};
use crate::states::GameState;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy_aseprite_ultra::prelude::AseUiSlice;

const SCALE: f32 = 3.0;

const SPEECH_BUBBLE_WIDTH: f32 = 128.0;

const SPEECH_BUBBLE_HEIGHT: f32 = 64.0;

#[derive(Component)]
pub struct SpeechBubble {
    count: usize,
    text: String,
}

#[derive(Component)]
pub struct SpeechBubbleText;

#[derive(Event)]
pub enum SpeechEvent {
    Speech(String),
    Close,
}

pub fn spawn_speech_bubble(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            SpeechBubble {
                count: 0,
                text: "".to_string(),
            },
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
                font: assets.dotgothic.clone(),
                font_size: 24.0,
                font_smoothing: FontSmoothing::AntiAliased,
            },
            Node {
                left: Val::Px(16.0),
                top: Val::Px(16.0),
                ..default()
            },
        ));
}

fn update_speech_bubble(
    mut speech_query: Query<&mut Node, With<SpeechBubble>>,
    rabbit_query: Query<&GlobalTransform, With<Rabbit>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok(mut speech) = speech_query.get_single_mut() {
        if let Ok(rabbit) = rabbit_query.get_single() {
            let (camera, camera_transform) = camera_query.single();
            if let Ok(p) = camera.world_to_viewport(
                camera_transform,
                rabbit.translation() + Vec3::new(0.0, 20.0, 0.0),
            ) {
                speech.left = Val::Px(p.x - 128.0 * 0.5 * SCALE);
                speech.top = Val::Px(p.y - 128.0 * 0.5 * SCALE);
            }
        }
    }
}

fn read_speech_events(
    mut events: EventReader<SpeechEvent>,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut camera_query: Query<&mut GameCamera>,
) {
    let mut camera = camera_query.single_mut();

    for event in events.read() {
        let (mut visibility, mut speech) = speech_query.single_mut();

        match event {
            SpeechEvent::Speech(s) => {
                camera.scale_factor = -2.0;
                *visibility = Visibility::Inherited;
                speech.count = 0;
                speech.text = s.clone();
            }
            SpeechEvent::Close => {
                camera.scale_factor = -1.0;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn countup(
    mut speech_query: Query<(&Visibility, &mut SpeechBubble)>,
    mut text_query: Query<&mut Text, With<SpeechBubbleText>>,
    mut se: EventWriter<SECommand>,
) {
    let (visibility, mut speech) = speech_query.single_mut();

    if *visibility == Visibility::Inherited {
        const DELAY: usize = 4;
        speech.count += 1;
        let count = speech.count / DELAY;
        let mut text = text_query.single_mut();
        let mut chars = speech.text.char_indices();
        let length = speech.text.char_indices().count();
        let (i, _) = chars.nth(count.min(length - 1)).unwrap();
        text.0 = speech.text[..i].to_string();
        speech.count = speech.count.min(length * DELAY + 1);
        if speech.count % DELAY == 0 {
            se.send(SECommand::new(SE::Kawaii));
        }
    }
}

pub struct SpeechBubblePlugin;

impl Plugin for SpeechBubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpeechEvent>();
        app.add_systems(
            Update,
            (update_speech_bubble, read_speech_events, countup).run_if(in_state(GameState::InGame)),
        );
    }
}
