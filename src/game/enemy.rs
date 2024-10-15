use super::{audio::play_se, set::GameSet, states::GameState};
use crate::game::constant::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub life: i32,
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, position: Vec2) {
    commands.spawn((
        Name::new("enemy"),
        StateScoped(GameState::InGame),
        Enemy { life: 20 },
        AsepriteAnimationBundle {
            aseprite: asset_server.load("slime.aseprite"),
            transform: Transform::from_translation(position.extend(5.0)),
            animation: Animation::default().with_tag("idle").with_speed(0.2),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(8.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 6.0,
            angular_damping: 1.0,
        },
    ));
}

fn setup_enemy(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_enemy(
        commands,
        asset_server,
        Vec2::new(TILE_SIZE * 8.0, TILE_SIZE * -10.0),
    );
}

pub fn update_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &Enemy), Without<Camera2d>>,
) {
    for (entity, enemy) in query.iter_mut() {
        commands.entity(entity).remove::<ExternalForce>();
        commands.entity(entity).remove::<ExternalImpulse>();
        if enemy.life <= 0 {
            commands.entity(entity).despawn();
            play_se(&mut commands, &asset_server, "hiyoko.ogg");
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_enemy);
        app.add_systems(
            FixedUpdate,
            update_enemy
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
