use crate::actor::Actor;
use crate::actor::ActorGroup;
use crate::collision::SENSOR_GROUPS;
use crate::component::vertical::Vertical;
use crate::constant::*;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::KAMAE;
use crate::se::ZOMBIE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Sensor;

#[derive(Component)]
pub struct Web {
    owner_actor_group: ActorGroup,

    /// このフレーム数を経過すると自然消滅します
    lifetime: u32,

    /// 粘着力
    /// アクターがこの蜘蛛の巣に触れると、adherenceがアクターのtrappedに加算され、
    /// この値が大きいほどアクターを引き留める時間が長くなります
    adherence: u32,
}

pub fn spawn_web(
    commands: &mut Commands,
    registry: &Registry,
    se: &mut EventWriter<SEEvent>,
    position: Vec2,
    owner_actor_group: ActorGroup,
) {
    se.send(SEEvent::pos(KAMAE, position));
    commands.spawn((
        Name::new("web"),
        StateScoped(GameState::InGame),
        Web {
            owner_actor_group,
            lifetime: 60 * 60,
            adherence: 60 * 4,
        },
        Transform::from_translation(position.extend(PAINT_LAYER_Z)),
        AseSpriteSlice {
            aseprite: registry.assets.atlas.clone(),
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
    mut actor_query: Query<(&mut Actor, &Transform, &Vertical)>,
    web_query: Query<&Web>,
    mut events: EventReader<CollisionEvent>,
    mut writer: EventWriter<SEEvent>,
) {
    for event in events.read() {
        match identify(&event, &web_query, &actor_query) {
            IdentifiedCollisionEvent::Started(web_entity, actor_entity) => {
                let web = web_query.get(web_entity).unwrap();
                let (mut actor, transform, vertical) = actor_query.get_mut(actor_entity).unwrap();
                if actor.trap_moratorium <= 0
                    && actor.actor_group != web.owner_actor_group
                    && vertical.v == 0.0
                {
                    actor.trapped = web.adherence;
                    writer.send(SEEvent::pos(ZOMBIE, transform.translation.truncate()));
                }
            }
            _ => {}
        }
    }
}

fn despawn(mut commands: Commands, mut web_query: Query<(Entity, &mut Web)>) {
    for (entity, mut web) in web_query.iter_mut() {
        web.lifetime -= 1;
        if web.lifetime <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct WebPlugin;

impl Plugin for WebPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (trap, despawn).in_set(FixedUpdateGameActiveSet),
        );
    }
}
