use crate::asset::GameAssets;
use crate::constant::*;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;

pub fn spawn_broken_piece(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    name: &str,
) {
    commands.spawn((
        StateScoped(GameState::InGame),
        Transform::from_translation(position.extend(PIECE_LAYER_Z)).with_rotation(
            Quat::from_rotation_z(2.0 * f32::consts::PI * rand::random::<f32>()),
        ),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: name.into(),
        },
        RigidBody::Dynamic,
        Collider::ball(2.0),
        CollisionGroups::new(PIECE_GROUP, PIECE_GROUP | WALL_GROUP | ENTITY_GROUP),
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
