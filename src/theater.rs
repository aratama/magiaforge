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
use crate::physics::InGameTime;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::ui::speech_bubble::update_speech_bubble_position;
use crate::ui::speech_bubble::SpeechBubble;
use crate::ui::speech_bubble::SpeechBubbleText;
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;

const DELAY: usize = 4;

#[derive(Debug, Clone)]
pub enum Act {
    /// フキダシを表示するキャラクターを指定します
    Focus(Entity),

    /// フキダシにテキストを表示します
    Speech(Dict<String>),

    /// BGMを変更します
    BGM(Option<Handle<AudioSource>>),

    SE(SE),

    /// フキダシを非表示にします
    Close,

    /// プレイヤーがインベントリにアイテムを入手します
    #[allow(dead_code)]
    GetItem(InventoryItemType),

    /// 次のアクションまで指定したフレーム数待機します
    #[allow(dead_code)]
    Wait(u32),

    /// 画面を揺らします
    Shake(f32),

    /// 画面を揺らすエフェクトを開始します
    ShakeStart(Option<f32>),

    Flash {
        position: Vec2,
        intensity: f32,
        radius: f32,
        duration: u32,
        reverse: bool,
    },

    Despown(Entity),

    #[allow(dead_code)]
    SpawnRabbit {
        position: Vec2,
    },

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
    pub shaking: Option<f32>,
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
    config: Res<GameConfig>,
    mut next_bgm: ResMut<NextBGM>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut camera: Query<&mut GameCamera>,
    mut theater: ResMut<Theater>,
    mut writer: EventWriter<OverlayEvent>,
    mut se_writer: EventWriter<SEEvent>,
    in_game_time: Res<InGameTime>,
) {
    if !in_game_time.active {
        return;
    }

    let (mut visibility, mut speech) = speech_query.single_mut();

    let mut camera = camera.single_mut();

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
                    se_writer.send(SEEvent::new(SE::Kawaii));
                }

                theater.speech_count += 1;
            }
        }
        Act::BGM(bgm) => {
            next_bgm.0 = bgm.clone();
            theater.act_index += 1;
        }
        Act::SE(se) => {
            se_writer.send(SEEvent::new(se));
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

            camera.target = None;
            if let Ok(mut actor) = player_query.get_single_mut() {
                actor.state = ActorState::Idle;
                actor.wait = 30;
            }
        }
        Act::Shake(shake) => {
            camera.vibration = shake;
            theater.act_index += 1;
        }
        Act::ShakeStart(shake) => {
            theater.shaking = shake;
            theater.act_index += 1;
        }
        Act::Flash {
            position,
            intensity,
            radius,
            duration,
            reverse,
        } => {
            commands.spawn((
                StateScoped(GameState::InGame),
                FlashLight {
                    intensity,
                    duration,
                    count: 0,
                    reverse,
                },
                Transform::from_translation(position.extend(0.0)),
                PointLight2d {
                    intensity,
                    radius,
                    ..default()
                },
            ));

            theater.act_index += 1;
        }
        Act::Despown(entity) => {
            commands.entity(entity).despawn_recursive();
            theater.act_index += 1;
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

#[derive(Component)]
struct FlashLight {
    intensity: f32,
    duration: u32,
    count: u32,
    reverse: bool,
}

fn flash_ligh_fade_out(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FlashLight, &mut PointLight2d)>,
) {
    for (entity, mut flash, mut light) in query.iter_mut() {
        if flash.count < flash.duration {
            flash.count += 1;
            let t = flash.count as f32 / flash.duration as f32;
            light.intensity = flash.intensity * if flash.reverse { t } else { 1.0 - t };
        } else {
            commands.entity(entity).despawn_recursive();
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

fn shake_camera(mut camera_query: Query<&mut GameCamera>, theater: ResMut<Theater>) {
    if let Some(shake) = theater.shaking {
        let mut camera = camera_query.single_mut();
        camera.vibration = shake;
    }
}

fn clear_senario(mut theater: ResMut<Theater>) {
    theater.senario.clear();
    theater.act_index = 0;
    theater.speech_count = 0;
    theater.wait = 0;
    theater.shaking = None;
}

pub struct SenarioPlugin;

impl Plugin for SenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TheaterEvent>();
        app.init_resource::<Theater>();
        app.add_systems(
            Update,
            (
                shake_camera,
                read_speech_events.before(update_speech_bubble_position),
                countup,
                next_page,
                flash_ligh_fade_out,
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnExit(GameState::InGame), clear_senario);
    }
}
