use crate::states::GameState;
use bevy::prelude::*;

/// 木箱やトーチなどの破壊可能なオブジェクトを表すコンポーネントです
/// 弾丸は Breakable コンポーネントを持つエンティティに対してダメージを与えます
#[derive(Default, Component, Reflect)]
pub struct Life {
    /// 破壊可能なオブジェクトのライフ
    /// ゼロになると消滅します
    pub life: i32,

    pub max_life: i32,

    /// ダメージを受けた時の振動の幅
    pub amplitude: f32,
}

impl Life {
    pub fn new(life: i32) -> Self {
        Self {
            life,
            max_life: life,
            amplitude: 0.0,
        }
    }
}

/// ダメージを受けた時に振動するスプライト
#[derive(Default, Component, Reflect)]
pub struct LifeBeingSprite;

fn vibrate_breakabke_sprite(
    time: Res<Time>,
    mut breakable_query: Query<(&mut Life, &Children)>,
    mut breakable_sprite_query: Query<&mut Transform, With<LifeBeingSprite>>,
) {
    for (mut breakable, children) in breakable_query.iter_mut() {
        for child in children {
            if let Ok(mut transform) = breakable_sprite_query.get_mut(*child) {
                transform.translation.x = (time.elapsed_secs() * 56.0).sin() * breakable.amplitude;
            }
            breakable.amplitude *= 0.9;
        }
    }
}

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            vibrate_breakabke_sprite.run_if(in_state(GameState::InGame)),
        );
    }
}
