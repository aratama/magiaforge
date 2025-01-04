use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::entity::gold::spawn_gold;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::GameSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// 攻撃されてライフがゼロになったら金塊を残して消滅するアクターを表します
#[derive(Component)]
pub struct DespawnWithGold {
    pub golds: u32,
}

/// 敵のライフが0以下になったら消滅させます
/// アクターはライフがゼロになったら消滅しますが、プレイヤーキャラクターの消滅と敵の消滅は処理が異なるので、
/// Enemy側に実装されています
fn dead_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut query: Query<(Entity, &DespawnWithGold, &Life, &Transform)>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, enemy, enemy_life, transform) in query.iter_mut() {
        if enemy_life.life <= 0 {
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(SE::Cry, transform.translation.truncate()));

            for _ in 0..enemy.golds {
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

pub struct DespawnWithGoldPlugin;

impl Plugin for DespawnWithGoldPlugin {
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
