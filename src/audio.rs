use crate::asset_credit::asset_to_credit;
use crate::config::GameConfig;
use crate::ui::pause_menu::BGMCredit;
use bevy::audio::PlaybackMode;
use bevy::audio::Volume;
use bevy::prelude::*;

const SPATIAL_AUDIO_MAX_DISTANCE: f32 = 400.0;

pub fn play_se(
    commands: &mut Commands,
    config: &GameConfig,
    source: &Handle<AudioSource>,
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

#[derive(Component)]
pub struct BGM {
    volume: f32,
}

fn change_bgm(
    mut commands: Commands,
    mut bgm_query: Query<(Entity, &AudioPlayer, &AudioSink, &mut BGM)>,
    next_bgm: ResMut<NextBGM>,
    config: Res<GameConfig>,
    mut bgm_credit_query: Query<&mut Text, With<BGMCredit>>,
) {
    if let Ok((entity, player, sink, mut bgm)) = bgm_query.get_single_mut() {
        if let Some(ref next) = next_bgm.0 {
            if player.0 != *next {
                bgm.volume = (bgm.volume - 0.01).max(0.0);
                sink.set_volume(config.bgm_volume * bgm.volume);

                if bgm.volume == 0.0 {
                    // AudioPlayer を上書きするだけでは音声を変更できないことに注意
                    // いったん despawn する必要がある
                    info!("BGM stopped");
                    commands.entity(entity).despawn_recursive();

                    spawn_bgm(&mut commands, &mut bgm_credit_query, next, &config);
                }
            }
        } else {
            bgm.volume = (bgm.volume - 0.01).max(0.0);
            sink.set_volume(config.bgm_volume * bgm.volume);
        }
    } else if let Some(ref next) = next_bgm.0 {
        info!("BGM started: {:?}", next);
        spawn_bgm(&mut commands, &mut bgm_credit_query, next, &config);
    }
}

fn spawn_bgm(
    commands: &mut Commands,
    bgm_credit_query: &mut Query<&mut Text, With<BGMCredit>>,
    next: &Handle<AudioSource>,
    config: &GameConfig,
) {
    let credit = asset_to_credit(next);

    if let Ok(mut bgm_credit) = bgm_credit_query.get_single_mut() {
        bgm_credit.0 = format!(
            "♪ {}『{}』{}",
            credit.authoer, credit.title, credit.appendix
        );
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
        app.add_systems(Update, (update_bgm_volue, change_bgm));
        app.init_resource::<NextBGM>();
    }
}
