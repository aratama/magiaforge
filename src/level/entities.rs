use std::collections::HashSet;

use crate::actor::book_shelf::spawn_book_shelf;
use crate::actor::chest::spawn_chest;
use crate::actor::chest::ChestItem;
use crate::actor::chest::ChestType;
use crate::actor::chest::CHEST_OR_BARREL;
use crate::actor::chest::JARS;
use crate::actor::rabbit::spawn_rabbit;
use crate::actor::stone_lantern::spawn_stone_lantern;
use crate::actor::witch::spawn_witch;
use crate::actor::ActorEvent;
use crate::actor::ActorGroup;
use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::message_rabbit::SpellListRabbit;
use crate::controller::player::Player;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::controller::training_dummy::TraningDummyController;
use crate::enemy::chicken::spawn_chiken;
use crate::enemy::chicken::Chicken;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::eyeball::EyeballControl;
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::huge_slime::Boss;
use crate::enemy::salamander::spawn_salamander;
use crate::enemy::salamander::Salamander;
use crate::enemy::sandbug::spawn_sandbag;
use crate::enemy::sandbug::Sandbag;
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
use crate::inventory::Inventory;
use crate::language::Dict;
use crate::page::in_game::new_shop_item_queue;
use crate::page::in_game::LevelSetup;
use crate::se::SEEvent;
use crate::spell::SpellType;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::WebSocketState;
use rand::seq::IteratorRandom;
use uuid::Uuid;

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
    Chest {
        position: Vec2,
        item: ChestItem,
    },
    Crate {
        position: Vec2,
    },
    CrateOrBarrel {
        position: Vec2,
    },
    Jar {
        position: Vec2,
    },
    BookShelf {
        position: Vec2,
    },
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

    Enemy {
        enemy_type: SpawnEnemyType,
        actor_group: ActorGroup,
        position: Vec2,
    },

    /// 変化の復帰のときなどに使います
    SpawnWitch {
        position: Vec2,
        witch: SpawnWitch,
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
        parent: Entity,
        position: Vec2,
        actor_group: ActorGroup,
        angle: f32,
        damage: u32,
    },
}

#[derive(Clone, Debug)]
pub struct SpawnWitch {
    pub wands: [Wand; MAX_WANDS],
    pub inventory: Inventory,
    pub wand: u8,
    pub witch_type: SpawnWitchType,
    pub getting_up: bool,
    pub name: String,
    pub life: u32,
    pub max_life: u32,
    pub golds: u32,
    pub discovered_spells: HashSet<SpellType>,
}

#[derive(Clone, Debug)]
pub enum SpawnWitchType {
    Player,

    #[allow(dead_code)]
    Dummy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnEnemyType {
    Slime,
    Eyeball,
    Shadow,
    Spider,
    Salamander,
    Chiken,
    Sandbag,
    Lantern,
    Chest,
    BookShelf,
}

pub fn spawn_entity(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ron: Res<Assets<GameConstants>>,
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
    let constants = ron.get(assets.spells.id()).unwrap();

    for event in reader.read() {
        if setup.shop_items.is_empty() {
            setup.shop_items = new_shop_item_queue(
                &constants,
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
            SpawnEntity::BookShelf { position } => {
                spawn_book_shelf(&mut commands, assets.atlas.clone(), *position);
            }
            SpawnEntity::Chest { position, item } => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    *position,
                    ChestType::Chest,
                    *item,
                );
            }
            SpawnEntity::Crate { position } => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    *position,
                    ChestType::Crate,
                    ChestItem::Gold(1),
                );
            }
            SpawnEntity::CrateOrBarrel { position } => {
                if rand::random::<u32>() % 4 != 0 {
                    let chest_type = *CHEST_OR_BARREL
                        .iter()
                        .choose(&mut rand::thread_rng())
                        .unwrap();

                    let item = ChestItem::Gold(match chest_type {
                        ChestType::Chest => 10,
                        ChestType::Crate => 1,
                        ChestType::Barrel => 1,
                        ChestType::BarrelBomb => 3,
                        ChestType::Jar(_) => 1,
                    });

                    spawn_chest(
                        &mut commands,
                        assets.atlas.clone(),
                        *position,
                        chest_type,
                        item,
                    );
                }
            }
            SpawnEntity::Jar { position } => {
                let chest_type = *JARS.iter().choose(&mut rand::thread_rng()).unwrap();
                let golds = match chest_type {
                    ChestType::Chest => 10,
                    ChestType::Crate => 1,
                    ChestType::Barrel => 1,
                    ChestType::BarrelBomb => 3,
                    ChestType::Jar(_) => 1,
                };
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    *position,
                    chest_type,
                    ChestItem::Gold(golds),
                );
            }
            SpawnEntity::MagicCircle { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    *position,
                    MagicCircleDestination::NextLevel,
                );
            }
            SpawnEntity::MagicCircleHome { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    *position,
                    MagicCircleDestination::Home,
                );
            }
            SpawnEntity::MultiPlayArenaMagicCircle { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    *position,
                    MagicCircleDestination::MultiplayArena,
                );
            }
            SpawnEntity::MagicCircleDemoEnding { position } => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    *position,
                    MagicCircleDestination::Ending,
                );
            }
            SpawnEntity::BrokenMagicCircle { position } => {
                spawn_broken_magic_circle(&mut commands, assets.atlas.clone(), *position);
            }
            SpawnEntity::StoneLantern { position } => {
                spawn_stone_lantern(&mut commands, &assets, *position);
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
                        aseprite: assets.atlas.clone(),
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
                        aseprite: assets.atlas.clone(),
                        name: "routes".into(),
                    },
                ));
            }
            SpawnEntity::ShopSpell { position } => {
                if let Some(item) = setup.shop_items.pop() {
                    spawn_dropped_item(&mut commands, &assets, &constants, *position, item);
                }
            }
            SpawnEntity::HugeSlime { position, boss } => {
                let entity = spawn_huge_slime(&mut commands, &assets, *position);
                if let Some(boss_name) = boss {
                    commands.entity(entity).insert(Boss {
                        name: boss_name.clone(),
                    });
                }
            }
            SpawnEntity::ShopRabbit { position } => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_yellow,
                    *position,
                    ShopRabbit,
                    ShopRabbitSensor,
                    ShopRabbitOuterSensor,
                );
            }
            SpawnEntity::TrainingRabbit { position } => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_red,
                    *position,
                    MessageRabbit {
                        senario: SenarioType::TrainingRabbit,
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            SpawnEntity::SinglePlayRabbit { position } => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_white,
                    *position,
                    MessageRabbit {
                        senario: SenarioType::SingleplayRabbit,
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            SpawnEntity::GuideRabbit { position } => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_blue,
                    *position,
                    MessageRabbit {
                        senario: SenarioType::HelloRabbit,
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            SpawnEntity::MultiplayerRabbit { position } => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_black,
                    *position,
                    MessageRabbit {
                        senario: SenarioType::MultiplayerRabbit,
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            SpawnEntity::ReadingRabbit { position } => {
                spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_green,
                    *position,
                    MessageRabbit {
                        senario: SenarioType::ReserchRabbit,
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );
            }
            SpawnEntity::SpellListRabbit { position } => {
                let entity = spawn_rabbit(
                    &mut commands,
                    &assets,
                    &assets.rabbit_blue,
                    *position,
                    MessageRabbit {
                        senario: SenarioType::SpellListRabbit,
                    },
                    MessageRabbitInnerSensor,
                    MessageRabbitOuterSensor,
                );

                commands.entity(entity).insert(SpellListRabbit);
            }
            SpawnEntity::ShopDoor { position } => {
                spawn_shop_door(&mut commands, &assets, *position);
            }
            SpawnEntity::BGM { position } => {
                spawn_bgm_switch(&mut commands, &assets, *position);
            }
            SpawnEntity::Bomb { position } => {
                spawn_bomb(&mut commands, &assets, *position);
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
                    &assets,
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

            SpawnEntity::Enemy {
                enemy_type,
                position,
                actor_group,
            } => {
                let entity = spawn_actor(
                    &mut commands,
                    &assets,
                    &life_bar_resource,
                    *enemy_type,
                    *position,
                    *actor_group,
                );
                add_default_behavior(&mut commands, *enemy_type, *position, entity);
            }

            SpawnEntity::Web {
                position,
                owner_actor_group,
            } => {
                spawn_web(
                    &mut commands,
                    &assets,
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
                spawn_fireball(&mut commands, &assets, *position, *velocity, *actor_group);
            }

            SpawnEntity::SpawnWitch {
                position,
                witch:
                    SpawnWitch {
                        wands,
                        inventory,
                        witch_type,
                        wand,
                        getting_up,
                        name,
                        life,
                        max_life,
                        golds,
                        discovered_spells,
                    },
            } => {
                let entity = spawn_witch(
                    &mut commands,
                    &assets,
                    *position,
                    0.0,
                    Uuid::new_v4(),
                    None,
                    *life,
                    *max_life,
                    &life_bar_resource,
                    false,
                    3.0,
                    *golds,
                    wands.clone(),
                    inventory.clone(),
                    match *witch_type {
                        SpawnWitchType::Player => ActorGroup::Player,
                        SpawnWitchType::Dummy => ActorGroup::Enemy,
                    },
                    *wand,
                );
                match *witch_type {
                    SpawnWitchType::Player => {
                        commands.entity(entity).insert(Player::new(
                            name.clone(),
                            *getting_up,
                            discovered_spells,
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

            SpawnEntity::Particle { position, spawn } => {
                spawn_particle_system(&mut commands, *position, &resource, &spawn);
            }

            SpawnEntity::Slash {
                parent,
                actor_group,
                position,
                angle,
                damage,
            } => {
                spawn_slash(
                    &mut commands,
                    &assets,
                    &mut se,
                    *parent,
                    *position,
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
    assets: &Res<GameAssets>,
    life_bar_resource: &Res<LifeBarResource>,
    enemy_type: SpawnEnemyType,
    position: Vec2,
    actor_group: ActorGroup,
) -> Entity {
    match enemy_type {
        SpawnEnemyType::Slime => spawn_slime(
            &mut commands,
            &assets,
            position,
            &life_bar_resource,
            0,
            actor_group,
            None,
        ),
        SpawnEnemyType::Eyeball => spawn_eyeball(
            &mut commands,
            &assets,
            position,
            &life_bar_resource,
            actor_group,
            8,
        ),
        SpawnEnemyType::Shadow => spawn_shadow(
            &mut commands,
            &assets,
            &life_bar_resource,
            actor_group,
            position,
        ),
        SpawnEnemyType::Spider => spawn_spider(
            &mut commands,
            &assets,
            &life_bar_resource,
            actor_group,
            position,
        ),
        SpawnEnemyType::Salamander => spawn_salamander(
            &mut commands,
            &assets,
            &life_bar_resource,
            actor_group,
            position,
        ),
        SpawnEnemyType::Chiken => {
            spawn_chiken(&mut commands, &assets, &life_bar_resource, position)
        }
        SpawnEnemyType::Sandbag => {
            spawn_sandbag(&mut commands, &assets, &life_bar_resource, position)
        }
        SpawnEnemyType::Lantern => spawn_stone_lantern(&mut commands, &assets, position),
        SpawnEnemyType::Chest => spawn_chest(
            &mut commands,
            assets.atlas.clone(),
            position,
            ChestType::Chest,
            ChestItem::Gold(1),
        ),
        SpawnEnemyType::BookShelf => {
            spawn_book_shelf(&mut commands, assets.atlas.clone(), position)
        }
    }
}

pub fn add_default_behavior(
    commands: &mut Commands,
    enemy_type: SpawnEnemyType,
    position: Vec2,
    entity: Entity,
) {
    match enemy_type {
        SpawnEnemyType::Slime => {
            commands.entity(entity).insert(SlimeControl::default());
        }
        SpawnEnemyType::Eyeball => {
            commands.entity(entity).insert(EyeballControl::default());
        }
        SpawnEnemyType::Shadow => {
            commands.entity(entity).insert(Shadow::default());
        }
        SpawnEnemyType::Spider => {
            commands.entity(entity).insert(Spider::default());
        }
        SpawnEnemyType::Salamander => {
            commands.entity(entity).insert(Salamander::default());
        }
        SpawnEnemyType::Chiken => {
            commands.entity(entity).insert(Chicken::default());
        }
        SpawnEnemyType::Sandbag => {
            commands.entity(entity).insert(Sandbag::new(position));
        }
        SpawnEnemyType::Lantern => {}
        SpawnEnemyType::Chest => {}
        SpawnEnemyType::BookShelf => {}
    }
}
