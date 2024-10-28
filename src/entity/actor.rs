use bevy::prelude::*;

/// ライフを持ち、弾丸のダメージの対象となるエンティティを表します
#[derive(Component)]
pub struct Actor {
    /// 次の魔法を発射できるまでのクールタイム
    pub cooltime: i32,
    pub life: i32,
    pub max_life: i32,
    pub latest_damage: i32,

    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, _app: &mut App) {}
}
