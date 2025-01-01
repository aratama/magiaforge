use crate::component::life::Life;
use crate::set::GameSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct Servant {
    pub master: Entity,
}

fn despown_if_no_master(
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
            (despown_if_no_master)
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
