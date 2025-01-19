use crate::asset::GameAssets;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorTypes;
use crate::hud::life_bar::LifeBarResource;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use bevy::prelude::*;

/// Sandbugは敵のアクターグループですが攻撃を行いません
#[derive(Component)]
pub struct Sandbag {
    home: Vec2,
}

impl Sandbag {
    pub fn new(home: Vec2) -> Self {
        Self { home }
    }
}

pub fn spawn_sandbag(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_locals: &Res<LifeBarResource>,
    position: Vec2,
) -> Entity {
    let entity = spawn_basic_enemy(
        &mut commands,
        &assets,
        assets.sandbug.clone(),
        position,
        life_bar_locals,
        ActorTypes::Sandbag,
        "sandbag",
        Some(SpellType::Jump),
        0,
        ActorGroup::Enemy,
        None,
        10000000,
        8.0,
        false,
    );
    entity
}

fn go_back(mut query: Query<(&mut Actor, &Transform, &Sandbag)>) {
    for (mut actor, witch_transform, dummy) in query.iter_mut() {
        let diff = dummy.home - witch_transform.translation.truncate();
        actor.move_direction = if 8.0 < diff.length() {
            diff.normalize_or_zero()
        } else {
            Vec2::ZERO
        };
    }
}

pub struct SandbagPlugin;

impl Plugin for SandbagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (go_back).in_set(FixedUpdateGameActiveSet));
    }
}
