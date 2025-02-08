use super::context::JavaScriptContext;
use crate::actor::{Actor, ActorState};
use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::hud::overlay::OverlayEvent;
use crate::language::Dict;
use crate::level::world::{GameLevel, GameWorld};
use crate::registry::Registry;
use crate::spell::Spell;
use crate::states::GameState;
use crate::ui::speech_bubble::SpeechBubble;
use bevy::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Cmd {
    /// フキダシにテキストを表示します
    Speech(Dict<String>),

    /// フキダシを非表示にします
    Close,

    GetSpell {
        spell: Spell,
    },

    Warp {
        destination_iid: String,
    },
    // /// BGMを変更します
    // BGM(Option<String>),

    // SE {
    //     path: String,
    // },

    // /// 次のアクションまで指定したフレーム数待機します
    // #[allow(dead_code)]
    // Wait {
    //     count: u32,
    // },

    // /// 画面を揺らします
    // Shake {
    //     value: f32,
    // },

    // /// 画面を揺らすエフェクトを開始します
    // ShakeStart {
    //     value: Option<f32>,
    // },

    // Flash {
    //     position: Expr,
    //     intensity: f32,
    //     radius: f32,
    //     duration: u32,
    //     reverse: bool,
    // },

    // /// エンディングを再生します
    // #[allow(dead_code)]
    // Ending,
    // Home,
    // Arena,
    // SetTile {
    //     x: i32,
    //     y: i32,
    //     w: u32,
    //     h: u32,
    //     tile: Tile,
    // },

    // SpawnRaven {
    //     name: String,
    //     position: Vec2,
    // },

    // Despawn {
    //     name: String,
    // },

    // Sprite {
    //     name: String,
    //     position: Expr,
    //     aseprite: String,
    // },

    // // todo ravenに合うような仮実装
    // SetCameraTarget {
    //     name: Option<String>,
    // },
}

#[derive(Event)]
pub struct CmdEvent(pub Cmd);

pub fn read_cmd_event(
    registry: Registry,
    mut reader: EventReader<CmdEvent>,
    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut level: ResMut<GameWorld>,
    mut overlay_writer: EventWriter<OverlayEvent>,
    mut camera: Query<&mut GameCamera>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut script: NonSendMut<JavaScriptContext>,
) {
    let (mut speech_visibility, mut speech) = speech_query.single_mut();

    for CmdEvent(cmd) in reader.read() {
        // info!("CmdEvent: {:?}", cmd);

        match cmd.clone() {
            Cmd::Speech(dict) => {
                *speech_visibility = Visibility::Inherited;
                speech.dict = dict.clone();
                speech.speech_count = 0;
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
