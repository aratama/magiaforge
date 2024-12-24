pub mod actor;
pub mod bgm;
pub mod book_shelf;
pub mod broken_magic_circle;
pub mod bullet;
pub mod bullet_particle;
pub mod chest;
pub mod damege;
pub mod dropped_item;
pub mod gold;
pub mod impact;
pub mod life;
pub mod magic_circle;
pub mod rabbit;
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameEntity {
    Chest,
    Crate,
    CrateOrBarrel,
    BookShelf,
    MagicCircle,
    MagicCircleHome,
    MultiPlayArenaMagicCircle,
    BrokenMagicCircle,
    Usage,
    Routes,
    StoneLantern,
    Spell,
    Wand,
    HugeSlime,
    ShopRabbit,
    TrainingRabbit,
    GuideRabbit,
    SinglePlayRabbit,
    MultiplayerRabbit,
    ReadingRabbit,
    Sandbug,
    ShopDoor,
    BGM,
}

#[derive(Component)]
pub struct EntityDepth;

/// 親のy座標に応じて子のz座標を自動で設定します
#[derive(Component)]
pub struct EntityChildrenAutoDepth {
    offset: f32,
}

fn update_entity_z(mut query: Query<&mut Transform, (With<EntityDepth>, Changed<Transform>)>) {
    for mut transform in query.iter_mut() {
        transform.translation.z = get_entity_z(transform.translation.y);
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

// pub fn select_by_entities_mut<'a, T: QueryData, S: QueryData, F: QueryFilter, G: QueryFilter>(
//     qa: &'a mut Query<T, F>,
//     qb: &'a Query<S, G>,
//     x: &Entity,
//     y: &Entity,
// ) -> Option<(T::Item<'a>, ROQueryItem<'a, S>)> {
//     {
//         if let Ok(t) = qa.get_mut(*x) {
//             if let Ok(s) = qb.get(*y) {
//                 return Some((t, s));
//             }
//         }
//     }
//     {
//         if let Ok(t) = qa.get_mut(*y) {
//             if let Ok(s) = qb.get(*x) {
//                 return Some((t, s));
//             }
//         }
//     }
//     return None;
// }

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_entity_z, update_entity_chilren_z).run_if(in_state(GameState::InGame)),
        );
    }
}
