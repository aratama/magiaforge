pub mod actor;
pub mod bgm;
pub mod bomb;
pub mod book_shelf;
pub mod broken_magic_circle;
pub mod bullet;
pub mod bullet_particle;
pub mod chest;
pub mod damege;
pub mod dropped_item;
pub mod explosion;
pub mod fire;
pub mod fireball;
pub mod gold;
pub mod grass;
pub mod impact;
pub mod magic_circle;
pub mod piece;
pub mod rabbit;
pub mod rock;
pub mod servant_seed;
pub mod shop;
pub mod stone_lantern;
pub mod witch;

use crate::constant::ENTITY_LAYER_Z;
use crate::constant::Z_ORDER_SCALE;
use crate::states::GameState;
use bevy::ecs::query::QueryData;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::query::ROQueryItem;
use bevy::prelude::*;

/// レベルマップで生成されるエンティティです
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum EntityType {
    // 施設
    MagicCircle,
    MagicCircleHome,
    MultiPlayArenaMagicCircle,
    BrokenMagicCircle,
    Usage,
    Routes,
    ShopSpell,
    ShopDoor,
    BGM,

    // ウサギ
    ShopRabbit,
    TrainingRabbit,
    GuideRabbit,
    SinglePlayRabbit,
    MultiplayerRabbit,
    ReadingRabbit,
    SpellListRabbit,

    // 魔法で生成されるもの
    Chest,
    Crate,
    CrateOrBarrel,
    BookShelf,
    StoneLantern,
    HugeSlime,
    Sandbug,
    Bomb,
    Chiken,
}

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
#[derive(Component)]
pub struct EntityChildrenAutoDepth {
    offset: f32,
}

fn update_entity_z(mut query: Query<(&EntityDepth, &mut Transform), Changed<Transform>>) {
    for (depth, mut transform) in query.iter_mut() {
        transform.translation.z = get_entity_z(transform.translation.y) + depth.offset;
    }
}

fn update_entity_chilren_z(
    mut children_query: Query<(&Parent, &mut Transform, &EntityChildrenAutoDepth)>,
    parent_query: Query<&Transform, (Without<EntityChildrenAutoDepth>, Changed<Transform>)>,
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

#[allow(dead_code)]
pub fn select_by_entities<'a, T: QueryData, S: QueryData, F: QueryFilter, G: QueryFilter>(
    qa: &'a Query<T, F>,
    qb: &'a Query<S, G>,
    x: &Entity,
    y: &Entity,
) -> Option<(ROQueryItem<'a, T>, ROQueryItem<'a, S>)> {
    if let Ok(t) = qa.get(*x) {
        if let Ok(s) = qb.get(*y) {
            return Some((t, s));
        }
    }
    if let Ok(t) = qa.get(*y) {
        if let Ok(s) = qb.get(*x) {
            return Some((t, s));
        }
    }
    return None;
}

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_entity_z, update_entity_chilren_z).run_if(in_state(GameState::InGame)),
        );
    }
}
