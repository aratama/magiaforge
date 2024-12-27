use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{physics::GamePhysics, states::GameState};

/// ゲーム内の時間の流れをカウントする汎用のカウンターです
/// 1フレームに1ずつカウントアップされます
/// ただし、ポーズメニューが開いているなどのときはカウントアップされません
#[derive(Default, Component, Reflect)]
pub struct Counter {
    pub count: u32,
}

/// AseSpriteAnimation と同時に使います
/// カウントアップが友好の場合にのみアニメーションを再生します
#[derive(Default, Component, Reflect)]
pub struct CounterAnimated;

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

fn countup(mut query: Query<&mut Counter>, physics: Res<GamePhysics>) {
    if physics.active {
        for mut counter in query.iter_mut() {
            counter.count += 1;
        }
    }
}

fn animate(
    mut query: Query<&mut AseSpriteAnimation, With<CounterAnimated>>,
    physics: Res<GamePhysics>,
) {
    for mut animation in query.iter_mut() {
        animation.animation.playing = physics.active;

        // animation.animation.playingが未実装のようなので上のコードは効果がないです
        // ワークアラウンド
        animation.animation.speed = if physics.active { 1.0 } else { 0.0 };
    }
}

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (countup, animate)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Counter>();
    }
}
