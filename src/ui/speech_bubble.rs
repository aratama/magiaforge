use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::camera::GameCamera;
use crate::config::GameConfig;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorState;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::Dict;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy_aseprite_ultra::prelude::AseUiAnimation;
use bevy_aseprite_ultra::prelude::AseUiSlice;

const SCALE: f32 = 3.0;

const SPEECH_BUBBLE_WIDTH: f32 = 160.0;

const SPEECH_BUBBLE_HEIGHT: f32 = 64.0;

const DELAY: usize = 4;

#[derive(Debug, Clone)]
pub enum SpeechAction {
    Focus(Entity),
    Speech(Dict<String>),
    BGM(Handle<AudioSource>),
    Close,

    #[allow(dead_code)]
    GetItem(InventoryItemType),
    #[allow(dead_code)]
    Wait(u32),
}

#[derive(Component)]
struct SpeechBubble {
    count: usize,
    pages: Vec<SpeechAction>,
    page: usize,
    entity: Option<Entity>,
    wait: u32,
}

#[derive(Component)]
pub struct SpeechBubbleText;

#[derive(Component)]
pub struct NextPage;

#[derive(Event)]
pub enum SpeechEvent {
    Speech { pages: Vec<SpeechAction> },
    Close,
}

pub fn spawn_speech_bubble(parent: &mut Commands, assets: &Res<GameAssets>) {
    parent
        .spawn((
            StateScoped(GameState::InGame),
            SpeechBubble {
                count: 0,
                pages: Vec::new(),
                page: 0,
                entity: None,
                wait: 0,
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

fn update_speech_bubble_position(
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

fn read_speech_events(
    mut events: EventReader<SpeechEvent>,
    mut speech_query: Query<(&mut SpeechBubble, &mut Visibility)>,
    mut player_query: Query<&mut Actor>,
) {
    for event in events.read() {
        let (mut speech, mut visibility) = speech_query.single_mut();
        match event {
            SpeechEvent::Speech { pages } => {
                speech.count = 0;
                speech.pages = pages.clone();
                speech.page = 0;
                if let Ok(mut player) = player_query.get_single_mut() {
                    player.state = ActorState::Idle;
                }
            }
            SpeechEvent::Close => {
                speech.entity = None;
                speech.pages.clear();
                speech.page = 0;
                speech.count = 0;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn countup(
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut speech_text_query: Query<&mut Text, With<SpeechBubbleText>>,
    mut se: EventWriter<SEEvent>,
    config: Res<GameConfig>,
    mut next_bgm: ResMut<NextBGM>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut camera: Query<&mut GameCamera>,
) {
    let (mut visibility, mut speech) = speech_query.single_mut();

    if speech.pages.len() <= speech.page {
        // let mut camera = camera.single_mut();
        // camera.target = None;
        return;
    }

    let pos = speech.count / DELAY;
    let mut speech_text = speech_text_query.single_mut();

    let page = speech.pages[speech.page].clone();
    match page {
        SpeechAction::Focus(speaker) => {
            speech.entity = Some(speaker);
            speech.page += 1;
        }
        SpeechAction::Speech(dict) => {
            *visibility = Visibility::Inherited;
            let page_string = dict.get(config.language);
            let chars = page_string.char_indices();
            let mut str = "".to_string();
            let mut s = 0;
            for (i, val) in chars.enumerate() {
                if i < pos {
                    str.push(val.1);
                }
                s += 1;
            }
            speech_text.0 = str;

            if pos < s {
                if speech.count % DELAY == 0 {
                    se.send(SEEvent::new(SE::Kawaii));
                }

                speech.count += 1;
            }
        }
        SpeechAction::BGM(bgm) => {
            next_bgm.0 = Some(bgm.clone());
            speech.page += 1;
        }
        SpeechAction::GetItem(item) => {
            if let Ok(mut actor) = player_query.get_single_mut() {
                actor.inventory.insert(InventoryItem {
                    item_type: item,
                    price: 0,
                });
            }
            speech.page += 1;
        }
        SpeechAction::Close => {
            *visibility = Visibility::Hidden;
            speech.page += 1;
            let mut camera = camera.single_mut();
            camera.target = None;
            if let Ok(mut actor) = player_query.get_single_mut() {
                actor.state = ActorState::Idle;
                actor.wait = 30;
            }
        }
        SpeechAction::Wait(wait) => {
            if speech.wait <= 0 {
                speech.wait = wait;
            } else {
                speech.wait -= 1;
                if speech.wait == 0 {
                    speech.page += 1;
                }
            }
        }
    }
}

fn next_page(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bubble_query: Query<(&mut SpeechBubble, &Visibility)>,
    mut writer: EventWriter<SpeechEvent>,
    config: Res<GameConfig>,
) {
    let (mut bubble, bubble_visivility) = bubble_query.single_mut();
    if *bubble_visivility == Visibility::Inherited {
        if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            let page = bubble.pages[bubble.page].clone();
            match page {
                SpeechAction::Speech(dict) => {
                    let page_string = dict.get(config.language);
                    let chars = page_string.char_indices();
                    let count = chars.count();
                    let pos = bubble.count / DELAY;
                    if pos < count {
                        bubble.count = count * DELAY;
                    } else if bubble.page < bubble.pages.len() - 1 {
                        bubble.page += 1;
                        bubble.count = 0;
                    } else {
                        writer.send(SpeechEvent::Close);
                    }
                }
                _ => {}
            }
        }
    }
}

fn next_page_visibility(
    mut query: Query<(&Parent, &mut Visibility), With<NextPage>>,
    bubble_query: Query<&SpeechBubble>,
    config: Res<GameConfig>,
) {
    let (parent, mut visibility) = query.single_mut();
    let bubble = bubble_query.get(parent.get()).unwrap();
    if let Some(page) = bubble.pages.get(bubble.page) {
        match page {
            SpeechAction::Speech(dict) => {
                let page_string = dict.get(config.language);
                let chars = page_string.char_indices();
                let count = chars.count();
                let pos = bubble.count / DELAY;
                *visibility = if pos < count {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                }
            }
            _ => {}
        }
    }
}

pub struct SpeechBubblePlugin;

impl Plugin for SpeechBubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpeechEvent>();
        app.add_systems(
            Update,
            (
                (read_speech_events, update_speech_bubble_position).chain(),
                countup,
                next_page,
                next_page_visibility,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
