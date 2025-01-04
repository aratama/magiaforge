use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::constant::*;
use crate::controller::player::Player;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Sensor;

use super::actor::Actor;
use super::actor::ActorGroup;
use super::fire::Burnable;

#[derive(Component)]
pub struct Web {
    owner_actor_group: ActorGroup,
    lifetime: u32,
}

pub fn spawn_web(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    owner_actor_group: ActorGroup,
) {
    commands.spawn((
        Name::new("web"),
        StateScoped(GameState::InGame),
        Web {
            owner_actor_group,
            lifetime: 60 * 60,
        },
        Life::new(4),
        Burnable { life: 30 },
        Transform::from_translation(position.extend(PAINT_LAYER_Z)),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "web".into(),
        },
        (
            Collider::ball(24.0),
            Sensor,
            *SENSOR_GROUPS,
            ActiveEvents::COLLISION_EVENTS,
        ),
    ));
}

fn trap(
    mut player_query: Query<&mut Actor, With<Player>>,
    web_query: Query<&Web>,
    mut events: EventReader<CollisionEvent>,
    mut writer: EventWriter<SEEvent>,
) {
    for event in events.read() {
        match identify(&event, &web_query, &player_query) {
            IdentifiedCollisionEvent::Started(web_entity, player_entity) => {
                info!("1");

                let web = web_query.get(web_entity).unwrap();
                let mut player = player_query.get_mut(player_entity).unwrap();
                if player.trap_moratorium <= 0 && player.actor_group != web.owner_actor_group {
                    info!("2");

                    player.trapped = 60 * 4;
                    writer.send(SEEvent::new(SE::TurnOn));
                }
            }
            _ => {}
        }
    }
}

fn despawn(mut commands: Commands, mut web_query: Query<(Entity, &mut Web, &Burnable)>) {
    for (entity, mut web, burnable) in web_query.iter_mut() {
        web.lifetime -= 1;
        if web.lifetime <= 0 || burnable.life <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct WebPlugin;

impl Plugin for WebPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (trap, despawn)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
