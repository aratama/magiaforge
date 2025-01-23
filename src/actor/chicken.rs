use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::component::life::Life;
use crate::component::vertical::Vertical;
use crate::controller::player::PlayerServant;
use crate::enemy::basic::spawn_basic_enemy;
use crate::hud::life_bar::LifeBarResource;
use crate::registry::Registry;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::wand::Wand;
use bevy::prelude::*;
use core::f32;

#[derive(Debug)]
enum ChickenState {
    Wait(u32),
    Walk { angle: f32, count: u32 },
}

#[derive(Component, Debug)]
pub struct Chicken {
    state: ChickenState,
}

impl Default for Chicken {
    fn default() -> Self {
        Chicken {
            state: ChickenState::Wait(60),
        }
    }
}

pub fn default_chiken() -> (Actor, Life) {
    (
        Actor {
            extra: ActorExtra::Chicken,
            actor_group: ActorGroup::Neutral,
            wands: Wand::single(Some(SpellType::Jump)),
            ..default()
        },
        Life::new(2),
    )
}

pub fn spawn_chiken(
    mut commands: &mut Commands,
    registry: &Registry,
    life_bar_locals: &Res<LifeBarResource>,
    actor: Actor,
    life: Life,
    position: Vec2,
) -> Entity {
    let entity = spawn_basic_enemy(
        &mut commands,
        &registry,
        life_bar_locals,
        registry.assets.chicken.clone(),
        position,
        "chicken",
        None,
        actor,
        life,
    );
    entity
}

fn control_chiken(mut chiken_query: Query<(&mut Chicken, &mut Actor, Option<&PlayerServant>)>) {
    for (mut chilken, mut actor, servant) in chiken_query.iter_mut() {
        if 0 < actor.frozen {
            continue;
        }

        if servant.is_none() {
            match chilken.state {
                ChickenState::Wait(ref mut count) => {
                    actor.move_direction = Vec2::ZERO;
                    if *count <= 0 {
                        chilken.state = ChickenState::Walk {
                            angle: f32::consts::PI * 2.0 * rand::random::<f32>(),
                            count: 60,
                        };
                    } else {
                        *count -= 1;
                    }
                }
                ChickenState::Walk {
                    ref mut angle,
                    ref mut count,
                } => {
                    actor.move_direction = Vec2::from_angle(*angle);
                    actor.pointer = actor.move_direction.normalize_or_zero();
                    if *count <= 0 {
                        chilken.state = ChickenState::Wait(rand::random::<u32>() % 120 + 120);
                    } else {
                        *count -= 1;
                    }
                }
            }
        }
    }
}

fn hopping(mut chicken_query: Query<(&Chicken, &Actor, &mut Vertical)>, registry: Registry) {
    let props = registry.get_actor_props(ActorType::Chicken);
    for (chicken, actor, mut vertical) in chicken_query.iter_mut() {
        if 0 < actor.frozen {
            continue;
        }
        match chicken.state {
            ChickenState::Wait(..) => {}
            ChickenState::Walk { .. } => {
                if vertical.just_landed || vertical.velocity == 0.0 {
                    vertical.velocity = props.jump;
                }
            }
        }
    }
}

pub struct ChikenControlPlugin;

impl Plugin for ChikenControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (control_chiken, hopping).in_set(FixedUpdateGameActiveSet),
        );
    }
}
