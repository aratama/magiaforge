use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(400.0, 20.0),
        TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)),
    ));

    /* Create the bouncing ball. */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(25.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(
    //         250.0, 400.0, 0.0,
    //     )));
}
