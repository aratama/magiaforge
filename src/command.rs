use crate::{
    asset::GameAssets, audio::play_se, config::GameConfig, hud::overlay::OverlayEvent,
    states::GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Event, Clone, Copy, Debug)]
pub enum GameCommand {
    SEDamage(Option<Vec2>),
    SENoDamage(Option<Vec2>),
    SECry(Option<Vec2>),
    SEBreak(Option<Vec2>),
    SEClick(Option<Vec2>),
    SEFire(Option<Vec2>),
    SESteps(Option<Vec2>),
    SETurnOn(Option<Vec2>),
    SEWarp(Option<Vec2>),
    SEPickUp(Option<Vec2>),
    SEHeal(Option<Vec2>),
    SESwitch(Option<Vec2>),
    SEEmptyMana(Option<Vec2>),
    SEDrop(Option<Vec2>),
    SEGrowl(Option<Vec2>),
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
    camera_query: Query<&Transform, With<Camera2d>>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
    mut next: ResMut<CommandState>,
) {
    let camera_position = camera_query.single().translation.truncate();

    for event in reader.read() {
        // info!("commands: {:?}", event);

        match event {
            GameCommand::SEDamage(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.dageki.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SENoDamage(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.shibafu.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SECry(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.hiyoko.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEBreak(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.kuzureru.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEClick(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.kettei.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEFire(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.suburi.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SESteps(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.asphalt.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SETurnOn(position) => {
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
            GameCommand::SEPickUp(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.cancel.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEHeal(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.kaifuku1.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SESwitch(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.cursor2.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEEmptyMana(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.cursor8.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEDrop(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.drop.clone(),
                    position,
                    camera_position,
                );
            }
            GameCommand::SEGrowl(position) => {
                play_se(
                    &audio,
                    &config,
                    assets.inoshishi.clone(),
                    position,
                    camera_position,
                );
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
