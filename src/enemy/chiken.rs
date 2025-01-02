use core::f32;

use crate::asset::GameAssets;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::hud::life_bar::LifeBarResource;
use crate::physics::InGameTime;
use crate::set::GameSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug)]
enum ChikenState {
    Wait(u32),
    Walk { angle: f32, count: u32 },
}

#[derive(Component, Debug)]
struct Chiken {
    state: ChikenState,
}

const CHIKEN_MOVE_FORCE: f32 = 50000.0;

pub fn spawn_chiken(
    mut commands: &mut Commands,
    aseprite: &Res<GameAssets>,
    life_bar_locals: &Res<LifeBarResource>,
    position: Vec2,
) {
    spawn_basic_enemy(
        &mut commands,
        aseprite.chiken.clone(),
        position,
        life_bar_locals,
        Chiken {
            state: ChikenState::Wait(60),
        },
        "chiken",
        None,
        CHIKEN_MOVE_FORCE,
        0,
        ActorGroup::Neutral,
        None,
        15,
    );
}

fn control_chiken(
    mut chiken_query: Query<(&mut Chiken, &mut Actor)>,
    in_game_timer: Res<InGameTime>,
) {
    if !in_game_timer.active {
        return;
    }

    for (mut chilken, mut actor) in chiken_query.iter_mut() {
        match chilken.state {
            ChikenState::Wait(ref mut count) => {
                actor.move_force = 0.0;
                if *count <= 0 {
                    chilken.state = ChikenState::Walk {
                        angle: f32::consts::PI * 2.0 * rand::random::<f32>(),
                        count: 60,
                    };
                } else {
                    *count -= 1;
                }
            }
            ChikenState::Walk {
                ref mut angle,
                ref mut count,
            } => {
                actor.move_force = CHIKEN_MOVE_FORCE;
                actor.move_direction = Vec2::from_angle(*angle);
                if *count <= 0 {
                    chilken.state = ChikenState::Wait(rand::random::<u32>() % 120 + 120);
                } else {
                    *count -= 1;
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
            (control_chiken)
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
