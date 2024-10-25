use super::super::asset::GameAssets;
use super::super::constant::*;
use super::super::states::GameState;
use super::actor::Actor;
use crate::game::actor::player::Player;
use crate::game::actor::remote::RemotePlayer;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

pub enum WitchType {
    PlayerWitch,
    RemoteWitch,
}

pub fn spawn_witch(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    x: f32,
    y: f32,
    uuid: Uuid,
    witch_type: WitchType,
    frame_count: FrameCount,
) {
    let mut entity = commands.spawn((
        Name::new("player"),
        StateScoped(GameState::InGame),
        Actor {
            uuid,
            cooltime: 0,
            life: 250,
            max_life: 250,
            latest_damage: 0,
            pointer: Vec2::ZERO,
        },
        AsepriteAnimationBundle {
            aseprite: assets.player.clone(),
            transform: Transform::from_xyz(x, y, 1.0),
            animation: Animation::default().with_tag("idle").with_speed(0.2),
            sprite: Sprite {
                // flip_x: true,
                // ここもanchorは効かないことに注意。Aseprite側のpivotで設定
                // anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, 1.0)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(5.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 6.0,
            angular_damping: 1.0,
        },
        ExternalForce::default(),
        ExternalImpulse::default(),
        CollisionGroups::new(PLAYER_GROUP, ENEMY_GROUP | WALL_GROUP),
        PointLight2d {
            radius: 150.0,
            intensity: 3.0,
            falloff: 10.0,
            ..default()
        },
    ));

    match witch_type {
        WitchType::PlayerWitch => entity.insert(Player {}),
        WitchType::RemoteWitch => entity.insert(RemotePlayer {
            last_update: frame_count,
        }),
    };
}
