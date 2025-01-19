use crate::component::life::Life;
use crate::set::FixedUpdateInGameSet;
use bevy::prelude::*;

/// 他のアクターに召喚されたアクターであることを表します
/// マスターのアクターが存在しなくなったら、このアクターも消滅します
#[derive(Component, Debug)]
pub struct Servant {
    pub master: Entity,
}

fn despawn_if_no_master(
    mut servant_query: Query<(&mut Life, &Servant)>,
    master_query: Query<Entity, Without<Servant>>,
) {
    for (mut servant_life, servant) in servant_query.iter_mut() {
        if !master_query.contains(servant.master) {
            servant_life.life = 0;
        }
    }
}

pub struct ServantPlugin;

impl Plugin for ServantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            despawn_if_no_master.in_set(FixedUpdateInGameSet),
        );
    }
}
