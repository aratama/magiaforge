use super::player::Player;
use crate::constant::*;
use crate::entity::actor::Actor;
use crate::entity::gold::spawn_gold;
use crate::command::GameCommand;
use crate::{asset::GameAssets, set::GameSet, states::GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

/// 自動操作でプレイヤーに襲い掛かってくる敵を表します
#[derive(Component)]
pub struct Enemy;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_POINT: i32 = 8;

/// 1マス以上5マス以内にプレイヤーがいたら追いかけます
fn chase_player(
    mut query: Query<(&mut ExternalForce, &mut Transform), (With<Enemy>, Without<Camera2d>)>,
    mut player_query: Query<(&Actor, &GlobalTransform), (With<Player>, Without<Enemy>)>,
) {
    if let Ok((player, player_transform)) = player_query.get_single_mut() {
        for (mut force, enemy_transform) in query.iter_mut() {
            let diff = player_transform.translation() - enemy_transform.translation;
            if 0 < player.life && diff.length() < ENEMY_DETECTION_RANGE {
                let direction = diff.normalize_or_zero();
                force.force = direction.truncate() * ENEMY_MOVE_FORCE;
            } else {
                force.force = Vec2::ZERO;
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

/// プレイヤーキャラクターと接触したらダメージを与えます
fn attack(
    enemy_query: Query<&mut Transform, (With<Enemy>, Without<Camera2d>)>,
    mut player_query: Query<
        (&mut Actor, &GlobalTransform, &mut ExternalImpulse),
        (With<Player>, Without<Enemy>),
    >,
    mut collision_events: EventReader<CollisionEvent>,
    mut writer: EventWriter<GameCommand>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                let _ = process_attack_event(&enemy_query, &mut player_query, &mut writer, a, b)
                    || process_attack_event(&enemy_query, &mut player_query, &mut writer, b, a);
            }
            _ => {}
        }
    }
}

fn process_attack_event(
    enemy_query: &Query<&mut Transform, (With<Enemy>, Without<Camera2d>)>,
    player_query: &mut Query<
        (&mut Actor, &GlobalTransform, &mut ExternalImpulse),
        (With<Player>, Without<Enemy>),
    >,
    writer: &mut EventWriter<GameCommand>,
    player_entity: &Entity,
    enemy_entity: &Entity,
) -> bool {
    if let Ok((mut player, player_transform, mut player_impulse)) =
        player_query.get_mut(*player_entity)
    {
        if let Ok(enemy_transform) = enemy_query.get(*enemy_entity) {
            if player.life <= 0 {
                return false;
            }
            let direction = player_transform.translation() - enemy_transform.translation;
            let impulse = direction.normalize_or_zero() * 20000.0;
            let damage = ENEMY_ATTACK_POINT;
            player_impulse.impulse = impulse.truncate();
            player.life = (player.life - damage).max(0);
            player.latest_damage = damage;

            writer.send(GameCommand::SEDageki);

            return true;
        }
    }

    false
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (chase_player, attack, dead_enemy)
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
