use super::context::JavaScriptContext;
use crate::actor::Actor;
use crate::actor::ActorState;
use crate::audio::NextBGM;
use crate::camera::GameCamera;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::controller::player::Player;
use crate::entity::light::spawn_flash_light;
use crate::hud::overlay::OverlayEvent;
use crate::language::Dict;
use crate::level::tile::Tile;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::spell::Spell;
use crate::states::GameState;
use crate::ui::speech_bubble::SpeechBubble;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;

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
    BGM {
        path: Option<String>,
    },

    SE {
        path: String,
    },
    /// 次のアクションまで指定したフレーム数待機します
    Wait {
        count: u32,
    },
    /// 画面を揺らします
    Shake {
        value: f32,
        attenuation: f32,
    },
    Flash {
        position: Vec2,
        intensity: f32,
        radius: f32,
        duration: u32,
        reverse: bool,
    },

    SetTile {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        tile: Tile,
    },
    Sprite {
        name: String,
        position: Vec2,
        aseprite: String,
    },

    Despawn {
        name: String,
    },
    // SpawnRaven {
    //     name: String,
    //     position: Vec2,
    // },

    // // todo ravenに合うような仮実装
    // SetCameraTarget {
    //     name: Option<String>,
    // },
}

#[derive(Event)]
pub struct CmdEvent(pub Cmd);

pub fn read_cmd_event(
    mut commands: Commands,
    registry: Registry,
    asset_server: Res<AssetServer>,
    mut world: ResMut<GameWorld>,
    mut script: NonSendMut<JavaScriptContext>,

    mut reader: EventReader<CmdEvent>,
    mut overlay_writer: EventWriter<OverlayEvent>,
    mut se_writer: EventWriter<SEEvent>,
    mut next_bgm: ResMut<NextBGM>,

    mut speech_query: Query<(&mut Visibility, &mut SpeechBubble)>,
    mut camera: Query<&mut GameCamera>,
    mut player_query: Query<&mut Actor, With<Player>>,
    entities_query: Query<(Entity, &Name)>,
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
                script.resume();
            }
            Cmd::Warp { destination_iid } => {
                let (destination_level, _entity) = registry
                    .get_level_by_entity_iid(destination_iid.as_str())
                    .unwrap();
                world.next_level = GameLevel::new(destination_level.identifier);
                world.destination_iid = Some(destination_iid);
                overlay_writer.send(OverlayEvent::Close(GameState::Warp));
                script.resume();
            } //

            Cmd::BGM { path } => {
                let handle = path.clone().map(|b| asset_server.load(b)).clone();
                if path.is_some() && handle.is_none() {
                    warn!("BGM not found: {:?}", path);
                }
                next_bgm.0 = handle;
                script.resume();
            }
            Cmd::SE { path } => {
                se_writer.send(SEEvent::new(path));
                script.resume();
            }

            Cmd::Wait { count } => {
                script.wait = count;
                *speech_visibility = Visibility::Hidden;
            }
            Cmd::Shake { value, attenuation } => {
                if let Ok(mut camera) = camera.get_single_mut() {
                    camera.vibration = value;
                    camera.attenuation = attenuation;
                }
                script.resume();
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
                    position,
                    intensity,
                    radius,
                    duration,
                    reverse,
                );
                script.resume();
            }
            Cmd::SetTile { x, y, w, h, tile } => {
                for i in x..x + w as i32 {
                    for j in y..y + h as i32 {
                        world.set_tile(i, j, tile.clone());
                    }
                }
                script.resume();
            }
            Cmd::Sprite {
                name,
                position,
                aseprite,
            } => {
                commands.spawn((
                    Name::new(name),
                    CounterAnimated,
                    AseSpriteAnimation {
                        aseprite: asset_server.load(aseprite),
                        animation: "idle".into(),
                    },
                    StateScoped(GameState::InGame),
                    Transform::from_translation(position.extend(0.0)),
                    EntityDepth::new(),
                ));
                script.resume();
            }
            Cmd::Despawn { name } => {
                // if !entities_query.contains_key(&name) {
                //     warn!("Entity not found: {:?}", name);
                // }
                for (entity, entity_name) in entities_query.iter() {
                    if entity_name.as_str() == name.as_str() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                script.resume();
            } // Cmd::SpawnRaven { name, position } => {
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
