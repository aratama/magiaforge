use crate::audio::NextBGM;
use crate::controller::player::Player;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::set::FixedUpdateInGameSet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// 使われていません
#[derive(Component)]
struct BGMSwitch {
    audio: Handle<AudioSource>,
}

fn sensor(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<&BGMSwitch>,
    player_query: Query<&Player>,
    mut next: ResMut<NextBGM>,
) {
    for collision_event in collision_events.read() {
        match identify(collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Started(sensor_entity, _) => {
                let bgm = sensor_query.get(sensor_entity).unwrap();
                next.0 = Some(bgm.audio.clone());
            }
            _ => {}
        }
    }
}

pub struct BGMSwitchPlugin;

impl Plugin for BGMSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (sensor,).in_set(FixedUpdateInGameSet));
    }
}
