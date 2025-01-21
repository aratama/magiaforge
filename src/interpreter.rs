use std::collections::HashMap;

use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::bgm::BGMType;
use crate::camera::GameCamera;
use crate::component::entity_depth::EntityDepth;
use crate::config::GameConfig;
use crate::constant::GameConstants;
use crate::constant::SenarioType;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorState;
use crate::entity::light::spawn_flash_light;
use crate::entity::rabbit::spawn_rabbit;
use crate::hud::overlay::OverlayEvent;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::Dict;
use crate::language::Languages;
use crate::level::tile::Tile;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::se::SEEvent;
use crate::se::SE;
use crate::spell::SpellType;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::new_spell::spawn_new_spell;
use crate::ui::speech_bubble::update_speech_bubble_position;
use crate::ui::speech_bubble::SpeechBubble;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;

const DELAY: usize = 4;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Cmd {
    Set {
        name: String,
        value: Value,
    },

    /// フキダシを表示するキャラクターを指定します
    Focus(Entity),

    /// フキダシにテキストを表示します
    Speech(Dict<String>),

    /// BGMを変更します
    BGM {
        bgm: Option<BGMType>,
    },

    SE {
        se: SE,
    },

    /// フキダシを非表示にします
    Close,

    /// プレイヤーがインベントリにアイテムを入手します
    #[allow(dead_code)]
    GetItem(InventoryItemType),

    /// 次のアクションまで指定したフレーム数待機します
    #[allow(dead_code)]
    Wait {
        count: u32,
    },

    /// 画面を揺らします
    Shake {
        value: f32,
    },

    /// 画面を揺らすエフェクトを開始します
    ShakeStart {
        value: Option<f32>,
    },

    Flash {
        position: Expr,
        intensity: f32,
        radius: f32,
        duration: u32,
        reverse: bool,
    },

    #[allow(dead_code)]
    SpawnRabbit {
        position: Vec2,
    },

    /// エンディングを再生します
    #[allow(dead_code)]
    Ending,

    Home,

    Arena,

    Warp {
        level: i32,
    },

    SetTile {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        tile: Tile,
    },

    SpawnRaven {
        name: String,
        position: Vec2,
    },

    Despawn {
        name: String,
    },

    // todo ravenに合うような仮実装
    SetCameraTarget {
        name: Option<String>,
    },

    GetSpell {
        spell: SpellType,
    },

    OnNewSpell {
        spell: SpellType,
        commands: Vec<Cmd>,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Expr {
    Vec2 { x: f32, y: f32 },
    String { value: String },
    Var { value: String },
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Value {
    Vec2 { x: f32, y: f32 },
    String { value: String },
}

impl Expr {
    pub fn to_vec2(&self, environment: &HashMap<String, Value>) -> Vec2 {
        match self {
            Expr::Vec2 { x, y } => Vec2::new(*x, *y),
            Expr::Var { value } => match environment.get(value) {
                Some(Value::Vec2 { x, y }) => Vec2::new(*x, *y),
                _ => panic!("Value is not Vec2: {:?}", self),
            },
            _ => panic!("Value is not Vec2: {:?}", self),
        }
    }
}

#[derive(Event)]
pub enum InterpreterEvent {
    /// シナリオを再生します
    Play { commands: Vec<Cmd> },

    /// 現在実行中のシナリオをすべて中止します
    Quit,
}

#[derive(Resource, Default)]
pub struct Interpreter {
    pub speech_count: usize,
    pub commands: Vec<Cmd>,
    pub environment: HashMap<String, Value>,
    pub index: usize,
    pub wait: u32,
    pub shaking: Option<f32>,
}

impl Interpreter {
    pub fn current_act(&self) -> Option<&Cmd> {
        self.commands.get(self.index)
    }
}

fn read_interpreter_events(
    mut events: EventReader<InterpreterEvent>,
    mut speech_query: Query<(&mut SpeechBubble, &mut Visibility)>,
    mut theater: ResMut<Interpreter>,
) {
    for event in events.read() {
        let (mut speech, mut visibility) = speech_query.single_mut();
        match event {
            InterpreterEvent::Play { commands } => {
                theater.speech_count = 0;
                theater.commands = commands.clone();
                theater.environment = HashMap::new();
                theater.index = 0;
            }
            InterpreterEvent::Quit => {
                speech.entity = None;
                theater.commands.clear();
                theater.index = 0;
                theater.speech_count = 0;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn interpret(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    config: Res<GameConfig>,
    mut next_bgm: ResMut<NextBGM>,
    mut player_query: Query<(&mut Actor, &Player)>,
    mut camera: Query<&mut GameCamera>,
    mut interpreter: ResMut<Interpreter>,
    mut writer: EventWriter<OverlayEvent>,
    mut se_writer: EventWriter<SEEvent>,
    mut level: ResMut<LevelSetup>,
    mut time: ResMut<NextState<TimeState>>,
    ron: Res<Assets<GameConstants>>,
    named_entity_query: Query<(&Name, Entity)>,
    mut interpreter_events: EventWriter<InterpreterEvent>,
) {
    let (mut visibility, mut speech) = speech_query.single_mut();

    let mut camera = camera.single_mut();

    if interpreter.commands.len() <= interpreter.index {
        return;
    }

    if 0 < interpreter.wait {
        interpreter.wait -= 1;
        return;
    }

    let constants = ron.get(assets.spells.id()).unwrap();

    let entities: HashMap<String, Entity> = named_entity_query
        .iter()
        .map(|(n, e)| (n.as_str().to_string(), e.clone()))
        .collect();

    match interpreter.commands[interpreter.index].clone() {
        Cmd::Set { name, value } => {
            interpreter.environment.insert(name, value);
            interpreter.index += 1;
        }

        Cmd::Focus(speaker) => {
            speech.entity = Some(speaker);
            interpreter.index += 1;
        }
        Cmd::Speech(dict) => {
            *visibility = Visibility::Inherited;

            if let Ok((mut actor, _)) = player_query.get_single_mut() {
                actor.state = ActorState::Idle;
                actor.fire_state = ActorFireState::Idle;
            }

            let text_end_position = interpreter.speech_count / DELAY;
            let page_string = dict.get(config.language);

            if text_end_position < page_string.char_indices().count() {
                let step = match config.language {
                    Languages::Ja => 1,
                    Languages::ZhCn => 1,
                    _ => 2,
                };

                if interpreter.speech_count % (DELAY * step) == 0 {
                    se_writer.send(SEEvent::new(SE::Kawaii));
                }

                interpreter.speech_count += step;
            }
        }
        Cmd::BGM { bgm } => {
            next_bgm.0 = bgm.map(|b| b.to_source(&assets)).clone();
            interpreter.index += 1;
        }
        Cmd::SE { se } => {
            se_writer.send(SEEvent::new(se));
            interpreter.index += 1;
        }
        Cmd::GetItem(item) => {
            if let Ok((mut actor, _)) = player_query.get_single_mut() {
                actor.inventory.insert(InventoryItem {
                    item_type: item,
                    price: 0,
                });
            }
            interpreter.index += 1;
        }
        Cmd::Close => {
            *visibility = Visibility::Hidden;
            interpreter.index += 1;

            camera.target = None;
            if let Ok((mut actor, _)) = player_query.get_single_mut() {
                actor.state = ActorState::Idle;
                actor.wait = 30;
            }
        }
        Cmd::Shake { value } => {
            camera.vibration = value;
            interpreter.index += 1;
        }
        Cmd::ShakeStart { value } => {
            interpreter.shaking = value;
            interpreter.index += 1;
        }
        Cmd::Flash {
            position,
            intensity,
            radius,
            duration,
            reverse,
        } => {
            spawn_flash_light(
                &mut commands,
                position.to_vec2(&interpreter.environment),
                intensity,
                radius,
                duration,
                reverse,
            );

            interpreter.index += 1;
        }
        Cmd::Despawn { name } => {
            if let Some(entity) = entities.get(&name) {
                commands.entity(*entity).despawn_recursive();
                // info!("despawn {} {}", file!(), line!());
            }
            interpreter.index += 1;
        }
        Cmd::Wait { count } => {
            interpreter.wait = count;
            interpreter.index += 1;
        }
        Cmd::SpawnRabbit { position } => {
            // テスト用、使っていない
            spawn_rabbit(
                &mut commands,
                &assets,
                &assets.rabbit_blue,
                position,
                MessageRabbit {
                    senario: SenarioType::HelloRabbit,
                },
                MessageRabbitInnerSensor,
                MessageRabbitOuterSensor,
            );

            interpreter.index += 1;
        }
        Cmd::Ending => {
            writer.send(OverlayEvent::Close(GameState::Ending));
            interpreter.index += 1;
        }
        Cmd::Home => {
            level.next_level = GameLevel::Level(0);
            writer.send(OverlayEvent::Close(GameState::Warp));
            interpreter.index += 1;
        }
        Cmd::Arena => {
            level.next_level = GameLevel::MultiPlayArena;
            writer.send(OverlayEvent::Close(GameState::Warp));
            interpreter.index += 1;
        }
        Cmd::Warp { level: l } => {
            level.next_level = GameLevel::Level(l);
            writer.send(OverlayEvent::Close(GameState::Warp));
            interpreter.index += 1;
        }
        Cmd::SetTile { x, y, w, h, tile } => {
            if let Some(ref mut chunk) = level.chunk {
                for i in x..x + w as i32 {
                    for j in y..y + h as i32 {
                        chunk.set_tile(i, j, tile);
                    }
                }
            }
            interpreter.index += 1;
        }
        Cmd::SpawnRaven { name, position } => {
            commands.spawn((
                Name::new(name),
                AseSpriteAnimation {
                    aseprite: assets.raven.clone(),
                    animation: "idle".into(),
                },
                EntityDepth::new(),
                Transform::from_translation(position.extend(0.0)),
            ));
            interpreter.index += 1;
        }

        Cmd::SetCameraTarget { name } => {
            match name {
                Some(name) => {
                    if let Some(entity) = entities.get(&name) {
                        camera.target = Some(*entity);
                    }
                }
                None => {
                    camera.target = None;
                }
            };
            interpreter.index += 1;
        }

        Cmd::GetSpell { spell } => {
            if let Ok((mut actor, player)) = player_query.get_single_mut() {
                actor.inventory.insert(InventoryItem {
                    item_type: InventoryItemType::Spell(spell),
                    price: 0,
                });
                if !player.discovered_spells.contains(&spell) {
                    spawn_new_spell(
                        &mut commands,
                        &assets,
                        &constants,
                        &mut time,
                        spell,
                        &mut se_writer,
                    );
                }
            }
            interpreter.index += 1;
        }

        Cmd::OnNewSpell { spell, commands } => {
            if let Ok((_, player)) = player_query.get_single_mut() {
                if !player.discovered_spells.contains(&spell) {
                    interpreter_events.send(InterpreterEvent::Play { commands });
                }
            }
            interpreter.index += 1;
        }
    }
}

fn next_page(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bubble_query: Query<&Visibility, With<SpeechBubble>>,
    mut writer: EventWriter<InterpreterEvent>,
    config: Res<GameConfig>,
    mut theater: ResMut<Interpreter>,
    state: Res<State<GameMenuState>>,
) {
    let bubble_visivility = bubble_query.single_mut();
    if *bubble_visivility == Visibility::Inherited && *state != GameMenuState::PauseMenuOpen {
        if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            match theater.current_act() {
                Some(Cmd::Speech(dict)) => {
                    let page_string = dict.get(config.language);
                    let chars = page_string.char_indices();
                    let count = chars.count();
                    let pos = theater.speech_count / DELAY;
                    if pos < count {
                        theater.speech_count = count * DELAY;
                    } else if theater.index < theater.commands.len() - 1 {
                        theater.index += 1;
                        theater.speech_count = 0;
                    } else {
                        writer.send(InterpreterEvent::Quit);
                    }
                }
                _ => {}
            }
        }
    }
}

fn shake_camera(mut camera_query: Query<&mut GameCamera>, interpreter: ResMut<Interpreter>) {
    if let Some(shake) = interpreter.shaking {
        let mut camera = camera_query.single_mut();
        camera.vibration = shake;
    }
}

fn clear_senario(mut interpreter: ResMut<Interpreter>) {
    interpreter.commands.clear();
    interpreter.index = 0;
    interpreter.speech_count = 0;
    interpreter.wait = 0;
    interpreter.shaking = None;
}

pub struct InterpreterPlugin;

impl Plugin for InterpreterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InterpreterEvent>();
        app.init_resource::<Interpreter>();
        app.add_systems(
            Update,
            (
                shake_camera,
                read_interpreter_events.before(update_speech_bubble_position),
                interpret,
                next_page,
            )
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
        app.add_systems(OnExit(GameState::InGame), clear_senario);
    }
}
