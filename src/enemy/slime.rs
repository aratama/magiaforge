use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::player::Player;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::life::Life;
use crate::hud::life_bar::LifeBarResource;
use crate::set::GameSet;
use crate::spell::SpellType;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct SlimeControl;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_RANGE: f32 = TILE_SIZE * 1.0;

pub fn spawn_slime(
    mut commands: &mut Commands,
    aseprite: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    spawn_basic_enemy(
        &mut commands,
        aseprite.slime.clone(),
        position,
        life_bar_locals,
        SlimeControl,
        "slime",
        SpellType::SlimeCharge,
        ENEMY_MOVE_FORCE,
    );
}

/// 1マス以上5マス以内にプレイヤーがいたら追いかけます
/// また、プレイヤーを狙います
fn control_slime(
    mut enemy_query: Query<(&mut Actor, &mut Transform), With<SlimeControl>>,
    mut player_query: Query<(&Life, &GlobalTransform), (With<Player>, Without<SlimeControl>)>,
) {
    if let Ok((enemy_life, player_transform)) = player_query.get_single_mut() {
        if 0 < enemy_life.life {
            for (mut actor, enemy_transform) in enemy_query.iter_mut() {
                let diff = player_transform.translation() - enemy_transform.translation;
                if diff.length() < ENEMY_ATTACK_RANGE {
                    actor.move_direction = Vec2::ZERO;
                    actor.pointer = diff.truncate();
                    actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    actor.move_direction = diff.normalize_or_zero().truncate();
                    actor.fire_state = ActorFireState::Idle;
                } else {
                    actor.move_direction = Vec2::ZERO;
                    actor.fire_state = ActorFireState::Idle;
                }
            }
        }
    }
}

pub struct SlimeControlPlugin;

impl Plugin for SlimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            control_slime
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
