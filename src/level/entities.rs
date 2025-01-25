use crate::actor::bomb::default_bomb;
use crate::actor::bomb::spawn_bomb;
use crate::actor::book_shelf::spawn_book_shelf;
use crate::actor::chest::chest_actor;
use crate::actor::chest::default_random_chest;
use crate::actor::chest::spawn_chest;
use crate::actor::chest::ChestItem;
use crate::actor::chest::ChestType;
use crate::actor::chicken::spawn_chiken;
use crate::actor::chicken::Chicken;
use crate::actor::get_default_actor;
use crate::actor::rabbit::default_rabbit;
use crate::actor::rabbit::spawn_rabbit;
use crate::actor::rock::default_rock;
use crate::actor::rock::spawn_fallen_rock;
use crate::actor::sandbug::spawn_sandbag;
use crate::actor::sandbug::Sandbag;
use crate::actor::stone_lantern::spawn_stone_lantern;
use crate::actor::witch::spawn_witch;
use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::constant::*;
use crate::controller::player::PlayerControlled;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::huge_slime::Boss;
use crate::enemy::salamander::spawn_salamander;
use crate::enemy::shadow::spawn_shadow;
use crate::enemy::shadow::Shadow;
use crate::enemy::slime::spawn_slime;
use crate::enemy::spider::spawn_spider;
use crate::entity::bgm::spawn_bgm_switch;
use crate::entity::broken_magic_circle::spawn_broken_magic_circle;
use crate::entity::bullet_particle::spawn_particle_system;
use crate::entity::bullet_particle::BulletParticleResource;
use crate::entity::bullet_particle::SpawnParticle;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::fireball::spawn_fireball;
use crate::entity::grass::Grasses;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::magic_circle::MagicCircleDestination;
use crate::entity::servant_seed::spawn_servant_seed;
use crate::entity::servant_seed::ServantType;
use crate::entity::shop::spawn_shop_door;
use crate::entity::slash::spawn_slash;
use crate::entity::web::spawn_web;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::language::Dict;
use crate::page::in_game::new_shop_item_queue;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::spell::Spell;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::WebSocketState;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Spawn {
    /// 種別を指定してアクターを生成します
    Actor {
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
        senario: String,
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
    Fireball {
        velocity: Vec2,
        actor_group: ActorGroup,
    },
    Seed {
        to: Vec2,
        actor_group: ActorGroup,
        owner: Option<Entity>,
        servant_type: ServantType,
        remote: bool,
        servant: bool,
    },
    Web {
        actor_group: ActorGroup,
    },
    MagicCircle,
    MagicCircleHome,
    MultiPlayArenaMagicCircle,
    MagicCircleDemoEnding,
    BrokenMagicCircle,
    Usage,
    Routes,
    ShopSpell,
    ShopDoor,
    BGM,
    RandomChest,
    SpellInChest {
        spell: Spell,
    },
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
    mut level: ResMut<LevelSetup>,
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
        if level.shop_items.is_empty() {
            level.shop_items = new_shop_item_queue(
                &registry,
                level
                    .next_state
                    .clone()
                    .unwrap_or_default()
                    .discovered_spells
                    .iter()
                    .cloned()
                    .collect(),
            )
        }

        match entity {
            Spawn::MagicCircle => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::NextLevel,
                );
            }
            Spawn::MagicCircleHome => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::Home,
                );
            }
            Spawn::MultiPlayArenaMagicCircle => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::MultiplayArena,
                );
            }
            Spawn::MagicCircleDemoEnding => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::Ending,
                );
            }
            Spawn::BrokenMagicCircle => {
                spawn_broken_magic_circle(&mut commands, registry.assets.atlas.clone(), *position);
            }
            Spawn::Usage => {
                commands.spawn((
                    Name::new("usage"),
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
            Spawn::Routes => {
                commands.spawn((
                    Name::new("routes"),
                    Transform::from_translation(position.extend(PAINT_LAYER_Z)),
                    Sprite {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.7),
                        ..default()
                    },
                    AseSpriteSlice {
                        aseprite: registry.assets.atlas.clone(),
                        name: "routes".into(),
                    },
                ));
            }
            Spawn::ShopSpell => {
                if let Some(item) = level.shop_items.pop() {
                    spawn_dropped_item(&mut commands, &registry, *position, &item);
                }
            }
            Spawn::Rabbit { aseprite, senario } => {
                spawn_rabbit(
                    &mut commands,
                    asset_server.load(aseprite),
                    &registry,
                    *position,
                    default_rabbit(aseprite, senario),
                    senario,
                );
            }
            Spawn::ShopDoor => {
                spawn_shop_door(&mut commands, &registry, *position);
            }
            Spawn::BGM => {
                spawn_bgm_switch(&mut commands, &asset_server, &registry, *position);
            }
            Spawn::RandomChest => {
                spawn_actor_internal(
                    &mut commands,
                    &asset_server,
                    &registry,
                    *position,
                    &default_random_chest(),
                    false,
                );
            }
            Spawn::SpellInChest { spell } => {
                let chest_item: ChestItem =
                    ChestItem::Item(InventoryItem::new(InventoryItemType::Spell(spell.clone())));
                spawn_actor_internal(
                    &mut commands,
                    &asset_server,
                    &registry,
                    *position,
                    &chest_actor(ChestType::Chest, chest_item, 0),
                    false,
                );
            }
            Spawn::Actor {
                actor_type,
                actor_group,
            } => {
                let mut actor = get_default_actor(&registry, *actor_type);
                actor.actor_group = *actor_group;
                let entity = spawn_actor(&mut commands, &asset_server, &registry, *position, actor);
                add_default_behavior(&mut commands, *actor_type, *position, entity);
            }
            Spawn::Boss {
                actor_type,
                name,
                on_despawn,
            } => {
                let mut actor = get_default_actor(&registry, *actor_type);
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
                owner,
                servant_type,
                remote,
                servant,
            } => {
                spawn_servant_seed(
                    &mut commands,
                    &registry,
                    &mut client_message_writer,
                    &websocket,
                    *position,
                    to,
                    actor_group,
                    owner,
                    servant_type,
                    remote,
                    servant,
                );
            }
            Spawn::Fireball {
                velocity,
                actor_group,
            } => {
                spawn_fireball(&mut commands, &registry, *position, *velocity, *actor_group);
            }

            Spawn::Web {
                actor_group: owner_actor_group,
            } => {
                spawn_web(
                    &mut commands,
                    &registry,
                    &mut se,
                    *position,
                    *owner_actor_group,
                );
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
        }
    }
}

pub fn spawn_actor(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    match actor.extra.clone() {
        ActorExtra::Witch => spawn_witch(&mut commands, registry, position, None, actor, false),
        ActorExtra::Slime => spawn_slime(&mut commands, &registry, actor, position, None),
        ActorExtra::Eyeball => spawn_eyeball(&mut commands, &registry, position, actor),
        ActorExtra::Shadow => spawn_shadow(&mut commands, &registry, position, actor),
        ActorExtra::Spider => spawn_spider(&mut commands, &registry, position, actor),
        ActorExtra::Salamander => spawn_salamander(&mut commands, &registry, position, actor),
        ActorExtra::Chicken => spawn_chiken(&mut commands, &registry, actor, position),
        ActorExtra::Sandbag => spawn_sandbag(&mut commands, &registry, position, actor),
        ActorExtra::Lantern => spawn_stone_lantern(&mut commands, &registry, position, actor),
        ActorExtra::Chest { chest_type, .. } => spawn_chest(
            &mut commands,
            registry.assets.atlas.clone(),
            position,
            actor,
            chest_type,
        ),
        ActorExtra::BookShelf => spawn_book_shelf(
            &mut commands,
            registry.assets.atlas.clone(),
            // 本棚のみ2タイルの幅があるため、例外的に半タイルずらした位置に生成します
            position + Vec2::new(TILE_HALF, 0.0),
            actor,
        ),
        ActorExtra::HugeSlime => spawn_huge_slime(&mut commands, &registry, position, actor),
        ActorExtra::Rabbit { aseprite, senario } => spawn_rabbit(
            &mut commands,
            asset_server.load(aseprite.clone()),
            &registry,
            position,
            default_rabbit(&aseprite, &senario),
            &senario,
        ),
        ActorExtra::Rock => spawn_fallen_rock(&mut commands, &registry, position, default_rock()),
        ActorExtra::Bomb => spawn_bomb(&mut commands, &registry, position, default_bomb()),
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

pub fn add_default_behavior(
    commands: &mut Commands,
    enemy_type: ActorType,
    position: Vec2,
    entity: Entity,
) {
    match enemy_type {
        ActorType::HugeSlime => {}
        ActorType::Witch => {}
        ActorType::Slime => {}
        ActorType::EyeBall => {}
        ActorType::Shadow => {
            commands.entity(entity).insert(Shadow::default());
        }
        ActorType::Spider => {}
        ActorType::Salamander => {}
        ActorType::Chicken => {
            commands.entity(entity).insert(Chicken::default());
        }
        ActorType::Sandbag => {
            commands.entity(entity).insert(Sandbag::new(position));
        }
        ActorType::Lantern => {}
        ActorType::Chest => {}
        ActorType::BookShelf => {}
        ActorType::Rabbit => {}
        ActorType::Rock => {}
        ActorType::Bomb => {}
    }
}
