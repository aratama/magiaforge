use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::player::Player;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::bullet::BulletType;
use crate::hud::life_bar::LifeBarResource;
use crate::set::GameSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct EyeballControl;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_RANGE: f32 = TILE_SIZE * 8.0;

pub fn spawn_eyeball(
    mut commands: &mut Commands,
    aseprite: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    spawn_basic_enemy(
        &mut commands,
        aseprite.eyeball.clone(),
        position,
        life_bar_locals,
        EyeballControl,
        "eyeball",
        BulletType::PurpleBullet,
        200,
        10,
    );
}

fn control_eyeball(
    mut enemy_query: Query<(&mut Actor, &mut ExternalForce, &mut Transform), With<EyeballControl>>,
    mut player_query: Query<(&Actor, &GlobalTransform), (With<Player>, Without<EyeballControl>)>,
) {
    if let Ok((player, player_transform)) = player_query.get_single_mut() {
        if 0 < player.life {
            for (mut actor, mut force, enemy_transform) in enemy_query.iter_mut() {
                let diff = player_transform.translation() - enemy_transform.translation;
                if diff.length() < ENEMY_ATTACK_RANGE {
                    force.force = Vec2::ZERO;
                    actor.pointer = diff.truncate();
                    actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    let direction = diff.normalize_or_zero();
                    force.force = direction.truncate() * ENEMY_MOVE_FORCE;
                    actor.fire_state = ActorFireState::Idle;
                } else {
                    force.force = Vec2::ZERO;
                    actor.fire_state = ActorFireState::Idle;
                }
            }
        }
    }
}

pub struct EyeballControlPlugin;

impl Plugin for EyeballControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            control_eyeball
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
