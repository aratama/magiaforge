use bevy::prelude::*;

use crate::{
    constant::{ENTITY_LAYER_Z, Z_ORDER_SCALE},
    states::GameState,
};

/// y座標の値に応じて自動的にz座標を設定し、スプライトの重なりを調整するコンポーネントです
#[derive(Component)]
pub struct EntityDepth {
    offset: f32,
}

impl EntityDepth {
    pub fn new() -> Self {
        Self { offset: 0.0 }
    }
    pub fn offset(offset: f32) -> Self {
        Self { offset }
    }
}

/// 親のy座標に応じて子のz座標を自動で設定します
/// ルートのエンティティではなく子のエンティティになっているスプライトにはEntityChildrenAutoDepthを使います
#[derive(Component)]
pub struct ChildEntityDepth {
    pub offset: f32,
}

fn update_entity_z(mut query: Query<(&EntityDepth, &mut Transform), Changed<Transform>>) {
    for (depth, mut transform) in query.iter_mut() {
        transform.translation.z = get_entity_z(transform.translation.y) + depth.offset;
    }
}

fn update_entity_chilren_z(
    mut children_query: Query<(&Parent, &mut Transform, &ChildEntityDepth)>,
    parent_query: Query<&Transform, (Without<ChildEntityDepth>, Changed<Transform>)>,
) {
    for (parent, mut transform, depth) in children_query.iter_mut() {
        if let Ok(parent_transform) = parent_query.get(parent.get()) {
            transform.translation.z = get_entity_z(parent_transform.translation.y) + depth.offset;
        }
    }
}

pub fn get_entity_z(y: f32) -> f32 {
    ENTITY_LAYER_Z - y * Z_ORDER_SCALE
}

pub struct EntityDepthPlugin;

impl Plugin for EntityDepthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_entity_z, update_entity_chilren_z).run_if(in_state(GameState::InGame)),
        );
    }
}
