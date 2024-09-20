use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

use bevy_rapier2d::prelude::*;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Person,
        SpriteBundle {
            transform: Transform::from_xyz(20.0, 20.0, 1.0),
            texture: asset_server.load("Pixel Art Top Down Basic/TX Player.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                rect: Some(Rect::new(0.0, 0.0, 32.0, 58.0)),
                ..default()
            },
            ..default()
        },
        KinematicCharacterController::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(10.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

pub fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (&mut Transform, &mut KinematicCharacterController),
        (With<Person>, Without<Camera2d>),
    >,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Person>)>,
) {
    let speed = 3.0;

    let velocity = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero()
        * speed;

    let (mut player, mut controller) = player_query.single_mut();

    player.translation.z = -player.translation.y;
    controller.translation = Some(velocity);

    let mut camera = camera_query.single_mut();

    camera.translation.x += (player.translation.x - camera.translation.x) * 0.1;
    camera.translation.y += (player.translation.y - camera.translation.y) * 0.1;
}

fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}
