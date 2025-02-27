use crate::set::FixedUpdateInGameSet;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_aseprite_ultra::prelude::AseUiAnimation;

/// ゲーム内の時間の流れをカウントする汎用のカウンターです
/// 1フレームに1ずつカウントアップされます
/// ただし、ポーズメニューが開いているなどのときはカウントアップされません
#[derive(Component, Reflect)]
pub struct Counter {
    pub count: i32,
    pub delta: i32,
}

/// カウントアップが有効の場合にのみアニメーションを再生します
/// AseSpriteAnimation や AseUiAnimation と同時に使います
#[derive(Default, Component, Reflect)]
pub struct CounterAnimated;

impl Default for Counter {
    fn default() -> Self {
        Self { count: 0, delta: 1 }
    }
}

impl Counter {
    pub fn up(count: i32) -> Self {
        Self { count, delta: 1 }
    }
    pub fn down(count: i32) -> Self {
        Self { count, delta: -1 }
    }
}

fn count(mut query: Query<&mut Counter>) {
    for mut counter in query.iter_mut() {
        counter.count += counter.delta;
    }
}

fn animate(
    mut query: Query<&mut AseSpriteAnimation, With<CounterAnimated>>,
    in_game_time: Res<State<TimeState>>,
) {
    for mut animation in query.iter_mut() {
        animation.animation.playing = *in_game_time == TimeState::Active;

        // animation.animation.playingが未実装のようなので上のコードは効果がないです
        // ワークアラウンド
        animation.animation.speed = if *in_game_time == TimeState::Active {
            1.0
        } else {
            0.0
        };
    }
}

fn animate_ui(
    mut query: Query<&mut AseUiAnimation, With<CounterAnimated>>,
    in_game_time: Res<State<TimeState>>,
) {
    for mut animation in query.iter_mut() {
        animation.animation.playing = *in_game_time == TimeState::Active;

        // animation.animation.playingが未実装のようなので上のコードは効果がないです
        // ワークアラウンド
        animation.animation.speed = if *in_game_time == TimeState::Active {
            1.0
        } else {
            0.0
        };
    }
}

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (count, animate, animate_ui).in_set(FixedUpdateInGameSet),
        );
        app.register_type::<Counter>();
    }
}
