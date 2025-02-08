use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorState;
use crate::camera::GameCamera;
use crate::config::GameConfig;
use crate::controller::player::Player;
use crate::hud::overlay::OverlayEvent;
use crate::language::Dict;
use crate::language::Languages;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::script::cmd::Cmd;
use crate::script::javascript_loader::JavaScriptContext;
use crate::se::SEEvent;
use crate::se::KAWAII;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::speech_bubble::SpeechBubble;
use bevy::prelude::*;

const DELAY: usize = 4;

#[derive(Resource, Default)]
pub struct Interpreter {
    pub speech_count: usize,
    pub commands: Vec<Cmd>,
    pub index: usize,
    pub wait: u32,
    pub shaking: Option<f32>,
}

fn speech_countup(
    speech_query: Query<(&Visibility, &SpeechBubble)>,
    config: Res<GameConfig>,
    mut player_query: Query<(&mut Actor, &Player)>,
    mut interpreter: ResMut<Interpreter>,
    mut se_writer: EventWriter<SEEvent>,
) {
    let (speech_visibility, speech) = speech_query.single();

    if *speech_visibility == Visibility::Hidden {
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
    registry: Registry,
    mut reader: EventReader<CmdEvent>,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut interpreter: ResMut<Interpreter>,
    mut level: ResMut<GameWorld>,
    mut overlay_writer: EventWriter<OverlayEvent>,
    mut camera: Query<&mut GameCamera>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut script: NonSendMut<JavaScriptContext>,
) {
    let (mut speech_visibility, mut speech) = speech_query.single_mut();

    for CmdEvent(cmd) in reader.read() {
        info!("CmdEvent: {:?}", cmd);

        match cmd.clone() {
            Cmd::Speech(dict) => {
                *speech_visibility = Visibility::Inherited;
                speech.dict = dict.clone();
                interpreter.speech_count = 0;
            }
            Cmd::Close => {
                *speech_visibility = Visibility::Hidden;
                speech.dict = Dict::empty();
                if let Ok(mut camera) = camera.get_single_mut() {
                    camera.target = None;
                }
                if let Ok(mut actor) = player_query.get_single_mut() {
                    actor.state = ActorState::Idle;
                    actor.wait = 30;
                }
                script.resume();
            }
            Cmd::GetSpell { spell } => {
                if let Ok(mut actor) = player_query.get_single_mut() {
                    actor.inventory.insert(spell.clone());
                }
            }
            Cmd::Warp { destination_iid } => {
                let (destination_level, _entity) = registry
                    .get_level_by_entity_iid(destination_iid.as_str())
                    .unwrap();
                level.next_level = GameLevel::new(destination_level.identifier);
                level.destination_iid = Some(destination_iid);
                overlay_writer.send(OverlayEvent::Close(GameState::Warp));
                script.resume();
            } //

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
        app.init_resource::<Interpreter>();
        app.add_systems(
            Update,
            (shake_camera, next_page, read_cmd_event, speech_countup)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
        app.add_systems(OnExit(GameState::InGame), clear_senario);
    }
}
