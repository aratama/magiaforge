use crate::{
    config::GameConfig,
    constant::{EMPTY_GROUP, ENTITY_GROUP, TILE_SIZE, WALL_GROUP, WITCH_GROUP},
    controller::player::Player,
    speech_bubble::SpeechEvent,
    states::GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::actor::Actor;

#[derive(Component)]
struct ShopDoor;

pub fn spawn_shop_door(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        ShopDoor,
        RigidBody::Fixed,
        Collider::cuboid(TILE_SIZE * 3.0, TILE_SIZE * 2.0),
        Transform::from_translation(position.extend(0.0)),
        CollisionGroups::new(WALL_GROUP, WITCH_GROUP),
    ));
}

fn sensor(
    mut collision_events: EventReader<CollisionEvent>,
    door_query: Query<&ShopDoor>,
    mut player_query: Query<&mut Actor, With<Player>>,
    mut speech_writer: EventWriter<SpeechEvent>,
    config: Res<GameConfig>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _option) => {
                let _ = enter(
                    a,
                    b,
                    &door_query,
                    &mut player_query,
                    &mut speech_writer,
                    &config,
                ) || enter(
                    b,
                    a,
                    &door_query,
                    &mut player_query,
                    &mut speech_writer,
                    &config,
                );
            }
            CollisionEvent::Stopped(..) => {}
        }
    }
}

fn enter(
    a: &Entity,
    b: &Entity,
    sensor_query: &Query<&ShopDoor>,
    player_query: &mut Query<&mut Actor, With<Player>>,
    speech_writer: &mut EventWriter<SpeechEvent>,
    config: &Res<GameConfig>,
) -> bool {
    if sensor_query.contains(*a) {
        info!("Enter shop");
        if sensor_query.contains(*a) {
            if let Ok(actor) = player_query.get(*b) {
                if 0 < actor.dept() {
                    speech_writer.send(SpeechEvent::Speech(
                        config
                            .language
                            .m17n(
                                "おいおい!\nまだ会計してない商品があるよ".to_string(),
                                "Hey, you!\n That item hasn't been paid for yet".to_string(),
                            )
                            .to_string(),
                    ));
                }

                return true;
            }
        }

        return true;
    }
    return false;
}

fn update_door_collision(
    player_query: Query<&mut Actor, (With<Player>, Changed<Actor>)>,
    mut door_query: Query<&mut CollisionGroups, With<ShopDoor>>,
) {
    if let Ok(actor) = player_query.get_single() {
        for mut door in door_query.iter_mut() {
            if actor.dept() == 0 {
                door.memberships = EMPTY_GROUP;
            } else {
                door.memberships = WALL_GROUP;
            }
        }
    }
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (sensor, update_door_collision)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
