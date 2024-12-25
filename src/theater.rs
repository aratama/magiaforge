use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::camera::GameCamera;
use crate::config::GameConfig;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorState;
use crate::entity::rabbit::spawn_rabbit;
use crate::hud::overlay::OverlayEvent;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::Dict;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::ui::speech_bubble::update_speech_bubble_position;
use crate::ui::speech_bubble::SpeechBubble;
use crate::ui::speech_bubble::SpeechBubbleText;
use bevy::prelude::*;

const DELAY: usize = 4;

#[derive(Debug, Clone)]
pub enum Act {
    /// フキダシを表示するキャラクターを指定します
    Focus(Entity),

    /// フキダシにテキストを表示します
    Speech(Dict<String>),

    /// BGMを変更します
    BGM(Option<Handle<AudioSource>>),

    /// フキダシを非表示にします
    Close,

    /// プレイヤーがインベントリにアイテムを入手します
    #[allow(dead_code)]
    GetItem(InventoryItemType),

    /// 次のアクションまで指定したフレーム数待機します
    #[allow(dead_code)]
    Wait(u32),

    #[allow(dead_code)]
    SpawnRabbit { position: Vec2 },

    /// エンディングを再生します
    Ending,
}

#[derive(Event)]
pub enum TheaterEvent {
    /// シナリオを再生します
    Play { acts: Vec<Act> },

    /// 現在実行中のシナリオをすべて中止します
    Quit,
}

#[derive(Resource, Default)]
pub struct Theater {
    pub speech_count: usize,
    pub senario: Vec<Act>,
    pub act_index: usize,
    pub wait: u32,
}

impl Theater {
    pub fn current_act(&self) -> Option<&Act> {
        self.senario.get(self.act_index)
    }
}

fn read_speech_events(
    mut events: EventReader<TheaterEvent>,
    mut speech_query: Query<(&mut SpeechBubble, &mut Visibility)>,
    mut player_query: Query<&mut Actor>,
    mut theater: ResMut<Theater>,
) {
    for event in events.read() {
        let (mut speech, mut visibility) = speech_query.single_mut();
        match event {
            TheaterEvent::Play { acts } => {
                theater.speech_count = 0;
                theater.senario = acts.clone();
                theater.act_index = 0;
                if let Ok(mut player) = player_query.get_single_mut() {
                    player.state = ActorState::Idle;
                }
            }
            TheaterEvent::Quit => {
                speech.entity = None;
                theater.senario.clear();
                theater.act_index = 0;
                theater.speech_count = 0;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn countup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut speech_text_query: Query<&mut Text, With<SpeechBubbleText>>,
    mut se: EventWriter<SEEvent>,
    config: Res<GameConfig>,
    mut next_bgm: ResMut<NextBGM>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut camera: Query<&mut GameCamera>,
    mut theater: ResMut<Theater>,
    mut writer: EventWriter<OverlayEvent>,
) {
    let (mut visibility, mut speech) = speech_query.single_mut();

    if theater.senario.len() <= theater.act_index {
        // let mut camera = camera.single_mut();
        // camera.target = None;
        return;
    }

    let pos = theater.speech_count / DELAY;
    let mut speech_text = speech_text_query.single_mut();

    let page = theater.senario[theater.act_index].clone();
    match page {
        Act::Focus(speaker) => {
            speech.entity = Some(speaker);
            theater.act_index += 1;
        }
        Act::Speech(dict) => {
            *visibility = Visibility::Inherited;

            if let Ok(mut actor) = player_query.get_single_mut() {
                actor.state = ActorState::Idle;
                actor.fire_state = ActorFireState::Idle;
            }

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
                if theater.speech_count % DELAY == 0 {
                    se.send(SEEvent::new(SE::Kawaii));
                }

                theater.speech_count += 1;
            }
        }
        Act::BGM(bgm) => {
            next_bgm.0 = bgm.clone();
            theater.act_index += 1;
        }
        Act::GetItem(item) => {
            if let Ok(mut actor) = player_query.get_single_mut() {
                actor.inventory.insert(InventoryItem {
                    item_type: item,
                    price: 0,
                });
            }
            theater.act_index += 1;
        }
        Act::Close => {
            *visibility = Visibility::Hidden;
            theater.act_index += 1;
            let mut camera = camera.single_mut();
            camera.target = None;
            if let Ok(mut actor) = player_query.get_single_mut() {
                actor.state = ActorState::Idle;
                actor.wait = 30;
            }
        }
        Act::Wait(wait) => {
            if theater.wait <= 0 {
                theater.wait = wait;
            } else {
                theater.wait -= 1;
                if theater.wait == 0 {
                    theater.act_index += 1;
                }
            }
        }
        Act::SpawnRabbit { position } => {
            spawn_rabbit(
                &mut commands,
                &assets,
                &assets.rabbit_blue,
                position,
                MessageRabbit {
                    messages: vec![
                        // Act::BGM(Some(assets.saihate.clone())),
                        // Act::Speech(HELLO.to_string()),
                        // Act::Speech(HELLO_RABBITS.to_string()),
                    ],
                },
                MessageRabbitInnerSensor,
                MessageRabbitOuterSensor,
            );

            theater.act_index += 1;
        }
        Act::Ending => {
            writer.send(OverlayEvent::Close(GameState::Ending));
            theater.act_index += 1;
        }
    }
}

fn next_page(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bubble_query: Query<&Visibility, With<SpeechBubble>>,
    mut writer: EventWriter<TheaterEvent>,
    config: Res<GameConfig>,
    mut theater: ResMut<Theater>,
) {
    let bubble_visivility = bubble_query.single_mut();
    if *bubble_visivility == Visibility::Inherited {
        if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            match theater.current_act() {
                Some(Act::Speech(dict)) => {
                    let page_string = dict.get(config.language);
                    let chars = page_string.char_indices();
                    let count = chars.count();
                    let pos = theater.speech_count / DELAY;
                    if pos < count {
                        theater.speech_count = count * DELAY;
                    } else if theater.act_index < theater.senario.len() - 1 {
                        theater.act_index += 1;
                        theater.speech_count = 0;
                    } else {
                        writer.send(TheaterEvent::Quit);
                    }
                }
                _ => {}
            }
        }
    }
}

pub struct SenarioPlugin;

impl Plugin for SenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TheaterEvent>();
        app.init_resource::<Theater>();
        app.add_systems(
            Update,
            (
                read_speech_events.before(update_speech_bubble_position),
                countup,
                next_page,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
