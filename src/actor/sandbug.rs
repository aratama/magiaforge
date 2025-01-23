use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::component::life::Life;
use crate::enemy::basic::spawn_basic_enemy;
use crate::hud::life_bar::LifeBarResource;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::wand::Wand;
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

pub fn default_sandbag() -> (Actor, Life) {
    (
        Actor {
            extra: ActorExtra::Sandbag,
            actor_group: ActorGroup::Neutral,
            wands: Wand::single(Some(SpellType::Jump)),
            ..default()
        },
        Life::new(10000000),
    )
}

pub fn spawn_sandbag(
    mut commands: &mut Commands,
    registry: &Registry,
    life_bar_locals: &Res<LifeBarResource>,
    position: Vec2,
    actor: Actor,
    life: Life,
) -> Entity {
    let entity = spawn_basic_enemy(
        &mut commands,
        &registry,
        life_bar_locals,
        registry.assets.sandbug.clone(),
        position,
        "sandbag",
        None,
        actor,
        life,
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
