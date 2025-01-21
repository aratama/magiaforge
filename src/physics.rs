use crate::set::FixedUpdateInGameSet;
use crate::states::TimeState;
use bevy::ecs::query::QueryData;
use bevy::ecs::query::QueryFilter;
use bevy::ecs::query::QueryItem;
use bevy::ecs::query::ROQueryItem;
use bevy::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_rapier2d::prelude::CollisionEvent;

fn switch_physics_activation(
    state: Res<State<TimeState>>,
    mut rapier_query: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if state.is_changed() {
        match *state.get() {
            TimeState::Active => {
                if let Ok(mut rapier) = rapier_query.get_single_mut() {
                    rapier.physics_pipeline_active = true;
                    rapier.query_pipeline_active = true;
                };
            }
            TimeState::Inactive => {
                if let Ok(mut rapier) = rapier_query.get_single_mut() {
                    rapier.physics_pipeline_active = false;
                    rapier.query_pipeline_active = false;
                };
            }
        }
    }
}

fn identify_items<D, F, G, H>(
    first_query: &Query<D, F>,
    second_query: &Query<G, H>,
    a: Entity,
    b: Entity,
) -> Option<(Entity, Entity)>
where
    D: QueryData,
    F: QueryFilter,
    G: QueryData,
    H: QueryFilter,
{
    if first_query.contains(a) && second_query.contains(b) {
        Some((a, b))
    } else if first_query.contains(b) && second_query.contains(a) {
        Some((b, a))
    } else {
        None
    }
}

fn identify_single_item<D, F>(
    first_query: &Query<D, F>,
    a: Entity,
    b: Entity,
) -> Option<(Entity, Entity)>
where
    D: QueryData,
    F: QueryFilter,
{
    if first_query.contains(a) {
        Some((a, b))
    } else if first_query.contains(b) {
        Some((b, a))
    } else {
        None
    }
}

pub enum IdentifiedCollisionEvent {
    Started(Entity, Entity),
    #[allow(dead_code)]
    Stopped(Entity, Entity),
    Unidentified,
}

pub fn identify<D, F, G, H>(
    collision_event: &CollisionEvent,
    first_query: &Query<D, F>,
    second_query: &Query<G, H>,
) -> IdentifiedCollisionEvent
where
    D: QueryData,
    F: QueryFilter,
    G: QueryData,
    H: QueryFilter,
{
    match collision_event {
        CollisionEvent::Started(a, b, _) => {
            if let Some((first_entity, second_entity)) =
                identify_items(first_query, second_query, *a, *b)
            {
                IdentifiedCollisionEvent::Started(first_entity, second_entity)
            } else {
                IdentifiedCollisionEvent::Unidentified
            }
        }
        CollisionEvent::Stopped(a, b, _) => {
            if let Some((first_entity, second_entity)) =
                identify_items(first_query, second_query, *a, *b)
            {
                IdentifiedCollisionEvent::Stopped(first_entity, second_entity)
            } else {
                IdentifiedCollisionEvent::Unidentified
            }
        }
    }
}

pub fn identify_single<D, F>(
    collistion_event: &CollisionEvent,
    first_query: &Query<D, F>,
) -> IdentifiedCollisionEvent
where
    D: QueryData,
    F: QueryFilter,
{
    match collistion_event {
        CollisionEvent::Started(a, b, _) => {
            if let Some((first_entity, second_entity)) = identify_single_item(first_query, *a, *b) {
                IdentifiedCollisionEvent::Started(first_entity, second_entity)
            } else {
                IdentifiedCollisionEvent::Unidentified
            }
        }
        CollisionEvent::Stopped(a, b, _) => {
            if let Some((first_entity, second_entity)) = identify_single_item(first_query, *a, *b) {
                IdentifiedCollisionEvent::Stopped(first_entity, second_entity)
            } else {
                IdentifiedCollisionEvent::Unidentified
            }
        }
    }
}

pub enum IdentifiedCollisionItem<'a, D1: QueryData, D2: QueryData> {
    #[allow(dead_code)]
    Started(ROQueryItem<'a, D1>, ROQueryItem<'a, D2>, Entity, Entity),
    #[allow(dead_code)]
    Stopped(ROQueryItem<'a, D1>, ROQueryItem<'a, D2>, Entity, Entity),
    Unidentified,
}

pub fn identify_item<'a, D1, F1, D2, F2>(
    collistion_event: &CollisionEvent,
    first_query: &'a Query<D1, F1>,
    second_query: &'a Query<D2, F2>,
) -> IdentifiedCollisionItem<'a, D1, D2>
where
    D1: QueryData,
    F1: QueryFilter,
    D2: QueryData,
    F2: QueryFilter,
{
    match collistion_event {
        CollisionEvent::Started(a, b, _) => {
            if let Some((first_entity, second_entity)) =
                identify_items(first_query, second_query, *a, *b)
            {
                let first: ROQueryItem<'a, D1> = first_query.get(first_entity).unwrap();
                let second: ROQueryItem<'a, D2> = second_query.get(second_entity).unwrap();
                IdentifiedCollisionItem::<'a, D1, D2>::Started(
                    first,
                    second,
                    first_entity,
                    second_entity,
                )
            } else {
                IdentifiedCollisionItem::Unidentified
            }
        }
        CollisionEvent::Stopped(a, b, _) => {
            if let Some((first_entity, second_entity)) =
                identify_items(first_query, second_query, *a, *b)
            {
                let first: ROQueryItem<'_, D1> = first_query.get(first_entity).unwrap();
                let second: ROQueryItem<'_, D2> = second_query.get(second_entity).unwrap();
                IdentifiedCollisionItem::Stopped(first, second, first_entity, second_entity)
            } else {
                IdentifiedCollisionItem::Unidentified
            }
        }
    }
}

pub enum IdentifiedCollisionItemMut<'a, D1: QueryData, D2: QueryData> {
    #[allow(dead_code)]
    Started(QueryItem<'a, D1>, QueryItem<'a, D2>, Entity, Entity),
    #[allow(dead_code)]
    Stopped(QueryItem<'a, D1>, QueryItem<'a, D2>, Entity, Entity),
    Unidentified,
}

#[allow(dead_code)]
pub fn identify_item_mut<'a, D1, F1, D2, F2>(
    collistion_event: &CollisionEvent,
    first_query: &'a mut Query<D1, F1>,
    second_query: &'a mut Query<D2, F2>,
) -> IdentifiedCollisionItemMut<'a, D1, D2>
where
    D1: QueryData,
    F1: QueryFilter,
    D2: QueryData,
    F2: QueryFilter,
{
    match collistion_event {
        CollisionEvent::Started(a, b, _) => {
            if let Some((first_entity, second_entity)) =
                identify_items(first_query, second_query, *a, *b)
            {
                let first: QueryItem<'a, D1> = first_query.get_mut(first_entity).unwrap();
                let second: QueryItem<'a, D2> = second_query.get_mut(second_entity).unwrap();
                IdentifiedCollisionItemMut::<'a, D1, D2>::Started(
                    first,
                    second,
                    first_entity,
                    second_entity,
                )
            } else {
                IdentifiedCollisionItemMut::Unidentified
            }
        }
        CollisionEvent::Stopped(a, b, _) => {
            if let Some((first_entity, second_entity)) =
                identify_items(first_query, second_query, *a, *b)
            {
                let first: QueryItem<'_, D1> = first_query.get_mut(first_entity).unwrap();
                let second: QueryItem<'_, D2> = second_query.get_mut(second_entity).unwrap();
                IdentifiedCollisionItemMut::Stopped(first, second, first_entity, second_entity)
            } else {
                IdentifiedCollisionItemMut::Unidentified
            }
        }
    }
}

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            switch_physics_activation.in_set(FixedUpdateInGameSet),
        );
    }
}
