use crate::command::GameCommand;
use crate::entity::actor::Actor;
use crate::entity::gold::spawn_gold;
use crate::{asset::GameAssets, set::GameSet, states::GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

/// 攻撃されてライフがゼロになったら金塊を残して消滅するアクターを表します
#[derive(Component)]
pub struct Enemy;

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
            writer.send(GameCommand::SECry(Some(transform.translation.truncate())));

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
            dead_enemy
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
