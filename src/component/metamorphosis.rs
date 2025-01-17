use super::counter::Counter;
use crate::{
    level::entities::{SpawnEntity, SpawnWitch},
    se::{SEEvent, SE},
    set::FixedUpdateGameActiveSet,
};
use bevy::prelude::*;

/// 魔女が変化したあとのモンスターであることを表します
/// 変化残り時間がゼロになったら、このエンティティを削除して、その位置に魔女をスポーンします
#[derive(Component)]
#[require(Counter)]
pub struct Metamorphosis {
    pub witch: SpawnWitch,
}

fn stop(
    mut commands: Commands,
    mut query: Query<(Entity, &Metamorphosis, &Counter, &Transform)>,
    mut writer: EventWriter<SpawnEntity>,
    mut se: EventWriter<SEEvent>,
) {
    for (entity, metamorphosis, counter, transform) in query.iter_mut() {
        if 60 * 10 <= counter.count {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            writer.send(SpawnEntity::SpawnWitch {
                position,
                witch: metamorphosis.witch.clone(),
            });
            se.send(SEEvent::pos(SE::Kyushu2Short, position));
        }
    }
}

pub struct MetamorphosisPlugin;

impl Plugin for MetamorphosisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, stop.in_set(FixedUpdateGameActiveSet));
    }
}
