use crate::{
    asset::GameAssets,
    audio::{play_se, BGM},
    config::GameConfig,
    hud::overlay::OverlayEvent,
    states::GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Event, Clone, Copy, Debug)]
pub enum GameCommand {
    SEDageki(Option<Vec2>),
    SEShibafu(Option<Vec2>),
    SEHiyoko(Option<Vec2>),
    SEKuzureru(Option<Vec2>),
    SEKettei(Option<Vec2>),
    SESuburi(Option<Vec2>),
    SEAsphalt(Option<Vec2>),
    SEMenuOpen(Option<Vec2>),
    SEWarp(Option<Vec2>),
    SECancel(Option<Vec2>),
    SEKaifuku(Option<Vec2>),
    SECursor2(Option<Vec2>),
    BGMNone,
    BGMBoubaku,
    BGMArechi,
    BGMDokutsu,
    StateMainMenu,
    StateInGame,
    StateWarp,
}

#[derive(Resource, Default)]
struct CommandState {
    next: Option<GameState>,
}

fn process_game_commands(
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
    mut reader: EventReader<GameCommand>,
    mut next_bgm: ResMut<BGM>,
    camera_query: Query<&Transform, With<Camera2d>>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
    mut next: ResMut<CommandState>,
) {
    let camera_position = camera_query.single().translation.truncate();

    for event in reader.read() {
        // info!("commands: {:?}", event);

        match event {
            GameCommand::SEDageki(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.dageki.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEShibafu(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.shibafu.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEHiyoko(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.hiyoko.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEKuzureru(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.kuzureru.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEKettei(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.kettei.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SESuburi(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.suburi.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEAsphalt(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.asphalt.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEMenuOpen(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.menu_open.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEWarp(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.warp.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SECancel(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.cancel.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEKaifuku(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.kaifuku1.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SECursor2(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.cursor2.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::BGMNone => {
                *next_bgm = BGM(None);
            }
            GameCommand::BGMBoubaku => {
                *next_bgm = BGM(Some(assets.boubaku.clone()));
            }
            GameCommand::BGMArechi => {
                *next_bgm = BGM(Some(assets.arechi.clone()));
            }
            GameCommand::BGMDokutsu => {
                info!("play dokutsu");
                *next_bgm = BGM(Some(assets.dokutsu.clone()));
            }
            GameCommand::StateMainMenu => {
                overlay_event_writer.send(OverlayEvent::Close(GameState::MainMenu));
            }
            GameCommand::StateInGame => {
                overlay_event_writer.send(OverlayEvent::Close(GameState::InGame));
            }
            GameCommand::StateWarp => {
                next.next = Some(GameState::Warp);
                overlay_event_writer.send(OverlayEvent::Close(GameState::Warp));
            }
        }
    }
}

pub struct GameCommandPlugin;

impl Plugin for GameCommandPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandState>();
        app.add_event::<GameCommand>().add_systems(
            FixedUpdate,
            (process_game_commands)
                .run_if(resource_exists::<GameAssets>)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
