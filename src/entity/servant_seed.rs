use crate::actor::get_default_actor;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::constant::*;
use crate::controller::remote::RemoteMessage;
use crate::curve::jump_curve;
use crate::level::entities::spawn_actor;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::BICHA;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component)]
pub struct ServantSeed {
    animation: u32,
    from: Vec2,
    to: Vec2,
    speed: u32,
    // actor_group: ActorGroup,
    // master: Option<Entity>,
    servant_type: ServantType,
    // servant: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ServantType {
    Slime,
    Eyeball,
    Chiken,
}

impl ServantType {
    pub fn to_asset(&self, assets: &Res<GameAssets>, actor_group: ActorGroup) -> Handle<Aseprite> {
        match (self, actor_group) {
            (ServantType::Slime, ActorGroup::Friend) => assets.friend_slime.clone(),
            (ServantType::Slime, ActorGroup::Neutral) => assets.friend_slime.clone(),
            (ServantType::Slime, ActorGroup::Enemy) => assets.slime.clone(),
            (ServantType::Slime, ActorGroup::Entity) => assets.friend_slime.clone(),
            (ServantType::Eyeball, ActorGroup::Friend) => assets.eyeball_friend.clone(),
            (ServantType::Eyeball, ActorGroup::Neutral) => assets.eyeball_friend.clone(),
            (ServantType::Eyeball, ActorGroup::Enemy) => assets.eyeball.clone(),
            (ServantType::Eyeball, ActorGroup::Entity) => assets.eyeball_friend.clone(),
            (ServantType::Chiken, _) => assets.chicken.clone(),
        }
    }
}

#[derive(Component)]
pub struct ServantSeedSprite;

pub fn spawn_servant_seed(
    commands: &mut Commands,
    registry: &Registry,
    writer: &mut EventWriter<ClientMessage>,
    websocket: &Res<WebSocketState>,
    from: Vec2,
    to: Vec2,
    actor_group: ActorGroup,
    _owner: Option<Entity>,
    servant_type: ServantType,
    remote: bool,
    _servant: bool,
) {
    let online = websocket.ready_state == ReadyState::OPEN;

    commands
        .spawn((
            Name::new("servant_seed"),
            StateScoped(GameState::InGame),
            ServantSeed {
                animation: 0,
                from: from,
                to: Vec2::new(
                    to.x + 16.0 * (rand::random::<f32>() - 0.5),
                    to.y + 16.0 * (rand::random::<f32>() - 0.5),
                ),
                speed: 60 + rand::random::<u32>() % 30,
                // actor_group: actor_group,
                // master: owner,
                servant_type: servant_type,
                // servant: servant,
            },
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "entity_shadow".into(),
            },
            Transform::from_translation(from.extend(SHADOW_LAYER_Z)),
        ))
        .with_child((
            ServantSeedSprite,
            CounterAnimated,
            AseSpriteAnimation {
                aseprite: servant_type.to_asset(&registry.assets, actor_group),
                animation: "idle".into(),
            },
        ));

    if remote && online {
        let message = RemoteMessage::ServantSeed {
            from: from,
            to: to,
            actor_group: match actor_group {
                ActorGroup::Friend => ActorGroup::Enemy,
                ActorGroup::Enemy => ActorGroup::Friend,
                ActorGroup::Neutral => ActorGroup::Neutral,
                ActorGroup::Entity => ActorGroup::Entity,
            },
            servant_type: servant_type,
        };
        let serialized = bincode::serialize::<RemoteMessage>(&message).unwrap();
        writer.send(ClientMessage::Binary(serialized));
    }
}

fn update_servant_seed(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ServantSeed, &mut Transform)>,
    mut se_writer: EventWriter<SEEvent>,
    current: Res<LevelSetup>,
    mut spawn_writer: EventWriter<SpawnServantEvent>,
) {
    for (entity, mut seed, mut transform) in query.iter_mut() {
        seed.animation += 1;
        transform.translation = seed
            .from
            .lerp(seed.to, seed.animation as f32 / seed.speed as f32)
            .extend(SERVANT_SEED_LAYER_Z);
        if seed.animation == seed.speed {
            commands.entity(entity).despawn_recursive();

            if let Some(ref chunk) = current.chunk {
                if chunk.get_tile_by_coords(seed.to).is_plane() {
                    spawn_writer.send(SpawnServantEvent {
                        servant_type: seed.servant_type,
                        position: seed.to,
                        // actor_group: seed.actor_group,
                        // master: seed.master,
                        // servant: seed.servant,
                    });
                    se_writer.send(SEEvent::pos(BICHA, seed.to));
                }
            }
        }
    }
}

#[derive(Event, Debug)]
struct SpawnServantEvent {
    servant_type: ServantType,
    position: Vec2,
    // actor_group: ActorGroup,
    // master: Option<Entity>,
    // servant: bool,
}

fn spawn_servant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Registry,
    mut reader: EventReader<SpawnServantEvent>,
) {
    for event in reader.read() {
        spawn_actor(
            &mut commands,
            &asset_server,
            &registry,
            event.position,
            get_default_actor(
                &registry,
                match event.servant_type {
                    ServantType::Slime => ActorType::Slime,
                    ServantType::Eyeball => ActorType::EyeBall,
                    ServantType::Chiken => ActorType::Chicken,
                },
            ),
        );
    }
}

fn update_slime_seed_sprite(
    parent_query: Query<&ServantSeed>,
    mut query: Query<(&Parent, &mut Transform), With<ServantSeedSprite>>,
) {
    for (parent, mut transform) in query.iter_mut() {
        if let Ok(seed) = parent_query.get(parent.get()) {
            transform.translation.y = jump_curve(seed.speed as f32, 100.0, seed.animation as f32);
        }
    }
}

pub struct ServantSeedPlugin;

impl Plugin for ServantSeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnServantEvent>();
        app.add_systems(
            FixedUpdate,
            (update_servant_seed, update_slime_seed_sprite, spawn_servant)
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
