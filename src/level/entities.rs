use super::world::GameLevel;
use super::world::LevelScoped;
use crate::actor::chest::chest_actor;
use crate::actor::chest::Chest;
use crate::actor::chest::ChestType;
use crate::actor::get_default_actor;
use crate::actor::spawn_actor;
use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::collision::SENSOR_GROUPS;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::message_rabbit::SpellListRabbit;
use crate::controller::player::PlayerControlled;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::enemy::huge_slime::Boss;
use crate::entity::broken_magic_circle::spawn_broken_magic_circle;
use crate::entity::bullet_particle::spawn_particle_system;
use crate::entity::bullet_particle::BulletParticleResource;
use crate::entity::bullet_particle::SpawnParticle;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::fire::spawn_fire;
use crate::entity::grass::Grasses;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::servant_seed::spawn_servant_seed;
use crate::entity::shop::spawn_shop_door;
use crate::entity::slash::spawn_slash;
use crate::entity::web::spawn_web;
use crate::inventory::InventoryItem;
use crate::language::Dict;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::spell::Spell;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::Sensor;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::WebSocketState;
use rand::seq::SliceRandom;
use serde::Deserialize;

/// ワールドにエンティティを生成します
/// これは魔法などでエンティティが生成されるときに使うほか、
/// LDtkで定義されたエンティティを生成するときにも使われます
///
/// LDtkでエンティティを生成するには、この列挙型のバリアントと同名のエンティティをLDtk側で定義して、
/// 必要に応じてカスタムフィールドを設定します
#[derive(Clone, Debug, Deserialize)]
pub enum Spawn {
    /// 種別を指定してアクターを生成します
    /// actor_groupはactor_typeごとのデフォルトになります
    Actor {
        actor_type: String,
    },

    ActorWithGroup {
        actor_type: ActorType,
        actor_group: ActorGroup,
    },

    /// Actorを復帰します
    /// 変化からの復帰や分裂のときなどに使います
    Respawn {
        actor: Actor,
        player_controlled: bool,
    },

    Rabbit {
        aseprite: String,
        trigger: String,
    },

    Boss {
        actor_type: ActorType,
        name: Dict<String>,
        on_despawn: String,
    },

    // アクター以外のエンティティ
    Particle {
        particle: SpawnParticle,
    },
    Slash {
        velocity: Vec2,
        actor_group: ActorGroup,
        angle: f32,
        damage: u32,
    },
    Seed {
        to: Vec2,
        actor_group: ActorGroup,
        master: Entity,
        servant_type: ActorType,
        remote: bool,
        servant: bool,
    },
    Web {
        actor_group: ActorGroup,
    },
    Fire,
    MagicCircle {
        destination: MagicCircleProps,
    },
    BrokenMagicCircle,
    Usage,
    ShopSpell,
    ShopDoor,
    RandomChest,
    Spell {
        spell: String,
    },
    SpellInChest {
        spell: Spell,
    },

    Lantern,
    Sandbag,
    Bookshelf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MagicCircleProps {
    #[serde(rename = "entityIid")]
    entity_iid: String,
    #[serde(rename = "levelIid")]
    level_iid: String,
}

/// エンティティを生成するイベントです
/// ゲーム内にエンティティを生成する場合は、基本的にSpawnEventを使います
/// 生成したエンティティに対して追加の処理を行うときのみ、
/// SpawnEventを使わずに直接 spawn_* 関数を呼んで生成します
#[derive(Event, Clone, Debug)]
pub struct SpawnEvent {
    pub position: Vec2,
    pub spawn: Spawn,
}

pub fn spawn_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Registry,
    mut world: ResMut<GameWorld>,
    mut context: Query<&mut RapierContext, With<DefaultRapierContext>>,
    mut se: EventWriter<SEEvent>,
    mut reader: EventReader<SpawnEvent>,
    mut client_message_writer: EventWriter<ClientMessage>,
    mut actor_event: EventWriter<ActorEvent>,
    websocket: Res<WebSocketState>,
    resource: Res<BulletParticleResource>,
    life_query: Query<&Transform, With<Actor>>,
    grass_query: Query<(Entity, &Transform), (With<Grasses>, Without<Actor>)>,
) {
    for SpawnEvent {
        position,
        spawn: entity,
    } in reader.read()
    {
        let Some(level) = world.get_level_by_position(*position) else {
            warn!("No level found at {:?}", position);
            continue;
        };

        if world.shop_items.is_empty() {
            world.shop_items = new_shop_item_queue(
                &registry,
                world
                    .next_state
                    .clone()
                    .unwrap_or_default()
                    .discovered_spells
                    .iter()
                    .cloned()
                    .collect(),
            )
        }

        match &entity {
            Spawn::MagicCircle { destination } => {
                let destination_level = registry.get_level_by_iid(&destination.level_iid);
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    &level,
                    *position,
                    GameLevel(destination_level.identifier.clone()),
                    destination.entity_iid.as_str(),
                );
            }
            Spawn::BrokenMagicCircle => {
                spawn_broken_magic_circle(
                    &mut commands,
                    registry.assets.atlas.clone(),
                    &level,
                    *position,
                );
            }
            Spawn::Usage => {
                commands.spawn((
                    Name::new("usage"),
                    LevelScoped(level.clone()),
                    StateScoped(GameState::InGame),
                    Transform::from_translation(position.extend(PAINT_LAYER_Z)),
                    Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    AseSpriteSlice {
                        aseprite: registry.assets.atlas.clone(),
                        name: "usage".into(),
                    },
                ));
            }
            Spawn::ShopSpell => {
                if let Some(item) = world.shop_items.pop() {
                    spawn_dropped_item(&mut commands, &registry, &level, *position, &item);
                }
            }
            Spawn::Rabbit {
                aseprite: aseprite_value,
                trigger: senario,
            } => {
                let actor = get_default_actor(&registry, &ActorType::new("Rabbit"));

                let entity = spawn_actor(&mut commands, &asset_server, &registry, *position, actor);

                let mut entity = commands.entity(entity);

                entity.insert(MessageRabbit::new(aseprite_value, senario));

                match senario.as_str() {
                    "ShopRabbit" => {
                        entity.insert(ShopRabbit);
                    }
                    "SpellListRabbit" => {
                        entity.insert(SpellListRabbit);
                    }
                    _ => {}
                };

                entity.with_children(|builder| {
                    match senario.as_str() {
                        "ShopRabbit" => {
                            builder.spawn((
                                ShopRabbitSensor,
                                Collider::ball(16.0),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                *SENSOR_GROUPS,
                                Transform::default(), // RabbitSensor経由でフキダシの位置を取得するので、ここにGlobalTransformが必要
                            ));
                        }
                        _ => {
                            builder.spawn((
                                MessageRabbitInnerSensor,
                                Collider::ball(16.0),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                *SENSOR_GROUPS,
                                Transform::default(), // RabbitSensor経由でフキダシの位置を取得するので、ここにGlobalTransformが必要
                            ));
                        }
                    };

                    match senario.as_str() {
                        "ShopRabbit" => {
                            builder.spawn((
                                ShopRabbitOuterSensor,
                                Collider::ball(32.0),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                *SENSOR_GROUPS,
                            ));
                        }
                        _ => {
                            builder.spawn((
                                MessageRabbitOuterSensor,
                                Collider::ball(32.0),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                *SENSOR_GROUPS,
                            ));
                        }
                    };
                });

                //  spawn_rabbit(
                //     &mut commands,
                //     asset_server.load(aseprite),
                //     &registry,
                //     *position,
                //     get_default_actor(&registry, ActorType::new("Rabbit")),
                //     senario,
                // );
            }
            Spawn::ShopDoor => {
                spawn_shop_door(&mut commands, &registry, &level, *position);
            }
            Spawn::RandomChest => {
                spawn_actor_internal(
                    &mut commands,
                    &asset_server,
                    &registry,
                    *position,
                    &&get_default_actor(&registry, &ActorType::new("Chest")),
                    false,
                );
            }
            Spawn::Spell { spell } => {
                spawn_dropped_item(
                    &mut commands,
                    &registry,
                    &level,
                    *position,
                    &InventoryItem::new(Spell(spell.clone())),
                );
            }
            Spawn::SpellInChest { spell } => {
                let actor = chest_actor(0, Some(spell.clone()));
                let entity = spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
                commands.entity(entity).insert(Chest {
                    chest_type: ChestType::Chest,
                });
            }
            Spawn::Actor { actor_type } => {
                let actor = get_default_actor(&registry, &ActorType(actor_type.clone()));
                spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
            }
            Spawn::ActorWithGroup {
                actor_type,
                actor_group,
            } => {
                let mut actor = get_default_actor(&registry, actor_type);
                actor.actor_group = *actor_group;
                spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
            }
            Spawn::Boss {
                actor_type,
                name,
                on_despawn,
            } => {
                let mut actor = get_default_actor(&registry, actor_type);
                actor.actor_group = ActorGroup::Enemy;
                let entity = spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
                commands.entity(entity).insert(Boss {
                    name: name.clone(),
                    on_despawn: on_despawn.clone(),
                });
            }
            &Spawn::Seed {
                to,
                actor_group,
                master: owner,
                servant_type,
                remote,
                servant,
            } => {
                spawn_servant_seed(
                    &mut commands,
                    &asset_server,
                    &registry,
                    &mut client_message_writer,
                    &websocket,
                    *position,
                    *to,
                    *actor_group,
                    *owner,
                    &servant_type,
                    *remote,
                    *servant,
                );
            }

            Spawn::Web {
                actor_group: owner_actor_group,
            } => {
                spawn_web(
                    &mut commands,
                    &registry,
                    &mut se,
                    &level,
                    *position,
                    *owner_actor_group,
                );
            }

            Spawn::Fire => {
                spawn_fire(&mut commands, &registry.assets, *position, None);
            }

            Spawn::Particle { particle: spawn } => {
                spawn_particle_system(&mut commands, *position, &resource, &spawn);
            }

            Spawn::Slash {
                actor_group,
                velocity,
                angle,
                damage,
            } => {
                spawn_slash(
                    &mut commands,
                    &registry,
                    &mut se,
                    *position,
                    *velocity,
                    *angle,
                    &mut context,
                    *actor_group,
                    &mut actor_event,
                    &life_query,
                    &grass_query,
                    *damage,
                );
            }

            Spawn::Respawn {
                actor,
                player_controlled,
            } => {
                spawn_actor_internal(
                    &mut commands,
                    &asset_server,
                    &registry,
                    *position,
                    actor,
                    *player_controlled,
                );
            }

            Spawn::Lantern => {
                let actor = get_default_actor(&registry, &ActorType::new("Lantern"));
                spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
            }
            Spawn::Sandbag => {
                let actor = get_default_actor(&registry, &ActorType::new("Sandbag"));
                spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
            }
            Spawn::Bookshelf => {
                let actor = get_default_actor(&registry, &ActorType::new("Bookshelf"));
                spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
            }
        }
    }
}

fn spawn_actor_internal(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    registry: &Registry,
    position: Vec2,
    actor: &Actor,
    player_controlled: bool,
) {
    let entity = spawn_actor(
        &mut commands,
        &asset_server,
        &registry,
        position,
        actor.clone(),
    );
    if player_controlled {
        commands.entity(entity).insert(PlayerControlled);
    }
}

fn new_shop_item_queue(registry: &Registry, discovered_spells: Vec<Spell>) -> Vec<InventoryItem> {
    let mut rng = rand::thread_rng();

    let mut shop_items: Vec<InventoryItem> = registry
        .spells()
        .iter()
        .filter(|s| discovered_spells.contains(&s) || registry.get_spell_props(*s).rank <= 1)
        .map(|s| InventoryItem {
            spell: s.clone(),
            price: registry.get_spell_props(s).price,
        })
        .collect();

    shop_items.shuffle(&mut rng);

    shop_items
}
