use crate::asset::GameAssets;
use crate::component::entity_depth::EntityDepth;
use crate::constant::*;
use crate::controller::player::Player;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;
use std::f32::consts::PI;

#[derive(Default, Component, Reflect)]
pub struct Gold {
    pub magnet: bool,
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
/// 大量に生成したときに重なりが減るように、この関数内でランダムな位置にずらしています
pub fn spawn_gold(commands: &mut Commands, assets: &Res<GameAssets>, x: f32, y: f32) {
    let tx = x;
    let ty = y;
    commands.spawn((
        Name::new("gold"),
        StateScoped(GameState::InGame),
        Gold { magnet: false },
        EntityDepth::new(),
        Transform::from_translation(Vec3::new(
            tx + (random::<f32>() - 0.5) * 16.0,
            ty + (random::<f32>() - 0.5) * 16.0,
            0.0,
        )),
        AseSpriteAnimation {
            aseprite: assets.gold.clone(),
            animation: "default".into(),
        },
        LockedAxes::ROTATION_LOCKED,
        Velocity::linear(Vec2::from_angle(2.0 * PI * random::<f32>()) * 20.0),
        RigidBody::Dynamic,
        // Restitution::coefficient(0.2),
        // Friction::coefficient(0.2),
        Damping {
            linear_damping: 5.0,
            angular_damping: 0.8,
        },
        Collider::cuboid(1.5, 2.5),
        *GOLD_GROUPS,
        // ActiveCollisionTypes::default(),
        // ActiveEvents::COLLISION_EVENTS,
        ExternalForce::default(),
    ));
}

fn magnet(
    player_query: Query<&Transform, With<Player>>,
    mut gold_query: Query<(&Gold, &Transform, &mut ExternalForce), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (gold, gold_transform, mut gold_force) in gold_query.iter_mut() {
            if gold.magnet {
                let diff =
                    player_transform.translation.truncate() - gold_transform.translation.truncate();
                gold_force.force = diff.normalize() * 20000.0;
            } else {
                gold_force.force = Vec2::ZERO;
            }
        }
    } else {
        for (_, _, mut gold_force) in gold_query.iter_mut() {
            gold_force.force = Vec2::ZERO;
        }
    }
}

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (magnet)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Gold>();
    }
}
