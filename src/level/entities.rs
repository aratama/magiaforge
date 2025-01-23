use crate::actor::book_shelf::spawn_book_shelf;
use crate::actor::chest::spawn_chest;
use crate::actor::chicken::spawn_chiken;
use crate::actor::chicken::Chicken;
use crate::actor::get_default_actor;
use crate::actor::rabbit::default_rabbit;
use crate::actor::rabbit::spawn_rabbit;
use crate::actor::rabbit::RabbitType;
use crate::actor::sandbug::spawn_sandbag;
use crate::actor::sandbug::Sandbag;
use crate::actor::stone_lantern::default_lantern;
use crate::actor::stone_lantern::spawn_stone_lantern;
use crate::actor::witch::spawn_witch;
use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::component::life::Life;
use crate::constant::*;
use crate::controller::message_rabbit::SpellListRabbit;
use crate::controller::player::Player;
use crate::controller::training_dummy::TraningDummyController;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::eyeball::EyeballControl;
use crate::enemy::huge_slime::default_huge_slime;
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::huge_slime::Boss;
use crate::enemy::salamander::spawn_salamander;
use crate::enemy::salamander::Salamander;
use crate::enemy::shadow::spawn_shadow;
use crate::enemy::shadow::Shadow;
use crate::enemy::slime::spawn_slime;
use crate::enemy::slime::SlimeControl;
use crate::enemy::spider::spawn_spider;
use crate::enemy::spider::Spider;
use crate::entity::bgm::spawn_bgm_switch;
use crate::entity::bomb::spawn_bomb;
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
use crate::hud::life_bar::LifeBarResource;
use crate::language::Dict;
use crate::page::in_game::new_shop_item_queue;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::se::SEEvent;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::WebSocketState;

#[derive(Event, Clone, Debug)]
pub enum SpawnEntity {
    // 施設
    MagicCircle {
        position: Vec2,
    },
    MagicCircleHome {
        position: Vec2,
    },
    MultiPlayArenaMagicCircle {
        position: Vec2,
    },
    MagicCircleDemoEnding {
        position: Vec2,
    },
    BrokenMagicCircle {
        position: Vec2,
    },
    Usage {
        position: Vec2,
    },
    Routes {
        position: Vec2,
    },
    ShopSpell {
        position: Vec2,
    },
    ShopDoor {
        position: Vec2,
    },
    BGM {
        position: Vec2,
    },

    // ウサギ
    ShopRabbit {
        position: Vec2,
    },
    TrainingRabbit {
        position: Vec2,
    },
    GuideRabbit {
        position: Vec2,
    },
    SinglePlayRabbit {
        position: Vec2,
    },
    MultiplayerRabbit {
        position: Vec2,
    },
    ReadingRabbit {
        position: Vec2,
    },
    SpellListRabbit {
        position: Vec2,
    },

    // 魔法で生成されるもの
    StoneLantern {
        position: Vec2,
    },
    HugeSlime {
        position: Vec2,
        boss: Option<Dict<String>>,
    },
    Bomb {
        position: Vec2,
    },
    Fireball {
        position: Vec2,
        velocity: Vec2,
        actor_group: ActorGroup,
    },

    Seed {
        from: Vec2,
        to: Vec2,
        actor_group: ActorGroup,
        owner: Option<Entity>,
        servant_type: ServantType,
        remote: bool,
        servant: bool,
    },

    DefaultActor {
        actor_type: ActorType,
        actor_group: ActorGroup,
        position: Vec2,
    },

    /// 変化の復帰のときなどに使います
    Actor {
        position: Vec2,
        life: Life,
        actor: Actor,
    },

    Web {
        position: Vec2,
        owner_actor_group: ActorGroup,
    },

    Particle {
        position: Vec2,
        spawn: SpawnParticle,
    },

    Slash {
        position: Vec2,
        velocity: Vec2,
        actor_group: ActorGroup,
        angle: f32,
        damage: u32,
    },
}

#[derive(Clone, Debug, Reflect)]
pub enum SpawnWitchType {
    Player,

    #[allow(dead_code)]
    Dummy,
}

pub fn spawn_entity(
    mut commands: Commands,
    registry: Registry,
    life_bar_resource: Res<LifeBarResource>,
    mut setup: ResMut<LevelSetup>,
    mut context: Query<&mut RapierContext, With<DefaultRapierContext>>,
    mut se: EventWriter<SEEvent>,
    mut reader: EventReader<SpawnEntity>,
    mut client_message_writer: EventWriter<ClientMessage>,
    mut actor_event: EventWriter<ActorEvent>,
    websocket: Res<WebSocketState>,
    resource: Res<BulletParticleResource>,
    life_query: Query<&Transform, With<Life>>,
    grass_query: Query<(Entity, &Transform), (With<Grasses>, Without<Life>)>,
) {
    for event in reader.read() {
        if setup.shop_items.is_empty() {
            setup.shop_items = new_shop_item_queue(
                &registry,
                setup
                    .next_state
                    .clone()
                    .unwrap_or_default()
                    .discovered_spells
                    .iter()
                    .cloned()
                    .collect(),
            )
        }

        match event {
            SpawnEntity::MagicCircle { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::NextLevel,
                );
            }
            SpawnEntity::MagicCircleHome { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::Home,
                );
            }
            SpawnEntity::MultiPlayArenaMagicCircle { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::MultiplayArena,
                );
            }
            SpawnEntity::MagicCircleDemoEnding { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &registry,
                    *position,
                    MagicCircleDestination::Ending,
                );
            }
            SpawnEntity::BrokenMagicCircle { position } => {
                spawn_broken_magic_circle(&mut commands, registry.assets.atlas.clone(), *position);
            }
            SpawnEntity::StoneLantern { position } => {
                let (actor, life) = default_lantern();
                spawn_stone_lantern(&mut commands, &registry, *position, actor, life);
            }
            SpawnEntity::Usage { position } => {
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
            SpawnEntity::Routes { position } => {
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
            SpawnEntity::ShopSpell { position } => {
                if let Some(item) = setup.shop_items.pop() {
                    spawn_dropped_item(&mut commands, &registry, *position, item);
                }
            }
            SpawnEntity::HugeSlime { position, boss } => {
                let (actor, life) = default_huge_slime();
                let entity = spawn_huge_slime(&mut commands, &registry, *position, actor, life);
                if let Some(boss_name) = boss {
                    commands.entity(entity).insert(Boss {
                        name: boss_name.clone(),
                    });
                }
            }
            SpawnEntity::ShopRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::Shop);
                spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::Shop,
                );
            }
            SpawnEntity::TrainingRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::Training);
                spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::Training,
                );
            }
            SpawnEntity::SinglePlayRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::Singleplay);
                spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::Singleplay,
                );
            }
            SpawnEntity::GuideRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::Guide);
                spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::Guide,
                );
            }
            SpawnEntity::MultiplayerRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::MultiPlay);
                spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::MultiPlay,
                );
            }
            SpawnEntity::ReadingRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::Reading);
                spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::Reading,
                );
            }
            SpawnEntity::SpellListRabbit { position } => {
                let (actor, life) = default_rabbit(RabbitType::SpellList);
                let entity = spawn_rabbit(
                    &mut commands,
                    &registry,
                    *position,
                    actor,
                    life,
                    RabbitType::SpellList,
                );

                commands.entity(entity).insert(SpellListRabbit);
            }
            SpawnEntity::ShopDoor { position } => {
                spawn_shop_door(&mut commands, &registry, *position);
            }
            SpawnEntity::BGM { position } => {
                spawn_bgm_switch(&mut commands, &registry, *position);
            }
            SpawnEntity::Bomb { position } => {
                spawn_bomb(&mut commands, &registry, *position);
            }
            SpawnEntity::Seed {
                from,
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
                    *from,
                    *to,
                    *actor_group,
                    *owner,
                    *servant_type,
                    *remote,
                    *servant,
                );
            }

            SpawnEntity::DefaultActor {
                actor_type,
                actor_group,
                position,
            } => {
                let (mut actor, life) = get_default_actor(*actor_type);
                actor.actor_group = *actor_group;
                let entity = spawn_actor(
                    &mut commands,
                    &registry,
                    &life_bar_resource,
                    *position,
                    actor,
                    life,
                );
                add_default_behavior(&mut commands, *actor_type, *position, entity);
            }

            SpawnEntity::Web {
                position,
                owner_actor_group,
            } => {
                spawn_web(
                    &mut commands,
                    &registry,
                    &mut se,
                    *position,
                    *owner_actor_group,
                );
            }

            SpawnEntity::Fireball {
                position,
                velocity,
                actor_group,
            } => {
                spawn_fireball(&mut commands, &registry, *position, *velocity, *actor_group);
            }

            SpawnEntity::Actor {
                position,
                life,
                actor,
            } => {
                let entity = spawn_actor(
                    &mut commands,
                    &registry,
                    &life_bar_resource,
                    *position,
                    actor.clone(),
                    life.clone(),
                );
                if let ActorExtra::Witch {
                    witch_type,
                    getting_up,
                    name,
                    discovered_spells,
                    ..
                } = actor.extra.clone()
                {
                    match witch_type {
                        SpawnWitchType::Player => {
                            commands.entity(entity).insert(Player::new(
                                name.clone(),
                                getting_up,
                                &discovered_spells,
                            ));
                        }
                        SpawnWitchType::Dummy => {
                            commands.entity(entity).insert(TraningDummyController {
                                home: *position,
                                fire: false,
                            });
                        }
                    };
                }
            }

            SpawnEntity::Particle { position, spawn } => {
                spawn_particle_system(&mut commands, *position, &resource, &spawn);
            }

            SpawnEntity::Slash {
                actor_group,
                position,
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
        }
    }
}

pub fn spawn_actor(
    mut commands: &mut Commands,
    registry: &Registry,
    life_bar_resource: &Res<LifeBarResource>,
    position: Vec2,
    actor: Actor,
    life: Life,
) -> Entity {
    match actor.extra.clone() {
        ActorExtra::Witch { .. } => spawn_witch(
            &mut commands,
            registry,
            position,
            None,
            &life_bar_resource,
            false,
            actor,
            life,
        ),
        ActorExtra::Slime => spawn_slime(
            &mut commands,
            &registry,
            &life_bar_resource,
            actor,
            life,
            position,
            None,
        ),
        ActorExtra::Eyeball => spawn_eyeball(
            &mut commands,
            &registry,
            &life_bar_resource,
            position,
            actor,
            life,
        ),
        ActorExtra::Shadow => spawn_shadow(
            &mut commands,
            &registry,
            &life_bar_resource,
            position,
            actor,
            life,
        ),
        ActorExtra::Spider => spawn_spider(
            &mut commands,
            &registry,
            &life_bar_resource,
            position,
            actor,
            life,
        ),
        ActorExtra::Salamander => spawn_salamander(
            &mut commands,
            &registry,
            &life_bar_resource,
            position,
            actor,
            life,
        ),
        ActorExtra::Chicken => spawn_chiken(
            &mut commands,
            &registry,
            &life_bar_resource,
            actor,
            life,
            position,
        ),
        ActorExtra::Sandbag => spawn_sandbag(
            &mut commands,
            &registry,
            &life_bar_resource,
            position,
            actor,
            life,
        ),
        ActorExtra::Lantern => spawn_stone_lantern(&mut commands, &registry, position, actor, life),
        ActorExtra::Chest { chest_type, .. } => spawn_chest(
            &mut commands,
            registry.assets.atlas.clone(),
            position,
            actor,
            life,
            chest_type,
        ),
        ActorExtra::BookShelf => spawn_book_shelf(
            &mut commands,
            registry.assets.atlas.clone(),
            position,
            actor,
            life,
        ),
        ActorExtra::HugeSlime => spawn_huge_slime(&mut commands, &registry, position, actor, life),
        ActorExtra::Rabbit { rabbit_type } => {
            let (actor, life) = default_rabbit(rabbit_type);
            spawn_rabbit(commands, &registry, position, actor, life, rabbit_type)
        }
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
        ActorType::Slime => {
            commands.entity(entity).insert(SlimeControl::default());
        }
        ActorType::EyeBall => {
            commands.entity(entity).insert(EyeballControl::default());
        }
        ActorType::Shadow => {
            commands.entity(entity).insert(Shadow::default());
        }
        ActorType::Spider => {
            commands.entity(entity).insert(Spider::default());
        }
        ActorType::Salamander => {
            commands.entity(entity).insert(Salamander::default());
        }
        ActorType::Chicken => {
            commands.entity(entity).insert(Chicken::default());
        }
        ActorType::Sandbag => {
            commands.entity(entity).insert(Sandbag::new(position));
        }
        ActorType::Lantern => {}
        ActorType::Chest => {}
        ActorType::BookShelf => {}
        ActorType::Rabbit { .. } => {}
    }
}
