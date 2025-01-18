use std::collections::HashSet;

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
use crate::entity::actor::ActorEvent;
use crate::entity::actor::ActorGroup;
use crate::entity::bgm::spawn_bgm_switch;
use crate::entity::bomb::spawn_bomb;
use crate::entity::book_shelf::spawn_book_shelf;
use crate::entity::broken_magic_circle::spawn_broken_magic_circle;
use crate::entity::bullet_particle::spawn_particle_system;
use crate::entity::bullet_particle::BulletParticleResource;
use crate::entity::bullet_particle::SpawnParticle;
use crate::entity::chest::spawn_chest;
use crate::entity::chest::ChestItem;
use crate::entity::chest::ChestType;
use crate::entity::chest::CHEST_OR_BARREL;
use crate::entity::chest::JARS;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::fireball::spawn_fireball;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::magic_circle::MagicCircleDestination;
use crate::entity::rabbit::spawn_rabbit;
use crate::entity::servant_seed::spawn_servant_seed;
use crate::entity::servant_seed::ServantType;
use crate::entity::shop::spawn_shop_door;
use crate::entity::slash::spawn_slash;
use crate::entity::stone_lantern::spawn_stone_lantern;
use crate::entity::web::spawn_web;
use crate::entity::witch::spawn_witch;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::Inventory;
use crate::message::HELLO_RABBIT_0;
use crate::message::HELLO_RABBIT_1;
use crate::message::HELLO_RABBIT_2;
use crate::message::HELLO_RABBIT_3;
use crate::message::HELLO_RABBIT_4;
use crate::message::HELLO_RABBIT_5;
use crate::message::MULTIPLAY;
use crate::message::RESERCH_RABBIT_0;
use crate::message::RESERCH_RABBIT_1;
use crate::message::RESERCH_RABBIT_2;
use crate::message::RESERCH_RABBIT_3;
use crate::message::RESERCH_RABBIT_4;
use crate::message::RESERCH_RABBIT_5;
use crate::message::SINGLEPLAY_RABBIT_1;
use crate::message::SINGLEPLAY_RABBIT_2;
use crate::message::SPELL_LIST1;
use crate::message::SPELL_LIST2;
use crate::message::SPELL_LIST3;
use crate::message::TRAINING_RABBIT;
use crate::message::TRAINING_RABBIT_1;
use crate::message::TRAINING_RABBIT_2;
use crate::message::TRAINING_RABBIT_3;
use crate::message::TRAINING_RABBIT_4;
use crate::message::TRAINING_RABBIT_5;
use crate::page::in_game::new_shop_item_queue;
use crate::page::in_game::LevelSetup;
use crate::se::SEEvent;
use crate::spell::SpellType;
use crate::theater::Act;
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
}

pub fn spawn_entity(
    mut commands: Commands,
    assets: Res<GameAssets>,
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
) {
    for event in reader.read() {
        if setup.shop_items.is_empty() {
            setup.shop_items = new_shop_item_queue(
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
                    spawn_dropped_item(&mut commands, &assets, *position, item);
                }
            }
            SpawnEntity::HugeSlime { position } => {
                spawn_huge_slime(&mut commands, &assets, *position);
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
                        messages: vec![
                            Act::Speech(TRAINING_RABBIT.to_string()),
                            Act::Speech(TRAINING_RABBIT_1.to_string()),
                            Act::Speech(TRAINING_RABBIT_2.to_string()),
                            Act::Speech(TRAINING_RABBIT_3.to_string()),
                            Act::Speech(TRAINING_RABBIT_4.to_string()),
                            Act::Speech(TRAINING_RABBIT_5.to_string()),
                        ],
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
                        messages: vec![
                            Act::Speech(SINGLEPLAY_RABBIT_1.to_string()),
                            Act::Speech(SINGLEPLAY_RABBIT_2.to_string()),
                        ],
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
                        messages: vec![
                            Act::BGM(Some(assets.saihate.clone())),
                            Act::Speech(HELLO_RABBIT_0.to_string()),
                            Act::Speech(HELLO_RABBIT_1.to_string()),
                            Act::Speech(HELLO_RABBIT_2.to_string()),
                            Act::Speech(HELLO_RABBIT_3.to_string()),
                            Act::Speech(HELLO_RABBIT_4.to_string()),
                            Act::Speech(HELLO_RABBIT_5.to_string()),
                        ],
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
                        messages: vec![Act::Speech(MULTIPLAY.to_string())],
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
                        messages: vec![
                            Act::Speech(RESERCH_RABBIT_0.to_string()),
                            Act::Speech(RESERCH_RABBIT_1.to_string()),
                            Act::Speech(RESERCH_RABBIT_2.to_string()),
                            Act::Speech(RESERCH_RABBIT_3.to_string()),
                            Act::Speech(RESERCH_RABBIT_4.to_string()),
                            Act::Speech(RESERCH_RABBIT_5.to_string()),
                        ],
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
                        messages: vec![
                            Act::Speech(SPELL_LIST1.to_string()),
                            Act::Speech(SPELL_LIST2.to_string()),
                            Act::Speech(SPELL_LIST3.to_string()),
                        ],
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
                spawn_actor_with_default_behavior(
                    &mut commands,
                    &assets,
                    *enemy_type,
                    &life_bar_resource,
                    *actor_group,
                    *position,
                );
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
                );
            }
        }
    }
}

pub fn spawn_actor_with_default_behavior(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    enemy_type: SpawnEnemyType,
    life_bar_resource: &Res<LifeBarResource>,
    actor_group: ActorGroup,
    position: Vec2,
) -> Entity {
    match enemy_type {
        SpawnEnemyType::Slime => {
            let entity = spawn_slime(
                &mut commands,
                &assets,
                position,
                &life_bar_resource,
                0,
                actor_group,
                None,
            );
            commands.entity(entity).insert(SlimeControl::default());
            entity
        }
        SpawnEnemyType::Eyeball => {
            let entity = spawn_eyeball(
                &mut commands,
                &assets,
                position,
                &life_bar_resource,
                actor_group,
                8,
            );
            commands.entity(entity).insert(EyeballControl::default());
            entity
        }
        SpawnEnemyType::Shadow => {
            let entity = spawn_shadow(
                &mut commands,
                &assets,
                &life_bar_resource,
                actor_group,
                position,
            );
            commands.entity(entity).insert(Shadow::default());
            entity
        }
        SpawnEnemyType::Spider => {
            let entity = spawn_spider(
                &mut commands,
                &assets,
                &life_bar_resource,
                actor_group,
                position,
            );
            commands.entity(entity).insert(Spider::default());
            entity
        }
        SpawnEnemyType::Salamander => {
            let entity = spawn_salamander(
                &mut commands,
                &assets,
                &life_bar_resource,
                actor_group,
                position,
            );
            commands.entity(entity).insert(Salamander::default());
            entity
        }
        SpawnEnemyType::Chiken => {
            let entity = spawn_chiken(&mut commands, &assets, &life_bar_resource, position);
            commands.entity(entity).insert(Chicken::default());
            entity
        }
        SpawnEnemyType::Sandbag => {
            let entity = spawn_sandbag(&mut commands, &assets, &life_bar_resource, position);
            commands.entity(entity).insert(Sandbag::new(position));
            entity
        }
    }
}
