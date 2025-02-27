use crate::config::GameConfig;
use crate::registry::path_to_string;
use crate::registry::Registry;
use crate::states::GameState;
use crate::ui::pause_menu::BGMCredit;
use bevy::audio::PlaybackMode;
use bevy::audio::Volume;
use bevy::prelude::*;

const SPATIAL_AUDIO_MAX_DISTANCE: f32 = 400.0;

pub fn play_se(
    commands: &mut Commands,
    config: &GameConfig,
    source: Handle<AudioSource>,
    position: &Option<Vec2>,
    camera_position: Vec2,
) {
    let volume = match position {
        None => 1.0,
        Some(p) => {
            1.0 - ((*p - camera_position).length() / SPATIAL_AUDIO_MAX_DISTANCE)
                .max(0.0)
                .min(1.0)
                .powf(2.0)
        }
    };

    // let panning = match position {
    //     None => 0.5,
    //     Some(p) => {
    //         0.5 + (((*p).x - camera_position.x) / SPATIAL_AUDIO_MAX_DISTANCE)
    //             .max(-0.5)
    //             .min(0.5)
    //     }
    // };

    commands.spawn((
        AudioPlayer::new(source.clone()),
        PlaybackSettings {
            volume: Volume::new(config.se_volume * volume),
            mode: bevy::audio::PlaybackMode::Despawn,
            // spatial: true,
            ..default()
        },
    ));
}

/// 次に再生するBGMを表すリソース
#[derive(Resource, Default)]
pub struct NextBGM(pub Option<Handle<AudioSource>>);

#[derive(Component, Debug)]
struct BGM {
    volume: f32,
}

fn change_bgm(
    mut commands: Commands,
    registry: Registry,
    // BGMをspawnした直後、一瞬 AudioSink がないフレームがあるので Option<&AudioSink> としています
    // そうでないとBGM がクエリにヒットせず、新しいBGMをspawnしてしまい、重複してspawnしてしまうことがある
    mut bgm_query: Query<(Entity, &AudioPlayer, Option<&AudioSink>, &mut BGM)>,
    next_bgm: ResMut<NextBGM>,
    config: Res<GameConfig>,
    mut bgm_credit_query: Query<&mut Text, With<BGMCredit>>,
) {
    if let Ok((entity, player, sink, mut bgm)) = bgm_query.get_single_mut() {
        if let Some(ref next) = next_bgm.0 {
            if player.0.path().map(path_to_string) != next.path().map(path_to_string) {
                bgm.volume = (bgm.volume - 0.01).max(0.0);
                if let Some(sink) = sink {
                    sink.set_volume(config.bgm_volume * bgm.volume);
                }

                if bgm.volume == 0.0 {
                    // AudioPlayer を上書きするだけでは音声を変更できないことに注意
                    // いったん despawn する必要がある
                    commands.entity(entity).despawn_recursive();

                    spawn_bgm(
                        &mut commands,
                        &registry,
                        &mut bgm_credit_query,
                        next,
                        &config,
                    );
                }
            }
        } else if let Some(sink) = sink {
            bgm.volume = (bgm.volume - 0.01).max(0.0);
            sink.set_volume(config.bgm_volume * bgm.volume);
        }
    } else if let Some(ref next) = next_bgm.0 {
        if bgm_query.is_empty() {
            info!("no bgm, spawn new bgm: {:?}", next);
            spawn_bgm(
                &mut commands,
                &registry,
                &mut bgm_credit_query,
                next,
                &config,
            );
        }
    }
}

fn spawn_bgm(
    commands: &mut Commands,
    registry: &Registry,
    bgm_credit_query: &mut Query<&mut Text, With<BGMCredit>>,
    next: &Handle<AudioSource>,
    config: &GameConfig,
) {
    let credit = registry.get_bgm(&next);

    if let Ok(mut bgm_credit) = bgm_credit_query.get_single_mut() {
        bgm_credit.0 = format!("♪ {}『{}』{}", credit.author, credit.title, credit.appendix);
    }

    commands.spawn((
        BGM { volume: 1.0 },
        AudioPlayer::new(next.clone()),
        PlaybackSettings {
            volume: Volume::new(config.bgm_volume),
            mode: PlaybackMode::Loop,
            ..default()
        },
    ));
}

fn update_bgm_volue(mut current_bgm: Query<(&AudioSink, &BGM)>, config: Res<GameConfig>) {
    if config.is_changed() {
        if let Ok((sink, bgm)) = current_bgm.get_single_mut() {
            sink.set_volume(config.bgm_volume * bgm.volume);
        }
    }
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_bgm_volue, change_bgm).run_if(
                in_state(GameState::MainMenu)
                    .or(in_state(GameState::InGame))
                    .or(in_state(GameState::Opening))
                    .or(in_state(GameState::Warp))
                    .or(in_state(GameState::NameInput))
                    .or(in_state(GameState::Ending)),
            ),
        );
        app.init_resource::<NextBGM>();
    }
}
