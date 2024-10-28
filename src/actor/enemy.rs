use super::player::Player;
use crate::config::GameConfig;
use crate::constant::*;
use crate::entity::actor::Actor;
use crate::{asset::GameAssets, audio::play_se, set::GameSet, states::GameState};
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

/// 自動操作でプレイヤーに襲い掛かってくる敵を表します
#[derive(Component)]
pub struct Enemy;

const ENEMY_MOVE_FORCE: f32 = 50000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 5.0;

const ENEMY_ATTACK_POINT: i32 = 8;

fn update_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut query: Query<
        (Entity, &Actor, &mut ExternalForce, &mut Transform),
        (With<Enemy>, Without<Camera2d>),
    >,
    mut player_query: Query<
        (Entity, &mut Actor, &GlobalTransform, &mut ExternalImpulse),
        (With<Player>, Without<Enemy>),
    >,
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    if let Ok((player_entity, mut player, player_transform, mut player_impulse)) =
        player_query.get_single_mut()
    {
        for (entity, enemy, mut force, mut enemy_transform) in query.iter_mut() {
            // 1マス以上5マス以内にプレイヤーがいたら追いかける
            let diff = player_transform.translation() - enemy_transform.translation;
            if 0 < player.life && diff.length() < ENEMY_DETECTION_RANGE {
                let direction = diff.normalize_or_zero();
                force.force = direction.truncate() * ENEMY_MOVE_FORCE;
            } else {
                force.force = Vec2::ZERO;
            }

            // ライフが0以下になったら消滅
            if enemy.life <= 0 {
                commands.entity(entity).despawn_recursive();
                play_se(&audio, &config, assets.hiyoko.clone());
            }

            // z を設定
            enemy_transform.translation.z =
                ENTITY_LAYER_Z - enemy_transform.translation.y * Z_ORDER_SCALE;
        }

        for collision_event in collision_events.read() {
            match collision_event {
                CollisionEvent::Started(a, b, _) => {
                    if *a == player_entity {
                        if let Ok(en) = query.get(*b) {
                            process_attack_event(
                                &en.3,
                                &mut player,
                                &player_transform,
                                &mut player_impulse,
                                &assets,
                                &audio,
                                &config,
                            );
                        };
                    } else if *b == player_entity {
                        if let Ok(en) = query.get(*a) {
                            process_attack_event(
                                &en.3,
                                &mut player,
                                &player_transform,
                                &mut player_impulse,
                                &assets,
                                &audio,
                                &config,
                            );
                        };
                    }
                }
                _ => {}
            }
        }
    }
}

fn process_attack_event(
    enemy_transform: &Transform,
    player: &mut Actor,
    player_transform: &GlobalTransform,
    player_impulse: &mut ExternalImpulse,
    assets: &Res<GameAssets>,
    audio: &Res<Audio>,
    config: &GameConfig,
) {
    if player.life <= 0 {
        return;
    }
    let direction = player_transform.translation() - enemy_transform.translation;
    let impulse = direction.normalize_or_zero() * 20000.0;
    let damage = ENEMY_ATTACK_POINT;
    player_impulse.impulse = impulse.truncate();
    player.life = (player.life - damage).max(0);
    player.latest_damage = damage;

    play_se(&audio, config, assets.dageki.clone());
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_enemy
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
