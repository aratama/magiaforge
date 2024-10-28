use bevy::asset::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule())
        .add_systems(FixedUpdate, update)
        .add_systems(Startup, startup)
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            mode: DebugRenderMode::COLLIDER_SHAPES | DebugRenderMode::COLLIDER_AABBS,
            ..default()
        })
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
    mut commands: Commands,
    frame_count: Res<FrameCount>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Transform>,
) {
    if frame_count.0 % 30 == 0 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("bullet.png"),
                transform: Transform::from_xyz(0.0, 50.0, 0.0),
                ..default()
            },
            (
                Velocity {
                    linvel: Vec2::new(50.0, 0.0),
                    angvel: 0.0,
                },
                RigidBody::KinematicVelocityBased,
                Collider::ball(10.0),
            ),
        ));
    }
}
