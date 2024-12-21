use crate::asset::GameAssets;
use crate::controller::training_dummy::TraningDummyController;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::ActorGroup;
use crate::hud::life_bar::LifeBarResource;
use crate::spell::SpellType;
use bevy::prelude::*;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

pub fn spawn_sandbag(
    mut commands: &mut Commands,
    aseprite: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    spawn_basic_enemy(
        &mut commands,
        aseprite.sandbug.clone(),
        position,
        life_bar_locals,
        TraningDummyController {
            home: position,
            fire: false,
        },
        "sandbag",
        SpellType::SlimeCharge,
        ENEMY_MOVE_FORCE,
        0,
        ActorGroup::Enemy,
        None,
        10000000,
    );
}
