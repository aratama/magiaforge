use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::constant::PAINT_LAYER_Z;
use crate::constant::SENSOR_GROUP;
use crate::constant::TILE_SIZE;
use crate::constant::WITCH_GROUP;
use crate::controller::player::Player;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct BGMSwitch {
    audio: Handle<AudioSource>,
}

pub fn spawn_bgm_switch(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands.spawn((
        BGMSwitch {
            audio: assets.saihate.clone(),
        },
        StateScoped(GameState::InGame),
        Sensor,
        Collider::cuboid(TILE_SIZE * 3.0, TILE_SIZE * 5.0),
        Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(SENSOR_GROUP, WITCH_GROUP),
    ));

    commands.spawn((
        StateScoped(GameState::InGame),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "facilities".into(),
        },
        Transform::from_translation(Vec3::new(position.x, position.y, PAINT_LAYER_Z)),
    ));
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
        app.add_systems(
            FixedUpdate,
            (sensor,)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
