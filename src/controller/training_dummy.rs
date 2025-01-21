use crate::controller::player::Player;
use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

#[derive(Component)]
pub struct TraningDummyController {
    pub home: Vec2,
    pub fire: bool,
}

fn update_enemy_witch_controller(
    mut query: Query<(&mut Actor, &Transform, &TraningDummyController)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut actor, witch_transform, dummy) in query.iter_mut() {
        if dummy.fire {
            if let Ok(player_transform) = player_query.get_single() {
                if player_transform
                    .translation
                    .truncate()
                    .distance(witch_transform.translation.truncate())
                    < 128.0
                {
                    actor.fire_state = ActorFireState::Fire;
                } else {
                    actor.fire_state = ActorFireState::Idle;
                }
            } else {
                actor.fire_state = ActorFireState::Idle;
            }
        } else {
            actor.fire_state = ActorFireState::Idle;
        }

        let diff = dummy.home - witch_transform.translation.truncate();
        actor.move_direction = if 8.0 < diff.length() {
            diff.normalize_or_zero()
        } else {
            Vec2::ZERO
        };
    }
}

pub struct TrainingDummyPlugin;

impl Plugin for TrainingDummyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_enemy_witch_controller.in_set(FixedUpdateGameActiveSet),
        );
    }
}
