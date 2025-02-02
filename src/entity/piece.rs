use crate::collision::PIECE_GROUPS;
use crate::constant::*;
use crate::level::world::{GameLevel, LevelScoped};
use crate::registry::Registry;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;

pub fn spawn_broken_piece(
    commands: &mut Commands,
    registry: &Registry,
    level: &GameLevel,
    position: Vec2,
    name: &str,
) {
    commands.spawn((
        LevelScoped(level.clone()),
        StateScoped(GameState::InGame),
        Transform::from_translation(position.extend(PIECE_LAYER_Z)).with_rotation(
            Quat::from_rotation_z(2.0 * f32::consts::PI * rand::random::<f32>()),
        ),
        AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
            name: name.into(),
        },
        RigidBody::Dynamic,
        Collider::ball(4.0),
        *PIECE_GROUPS,
        Velocity {
            linvel: Vec2::new(
                rand::random::<f32>() * 100.0 - 50.0,
                rand::random::<f32>() * 100.0 - 50.0,
            ),
            angvel: rand::random::<f32>() * 10.0 - 5.0,
        },
        Damping {
            linear_damping: 2.0,
            angular_damping: 2.0,
        },
    ));
}
