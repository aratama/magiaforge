use std::cmp::Ordering;

use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::{DefaultRapierContext, PhysicsSet, RapierConfiguration};

#[derive(Resource)]
pub struct GamePhysics {
    /// ゲームの物理シミュレーションを実行するかどうか
    /// アニメーションは別です
    pub active: bool,
}

impl Default for GamePhysics {
    fn default() -> Self {
        Self { active: true }
    }
}

fn switch_physics_activation(
    state: Res<GamePhysics>,
    mut rapier_query: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if state.is_changed() {
        if state.active {
            if let Ok(mut rapier) = rapier_query.get_single_mut() {
                rapier.physics_pipeline_active = true;
                rapier.query_pipeline_active = true;
            };
        } else {
            if let Ok(mut rapier) = rapier_query.get_single_mut() {
                rapier.physics_pipeline_active = false;
                rapier.query_pipeline_active = false;
            };
        }
    }
}

pub fn compare_distance(origin: Vec2) -> impl FnMut(&Vec2, &Vec2) -> Ordering {
    move |a, b| {
        let a_diff = a - origin;
        let b_diff = b - origin;
        a_diff.length().partial_cmp(&b_diff.length()).unwrap()
    }
}

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamePhysics>();
        app.add_systems(
            FixedUpdate,
            switch_physics_activation
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
