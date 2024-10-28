use super::actor::Actor;
use crate::actor::player::Player;
use crate::asset::GameAssets;
use crate::constant::*;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WITCH_COLLIDER_RADIUS: f32 = 5.0;

pub enum WitchType {
    PlayerWitch,
    RemoteWitch,
}

#[derive(Component)]
pub struct LightWithWitch {
    owner: Entity,
}

pub fn spawn_witch(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    x: f32,
    y: f32,
    witch_type: WitchType,
    frame_count: FrameCount,

    life: i32,
    max_life: i32,
) {
    let mut entity = commands.spawn((
        Name::new("player"),
        StateScoped(GameState::InGame),
        Actor {
            cooltime: 0,
            life,
            max_life,
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
        Velocity::default(),
        Collider::ball(WITCH_COLLIDER_RADIUS),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 6.0,
            angular_damping: 1.0,
        },
        ExternalForce::default(),
        ExternalImpulse::default(),
        CollisionGroups::new(ENEMY_GROUP, ENEMY_GROUP | WALL_GROUP | BULLET_GROUP),
    ));

    let index = entity.id();

    match witch_type {
        WitchType::PlayerWitch => entity.insert(Player {
            last_idle_frame_count: frame_count,
            last_ilde_x: x,
            last_ilde_y: y,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: life,
            last_idle_max_life: max_life,
        }),
        WitchType::RemoteWitch => entity.insert(Player {
            last_idle_frame_count: frame_count,
            last_ilde_x: x,
            last_ilde_y: y,
            last_idle_vx: 0.0,
            last_idle_vy: 0.0,
            last_idle_life: life,
            last_idle_max_life: max_life,
        }),
    };
}

pub struct WitchPlugin;

impl Plugin for WitchPlugin {
    fn build(&self, app: &mut App) {}
}
