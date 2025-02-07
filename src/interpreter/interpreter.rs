use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorState;
use crate::audio::NextBGM;
use crate::camera::GameCamera;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::config::GameConfig;
use crate::constant::ARENA;
use crate::constant::HOME_LEVEL;
use crate::controller::player::Player;
use crate::entity::light::spawn_flash_light;
use crate::hud::overlay::OverlayEvent;
use crate::interpreter::cmd::Cmd;
use crate::interpreter::cmd::Value;
use crate::inventory::InventoryItem;
use crate::language::Languages;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::script::javascript_loader::JavaScriptContext;
use crate::se::SEEvent;
use crate::se::KAWAII;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::new_spell::spawn_new_spell;
use crate::ui::speech_bubble::update_speech_bubble_position;
use crate::ui::speech_bubble::SpeechBubble;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use std::collections::HashMap;

const DELAY: usize = 4;

#[derive(Event)]
pub enum InterpreterEvent {
    /// シナリオを再生します
    /// このとき、現在再生中のコマンド列は失われます
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
    asset_server: Res<AssetServer>,
    registry: Registry,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    config: Res<GameConfig>,
    mut next_bgm: ResMut<NextBGM>,
    mut player_query: Query<(&mut Actor, &Player)>,
    mut camera: Query<&mut GameCamera>,
    mut interpreter: ResMut<Interpreter>,
    mut writer: EventWriter<OverlayEvent>,
    mut se_writer: EventWriter<SEEvent>,
    mut level: ResMut<GameWorld>,
    mut time: ResMut<NextState<TimeState>>,
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
            speech.dict = dict.clone();

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
                    se_writer.send(SEEvent::new(KAWAII));
                }

                interpreter.speech_count += step;
            }
        }
        Cmd::BGM(path) => {
            let handle = path.clone().map(|b| asset_server.load(b)).clone();
            if path.is_some() && handle.is_none() {
                warn!("BGM not found: {:?}", path);
            }
            next_bgm.0 = handle;
            interpreter.index += 1;
        }
        Cmd::SE { path } => {
            se_writer.send(SEEvent::new(path));
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
            if !entities.contains_key(&name) {
                warn!("Entity not found: {:?}", name);
            }
            for (entity_name, entity) in entities.iter() {
                if *entity_name == name {
                    commands.entity(*entity).despawn_recursive();
                }
            }
            interpreter.index += 1;
        }

        Cmd::Sprite {
            name,
            position,
            aseprite,
        } => {
            let p = position.to_vec2(&interpreter.environment);
            commands.spawn((
                Name::new(name),
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: asset_server.load(aseprite),
                    animation: "idle".into(),
                },
                StateScoped(GameState::InGame),
                Transform::from_translation(p.extend(0.0)),
                EntityDepth::new(),
            ));
            interpreter.index += 1;
        }

        Cmd::Wait { count } => {
            interpreter.wait = count;
            interpreter.index += 1;
        }

        Cmd::Ending => {
            writer.send(OverlayEvent::Close(GameState::Ending));
            interpreter.index += 1;
        }
        Cmd::Home => {
            level.next_level = GameLevel::new(HOME_LEVEL);
            writer.send(OverlayEvent::Close(GameState::Warp));
            interpreter.index += 1;
        }
        Cmd::Arena => {
            level.next_level = GameLevel::new(ARENA);
            writer.send(OverlayEvent::Close(GameState::Warp));
            interpreter.index += 1;
        }
        Cmd::Warp {
            destination_level,
            destination_iid,
        } => {
            level.next_level = destination_level;
            level.destination_iid = Some(destination_iid);
            writer.send(OverlayEvent::Close(GameState::Warp));
            interpreter.index += 1;
        }
        Cmd::SetTile { x, y, w, h, tile } => {
            for i in x..x + w as i32 {
                for j in y..y + h as i32 {
                    level.set_tile(i, j, tile.clone());
                }
            }
            interpreter.index += 1;
        }
        Cmd::SpawnRaven { name, position } => {
            commands.spawn((
                Name::new(name),
                AseSpriteAnimation {
                    aseprite: registry.assets.raven.clone(),
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
                    spell: spell.clone(),
                    price: 0,
                });
                if !player.discovered_spells.contains(&spell) {
                    spawn_new_spell(&mut commands, &registry, &mut time, spell, &mut se_writer);
                }
            }
            interpreter.index += 1;
        }

        Cmd::OnNewSpell {
            spell,
            commands_then,
            commands_else,
        } => {
            if let Ok((_, player)) = player_query.get_single_mut() {
                interpreter_events.send(InterpreterEvent::Play {
                    commands: if player.discovered_spells.contains(&spell) {
                        commands_else
                    } else {
                        commands_then
                    },
                });
            }
            interpreter.index += 1;
        }
    }
}

fn speech_countup(
    speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    config: Res<GameConfig>,
    mut player_query: Query<(&mut Actor, &Player)>,
    mut interpreter: ResMut<Interpreter>,
    mut se_writer: EventWriter<SEEvent>,
) {
    let (visibility, speech) = speech_query.single();

    if *visibility == Visibility::Hidden {
        return;
    }

    if let Ok((mut actor, _)) = player_query.get_single_mut() {
        actor.state = ActorState::Idle;
        actor.fire_state = ActorFireState::Idle;
    }

    let text_end_position = interpreter.speech_count / DELAY;
    let page_string = speech.dict.get(config.language);

    if text_end_position < page_string.char_indices().count() {
        let step = match config.language {
            Languages::Ja => 1,
            Languages::ZhCn => 1,
            _ => 2,
        };

        if interpreter.speech_count % (DELAY * step) == 0 {
            se_writer.send(SEEvent::new(KAWAII));
        }

        interpreter.speech_count += step;
    }
}

#[derive(Event)]
pub struct CmdEvent(pub Cmd);

fn read_cmd_event(
    mut reader: EventReader<CmdEvent>,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut interpreter: ResMut<Interpreter>,
) {
    let (mut visibility, mut speech) = speech_query.single_mut();

    for CmdEvent(event) in reader.read() {
        match event.clone() {
            // Cmd::Set { name, value } => {
            //     // interpreter.environment.insert(name, value);
            //     // interpreter.index += 1;
            // }
            Cmd::Focus(speaker) => {
                speech.entity = Some(speaker);
            }
            Cmd::Speech(dict) => {
                *visibility = Visibility::Inherited;
                speech.dict = dict.clone();
                interpreter.speech_count = 0;
            }
            // Cmd::BGM(path) => {
            //     // let handle = path.clone().map(|b| asset_server.load(b)).clone();
            //     // if path.is_some() && handle.is_none() {
            //     //     warn!("BGM not found: {:?}", path);
            //     // }
            //     // next_bgm.0 = handle;
            //     // interpreter.index += 1;
            // }
            // Cmd::SE { path } => {
            //     // se_writer.send(SEEvent::new(path));
            //     // interpreter.index += 1;
            // }
            // Cmd::Close => {
            //     // *visibility = Visibility::Hidden;
            //     // interpreter.index += 1;

            //     // camera.target = None;
            //     // if let Ok((mut actor, _)) = player_query.get_single_mut() {
            //     //     actor.state = ActorState::Idle;
            //     //     actor.wait = 30;
            //     // }
            // }
            // Cmd::Shake { value } => {
            //     // camera.vibration = value;
            //     // interpreter.index += 1;
            // }
            // Cmd::ShakeStart { value } => {
            //     // interpreter.shaking = value;
            //     // interpreter.index += 1;
            // }
            // Cmd::Flash {
            //     position,
            //     intensity,
            //     radius,
            //     duration,
            //     reverse,
            // } => {
            //     // spawn_flash_light(
            //     //     &mut commands,
            //     //     position.to_vec2(&interpreter.environment),
            //     //     intensity,
            //     //     radius,
            //     //     duration,
            //     //     reverse,
            //     // );

            //     // interpreter.index += 1;
            // }
            // Cmd::Despawn { name } => {
            //     // if !entities.contains_key(&name) {
            //     //     warn!("Entity not found: {:?}", name);
            //     // }
            //     // for (entity_name, entity) in entities.iter() {
            //     //     if *entity_name == name {
            //     //         commands.entity(*entity).despawn_recursive();
            //     //     }
            //     // }
            //     // interpreter.index += 1;
            // }

            // Cmd::Sprite {
            //     name,
            //     position,
            //     aseprite,
            // } => {
            //     // let p = position.to_vec2(&interpreter.environment);
            //     // commands.spawn((
            //     //     Name::new(name),
            //     //     CounterAnimated,
            //     //     AseSpriteAnimation {
            //     //         aseprite: asset_server.load(aseprite),
            //     //         animation: "idle".into(),
            //     //     },
            //     //     StateScoped(GameState::InGame),
            //     //     Transform::from_translation(p.extend(0.0)),
            //     //     EntityDepth::new(),
            //     // ));
            //     // interpreter.index += 1;
            // }

            // Cmd::Wait { count } => {
            //     // interpreter.wait = count;
            //     // interpreter.index += 1;
            // }

            // Cmd::Ending => {
            //     // writer.send(OverlayEvent::Close(GameState::Ending));
            //     // interpreter.index += 1;
            // }
            // Cmd::Home => {
            //     // level.next_level = GameLevel::new(HOME_LEVEL);
            //     // writer.send(OverlayEvent::Close(GameState::Warp));
            //     // interpreter.index += 1;
            // }
            // Cmd::Arena => {
            //     // level.next_level = GameLevel::new(ARENA);
            //     // writer.send(OverlayEvent::Close(GameState::Warp));
            //     // interpreter.index += 1;
            // }
            // Cmd::Warp {
            //     destination_level,
            //     destination_iid,
            // } => {
            //     // level.next_level = destination_level;
            //     // level.destination_iid = Some(destination_iid);
            //     // writer.send(OverlayEvent::Close(GameState::Warp));
            //     // interpreter.index += 1;
            // }
            // Cmd::SetTile { x, y, w, h, tile } => {
            //     // for i in x..x + w as i32 {
            //     //     for j in y..y + h as i32 {
            //     //         level.set_tile(i, j, tile.clone());
            //     //     }
            //     // }
            //     // interpreter.index += 1;
            // }
            // Cmd::SpawnRaven { name, position } => {
            //     // commands.spawn((
            //     //     Name::new(name),
            //     //     AseSpriteAnimation {
            //     //         aseprite: registry.assets.raven.clone(),
            //     //         animation: "idle".into(),
            //     //     },
            //     //     EntityDepth::new(),
            //     //     Transform::from_translation(position.extend(0.0)),
            //     // ));
            //     // interpreter.index += 1;
            // }

            // Cmd::SetCameraTarget { name } => {
            //     // match name {
            //     //     Some(name) => {
            //     //         if let Some(entity) = entities.get(&name) {
            //     //             camera.target = Some(*entity);
            //     //         }
            //     //     }
            //     //     None => {
            //     //         camera.target = None;
            //     //     }
            //     // };
            //     // interpreter.index += 1;
            // }

            // Cmd::GetSpell { spell } => {
            //     // if let Ok((mut actor, player)) = player_query.get_single_mut() {
            //     //     actor.inventory.insert(InventoryItem {
            //     //         spell: spell.clone(),
            //     //         price: 0,
            //     //     });
            //     //     if !player.discovered_spells.contains(&spell) {
            //     //         spawn_new_spell(&mut commands, &registry, &mut time, spell, &mut se_writer);
            //     //     }
            //     // }
            //     // interpreter.index += 1;
            // }

            // Cmd::OnNewSpell {
            //     spell,
            //     commands_then,
            //     commands_else,
            // } => {
            //     // if let Ok((_, player)) = player_query.get_single_mut() {
            //     //     interpreter_events.send(InterpreterEvent::Play {
            //     //         commands: if player.discovered_spells.contains(&spell) {
            //     //             commands_else
            //     //         } else {
            //     //             commands_then
            //     //         },
            //     //     });
            //     // }
            //     // interpreter.index += 1;
            // }
            _ => {}
        }
    }
}

fn next_page(
    mouse: Res<ButtonInput<MouseButton>>,
    mut bubble_query: Query<(&Visibility, &SpeechBubble)>,
    config: Res<GameConfig>,
    mut theater: ResMut<Interpreter>,
    state: Res<State<GameMenuState>>,
    mut script: NonSendMut<JavaScriptContext>,
) {
    let (bubble_visivility, bubble) = bubble_query.single_mut();
    if *bubble_visivility == Visibility::Inherited && *state != GameMenuState::PauseMenuOpen {
        if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            let page_string = bubble.dict.get(config.language);
            let chars = page_string.char_indices();
            let count = chars.count();
            let pos = theater.speech_count / DELAY;
            if pos < count {
                theater.speech_count = count * DELAY;
            } else {
                script.resume();
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
        app.add_event::<CmdEvent>();
        app.add_event::<InterpreterEvent>();
        app.init_resource::<Interpreter>();
        app.add_systems(
            Update,
            (
                shake_camera,
                read_interpreter_events.before(update_speech_bubble_position),
                interpret,
                next_page,
                read_cmd_event,
                speech_countup,
            )
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
        app.add_systems(OnExit(GameState::InGame), clear_senario);
    }
}
