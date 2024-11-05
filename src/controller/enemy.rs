use super::player::Player;
use crate::command::GameCommand;
use crate::constant::*;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::gold::spawn_gold;
use crate::{asset::GameAssets, set::GameSet, states::GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

/// 自動操作でプレイヤーに襲い掛かってくる敵を表します
#[derive(Component)]
pub struct Enemy;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_RANGE: f32 = TILE_SIZE * 5.0;

/// 1マス以上5マス以内にプレイヤーがいたら追いかけます
/// また、プレイヤーを狙います
fn chase_and_aim_player(
    mut query: Query<
        (&mut ExternalForce, &mut Transform, &mut Actor),
        (With<Enemy>, Without<Camera2d>),
    >,
    mut player_query: Query<(&Actor, &GlobalTransform), (With<Player>, Without<Enemy>)>,
) {
    if let Ok((player, player_transform)) = player_query.get_single_mut() {
        if 0 < player.life {
            for (mut force, enemy_transform, mut actor) in query.iter_mut() {
                let diff = player_transform.translation() - enemy_transform.translation;
                if diff.length() < ENEMY_ATTACK_RANGE {
                    force.force = Vec2::ZERO;
                    actor.pointer = diff.truncate();
                    actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    let direction = diff.normalize_or_zero();
                    force.force = direction.truncate() * ENEMY_MOVE_FORCE;
                    actor.fire_state = ActorFireState::Idle;
                } else {
                    force.force = Vec2::ZERO;
                    actor.fire_state = ActorFireState::Idle;
                }
            }
        }
    }
}

/// 敵のライフが0以下になったら消滅させます
/// アクターはライフがゼロになったら消滅しますが、プレイヤーキャラクターの消滅と敵の消滅は処理が異なるので、
/// Enemy側に実装されています
fn dead_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut query: Query<(Entity, &Actor, &Transform), With<Enemy>>,
    mut writer: EventWriter<GameCommand>,
) {
    for (entity, enemy, transform) in query.iter_mut() {
        if enemy.life <= 0 {
            commands.entity(entity).despawn_recursive();
            writer.send(GameCommand::SEHiyoko);

            for _ in 0..(1 + random::<u32>() % 3) {
                spawn_gold(
                    &mut commands,
                    &assets,
                    transform.translation.x,
                    transform.translation.y,
                );
            }
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                chase_and_aim_player,
                // attack,
                dead_enemy,
            )
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
