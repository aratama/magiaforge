use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AseSpriteAnimation, AseUiAnimation};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{physics::InGameTime, states::GameState};

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

fn countup(mut query: Query<&mut Counter>, in_game_time: Res<InGameTime>) {
    if in_game_time.active {
        for mut counter in query.iter_mut() {
            counter.count += 1;
        }
    }
}

fn animate(
    mut query: Query<&mut AseSpriteAnimation, With<CounterAnimated>>,
    in_game_time: Res<InGameTime>,
) {
    for mut animation in query.iter_mut() {
        animation.animation.playing = in_game_time.active;

        // animation.animation.playingが未実装のようなので上のコードは効果がないです
        // ワークアラウンド
        animation.animation.speed = if in_game_time.active { 1.0 } else { 0.0 };
    }
}

fn animate_ui(
    mut query: Query<&mut AseUiAnimation, With<CounterAnimated>>,
    in_game_time: Res<InGameTime>,
) {
    for mut animation in query.iter_mut() {
        animation.animation.playing = in_game_time.active;

        // animation.animation.playingが未実装のようなので上のコードは効果がないです
        // ワークアラウンド
        animation.animation.speed = if in_game_time.active { 1.0 } else { 0.0 };
    }
}

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (countup, animate, animate_ui)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Counter>();
    }
}
