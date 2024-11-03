use std::f32::consts::PI;

use super::EntityDepth;
use crate::{asset::GameAssets, constant::*, states::GameState};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Default, Component)]
pub struct Gold;

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
/// 大量に生成したときに重なりが減るように、この関数内でランダムな位置にずらしています
pub fn spawn_gold(commands: &mut Commands, assets: &Res<GameAssets>, x: f32, y: f32) {
    let tx = x;
    let ty = y;
    commands.spawn((
        Name::new("gold"),
        StateScoped(GameState::InGame),
        Gold,
        EntityDepth,
        AsepriteSliceBundle {
            aseprite: assets.asset.clone(),
            slice: "gold".into(),
            transform: Transform::from_translation(Vec3::new(
                tx + (random::<f32>() - 0.5) * 16.0,
                ty + (random::<f32>() - 0.5) * 16.0,
                0.0,
            )),
            ..default()
        },
        LockedAxes::ROTATION_LOCKED,
        Velocity::linear(Vec2::from_angle(2.0 * PI * random::<f32>()) * 20.0),
        RigidBody::Dynamic,
        // Restitution::coefficient(0.2),
        // Friction::coefficient(0.2),
        Damping {
            linear_damping: 0.8,
            angular_damping: 0.8,
        },
        Collider::cuboid(1.5, 2.5),
        CollisionGroups::new(ENTITY_GROUP, ENTITY_GROUP | ACTOR_GROUP | WALL_GROUP),
        ActiveCollisionTypes::default(),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, _app: &mut App) {}
}
