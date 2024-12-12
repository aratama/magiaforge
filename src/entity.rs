pub mod actor;
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
pub mod stone_lantern;
pub mod witch;

use crate::{
    constant::{ENTITY_LAYER_Z, Z_ORDER_SCALE},
    states::GameState,
};
use bevy::{
    ecs::query::{QueryData, QueryFilter, ROQueryItem},
    prelude::*,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameEntity {
    Chest,
    BookShelf,
    MagicCircle,
    MagicCircleHome,
    MultiPlayArenaMagicCircle,
    BrokenMagicCircle,
    Usage,
    Routes,
    StoneLantern,
    Spell,
    Crate,
    HugeSlime,
}

#[derive(Component)]
pub struct EntityDepth;

fn update_entity_z(mut query: Query<&mut Transform, (With<EntityDepth>, Changed<Transform>)>) {
    for mut transform in query.iter_mut() {
        transform.translation.z = get_entity_z(transform.translation.y);
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
        app.add_systems(Update, update_entity_z.run_if(in_state(GameState::InGame)));
    }
}
