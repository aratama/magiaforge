use crate::{
    command::GameCommand,
    constant::{DROPPED_ITEM_GROUP, PLAYER_INTERACTION_SENSOR_GROUP},
    controller::player::Player,
    entity::{actor::Actor, dropped_item::DroppedItemEntity, witch::Witch},
    set::GameSet,
    states::{GameMenuState, GameState},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

/// プレイヤーキャラクターがアイテムを拾ったりギミックの操作をするためのセンサーです
#[derive(Component)]
struct InteractionSensor {
    entities: HashSet<Entity>,
}

pub fn spawn_interaction_sensor(builder: &mut ChildBuilder) {
    builder.spawn((
        InteractionSensor {
            entities: HashSet::new(),
        },
        VisibilityBundle::default(),
        Transform::IDENTITY,
        GlobalTransform::default(),
        RigidBody::Fixed,
        Sensor,
        Collider::cuboid(8.0, 8.0),
        CollisionGroups::new(PLAYER_INTERACTION_SENSOR_GROUP, DROPPED_ITEM_GROUP),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::STATIC_STATIC,
    ));
}

fn update_interaction_sensor_transform(
    actor_query: Query<&Actor, (With<Witch>, With<Player>)>,
    mut sensror_query: Query<(&Parent, &mut Transform), With<InteractionSensor>>,
) {
    for (parent, mut sensor) in sensror_query.iter_mut() {
        if let Ok(actor) = actor_query.get(**parent) {
            let angle = actor.pointer.to_angle();
            sensor.translation = Vec3::new(8.0, 0.0, 0.0);
            sensor.rotation = Quat::IDENTITY;
            sensor.rotate_around(Vec3::ZERO, Quat::from_rotation_z(angle));
        }
    }
}

fn interaction(
    mut sensor_query: Query<&mut InteractionSensor>,
    spell_entity_query: Query<(Entity, &DroppedItemEntity)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                let _ = process_started_event(&mut sensor_query, &spell_entity_query, a, b)
                    || process_started_event(&mut sensor_query, &spell_entity_query, b, a);
            }
            CollisionEvent::Stopped(a, b, _) => {
                let _ = process_stopped_event(&mut sensor_query, &spell_entity_query, a, b)
                    || process_stopped_event(&mut sensor_query, &spell_entity_query, b, a);
            }
        }
    }
}

fn process_started_event(
    sensor_query: &mut Query<&mut InteractionSensor>,
    spell_entity_query: &Query<(Entity, &DroppedItemEntity)>,
    a: &Entity,
    b: &Entity,
) -> bool {
    match (sensor_query.get_mut(*a), spell_entity_query.get(*b)) {
        (Ok(mut sensor), Ok((spell_entity, _))) => {
            sensor.entities.insert(spell_entity);
            true
        }
        _ => false,
    }
}

fn process_stopped_event(
    sensor_query: &mut Query<&mut InteractionSensor>,
    spell_entity_query: &Query<(Entity, &DroppedItemEntity)>,
    a: &Entity,
    b: &Entity,
) -> bool {
    match (sensor_query.get_mut(*a), spell_entity_query.get(*b)) {
        (Ok(mut sensor), Ok((spell_entity, _))) => {
            sensor.entities.remove(&spell_entity);
            true
        }
        _ => false,
    }
}

// インタラクションセンサーで発見されているエンティティのうち、
// プレイヤーに最も近いものに対してインタラクションマーカーを表示します
fn update_interaction_marker_visible(
    player_query: Query<&Transform, With<Player>>,
    interaction_sensor_query: Query<&InteractionSensor>,
    mut spell_query: Query<(Entity, &Transform, &mut DroppedItemEntity)>,
) {
    // ひとまず全てのマーカーを非表示にする
    for (_, _, mut spell) in spell_query.iter_mut() {
        spell.interaction_marker = false;
    }

    // プレイヤーに最も近いスペルエンティティに対してマーカーを表示する
    if let Ok(sensor) = interaction_sensor_query.get_single() {
        if let Ok(player) = player_query.get_single() {
            // センサーに含まれるすべてのエンティティを抽出
            let mut spells = Vec::new();
            for entity in sensor.entities.iter() {
                if let Ok((spell, transform, _)) = spell_query.get(*entity) {
                    spells.push((spell, transform));
                }
            }

            // ソートしてプレイヤーに最も近いエンティティを取得

            if let Some(nearest) =
                pick_nearest_drop_item(&mut spells, player.translation.truncate())
            {
                if let Ok((_, _, mut spell)) = spell_query.get_mut(nearest) {
                    spell.interaction_marker = true;
                }
            }
        }
    }
}

/// ソートしてプレイヤーに最も近いエンティティを取得
pub fn pick_nearest_drop_item(
    entities: &mut Vec<(Entity, &Transform)>,
    from: Vec2,
) -> Option<Entity> {
    entities.sort_by(|(_, a), (_, b)| {
        let a = a.translation.truncate().distance(from);
        let b = b.translation.truncate().distance(from);
        a.total_cmp(&b)
    });

    if let Some((nearest, _)) = entities.first() {
        return Some(*nearest);
    } else {
        return None;
    }
}

fn pick_up(
    mut sensor_query: Query<&InteractionSensor>,
    mut player_query: Query<(&mut Player, &Transform)>,
    spell_query: Query<(Entity, &DroppedItemEntity, &Transform)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut global: EventWriter<GameCommand>,
    state: Res<State<GameMenuState>>,
) {
    if keys.just_pressed(KeyCode::KeyE) && *state.get() == GameMenuState::Closed {
        let sensor = sensor_query.single_mut();
        if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
            // ソートして最も距離が近いものを選択

            // センサーに含まれるすべてのエンティティを抽出
            let mut spells = Vec::new();
            for entity in sensor.entities.iter() {
                if let Ok((entity, _, transform)) = spell_query.get(*entity) {
                    spells.push((entity, transform));
                }
            }

            if let Some(nearest) =
                pick_nearest_drop_item(&mut spells, player_transform.translation.truncate())
            {
                if let Ok((_, DroppedItemEntity { item_type, .. }, _)) = spell_query.get(nearest) {
                    if player.inventory.insert(*item_type) {
                        commands.entity(nearest).despawn_recursive();
                        global.send(GameCommand::SEPickUp(None));
                        // エンティティを削除すれば Stopped イベントが発生してリストから消えるので、
                        // ここで削除する必要はない
                        // entities.remove(entity);
                    } else {
                        warn!("Inventory is full");
                    }
                }
            }
        }
    }
}

pub struct EntityPickerPlugin;

impl Plugin for EntityPickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_interaction_sensor_transform,
                update_interaction_marker_visible,
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            (interaction, pick_up)
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
