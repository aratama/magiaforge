use crate::{
    asset::GameAssets,
    audio::{play_se, BGM},
    config::GameConfig,
    hud::overlay::OverlayNextState,
    states::GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::Audio;

#[derive(Event, Clone)]
pub enum GameCommand {
    SEDageki,
    SEShibafu,
    SEHiyoko,
    SEKuzureru,
    SEKettei,
    SESuburi,
    SEAsphalt,
    SEMenuOpen,
    SEWarp,
    SECancel,
    BGMNone,
    BGMBoubaku,
    BGMArechi,
    StateMainMenu,
    StateNameInput,
    StateConfig,
    StateInGame,
    StateWarp,
}

fn process_game_commands(
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
    mut reader: EventReader<GameCommand>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    mut next_bgm: ResMut<BGM>,
) {
    for event in reader.read() {
        match event {
            GameCommand::SEDageki => {
                play_se(&audio, &config, assets.dageki.clone());
            }
            GameCommand::SEShibafu => {
                play_se(&audio, &config, assets.shibafu.clone());
            }
            GameCommand::SEHiyoko => {
                play_se(&audio, &config, assets.hiyoko.clone());
            }
            GameCommand::SEKuzureru => {
                play_se(&audio, &config, assets.kuzureru.clone());
            }
            GameCommand::SEKettei => {
                play_se(&audio, &config, assets.kettei.clone());
            }
            GameCommand::SESuburi => {
                play_se(&audio, &config, assets.suburi.clone());
            }
            GameCommand::SEAsphalt => {
                play_se(&audio, &config, assets.asphalt.clone());
            }
            GameCommand::SEMenuOpen => {
                play_se(&audio, &config, assets.menu_open.clone());
            }
            GameCommand::SEWarp => {
                play_se(&audio, &config, assets.warp.clone());
            }
            GameCommand::SECancel => {
                play_se(&audio, &config, assets.cancel.clone());
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
            GameCommand::StateMainMenu => {
                *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
            }
            GameCommand::StateNameInput => {
                *overlay_next_state = OverlayNextState(Some(GameState::NameInput));
            }
            GameCommand::StateConfig => {
                *overlay_next_state = OverlayNextState(Some(GameState::Config));
            }
            GameCommand::StateInGame => {
                *overlay_next_state = OverlayNextState(Some(GameState::InGame));
            }
            GameCommand::StateWarp => {
                *overlay_next_state = OverlayNextState(Some(GameState::Warp));
            }
        }
    }
}

pub struct GameCommandPlugin;

impl Plugin for GameCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameCommand>().add_systems(
            FixedUpdate,
            process_game_commands.run_if(resource_exists::<GameAssets>),
        );
    }
}
