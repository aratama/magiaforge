use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::message_rabbit::SpellListRabbit;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::enemy::chicken::spawn_chiken;
use crate::enemy::eyeball::spawn_eyeball;
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::sandbug::spawn_sandbag;
use crate::enemy::shadow::spawn_shadow;
use crate::enemy::slime::spawn_slime;
use crate::enemy::spider::spawn_spider;
use crate::entity::actor::ActorGroup;
use crate::entity::bgm::spawn_bgm_switch;
use crate::entity::bomb::spawn_bomb;
use crate::entity::book_shelf::spawn_book_shelf;
use crate::entity::broken_magic_circle::spawn_broken_magic_circle;
use crate::entity::chest::spawn_chest;
use crate::entity::chest::ChestType;
use crate::entity::chest::CHEST_OR_BARREL;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::magic_circle::spawn_magic_circle;
use crate::entity::magic_circle::MagicCircleDestination;
use crate::entity::rabbit::spawn_rabbit;
use crate::entity::servant_seed::spawn_servant_seed;
use crate::entity::servant_seed::ServantType;
use crate::entity::shop::spawn_shop_door;
use crate::entity::stone_lantern::spawn_stone_lantern;
use crate::entity::web::spawn_web;
use crate::hud::life_bar::LifeBarResource;
use crate::message::HELLO;
use crate::message::HELLO_RABBITS;
use crate::message::MULTIPLAY;
use crate::message::RESERCH_RABBIT_0;
use crate::message::RESERCH_RABBIT_1;
use crate::message::RESERCH_RABBIT_2;
use crate::message::RESERCH_RABBIT_3;
use crate::message::RESERCH_RABBIT_4;
use crate::message::RESERCH_RABBIT_5;
use crate::message::SINGLEPLAY;
use crate::message::SINGLEPLAY_2;
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
use crate::theater::Act;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::WebSocketState;
use rand::seq::IteratorRandom;

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
    },
    Crate {
        position: Vec2,
    },
    CrateOrBarrel {
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
    Sandbug {
        position: Vec2,
    },
    Bomb {
        position: Vec2,
    },
    Chiken {
        position: Vec2,
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
        position: Vec2,
    },

    Web {
        position: Vec2,
        owner_actor_group: ActorGroup,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnEnemyType {
    Slime,
    Eyeball,
    Shadow,
    Spider,
}

pub fn spawn_entity(
    mut commands: Commands,
    assets: Res<GameAssets>,
    life_bar_resource: Res<LifeBarResource>,
    mut setup: ResMut<LevelSetup>,
    mut reader: EventReader<SpawnEntity>,
    mut client_message_writer: EventWriter<ClientMessage>,
    websocket: Res<WebSocketState>,
) {
    for event in reader.read() {
        if setup.shop_items.is_empty() {
            setup.shop_items =
                new_shop_item_queue(setup.next_state.discovered_spells.iter().cloned().collect())
        }

        match event {
            SpawnEntity::BookShelf { position } => {
                spawn_book_shelf(&mut commands, assets.atlas.clone(), *position);
            }
            SpawnEntity::Chest { position } => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    *position,
                    ChestType::Chest,
                );
            }
            SpawnEntity::Crate { position } => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    *position,
                    ChestType::Crate,
                );
            }
            SpawnEntity::CrateOrBarrel { position } => {
                if rand::random::<u32>() % 4 != 0 {
                    spawn_chest(
                        &mut commands,
                        assets.atlas.clone(),
                        *position,
                        *CHEST_OR_BARREL
                            .iter()
                            .choose(&mut rand::thread_rng())
                            .unwrap(),
                    );
                }
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
                            Act::Speech(SINGLEPLAY.to_string()),
                            Act::Speech(SINGLEPLAY_2.to_string()),
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
                            Act::Speech(HELLO.to_string()),
                            Act::Speech(HELLO_RABBITS.to_string()),
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
            SpawnEntity::Sandbug { position } => {
                spawn_sandbag(&mut commands, &assets, *position, &life_bar_resource);
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
            SpawnEntity::Chiken { position } => {
                spawn_chiken(&mut commands, &assets, &life_bar_resource, *position, false);
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
            } => match enemy_type {
                SpawnEnemyType::Slime => {
                    spawn_slime(
                        &mut commands,
                        &assets,
                        *position,
                        &life_bar_resource,
                        0,
                        5,
                        ActorGroup::Enemy,
                        None,
                    );
                }
                SpawnEnemyType::Eyeball => {
                    spawn_eyeball(
                        &mut commands,
                        &assets,
                        *position,
                        &life_bar_resource,
                        ActorGroup::Enemy,
                        8,
                    );
                }
                SpawnEnemyType::Shadow => {
                    spawn_shadow(&mut commands, &assets, &life_bar_resource, *position);
                }
                SpawnEnemyType::Spider => {
                    spawn_spider(&mut commands, &assets, &life_bar_resource, *position);
                }
            },

            SpawnEntity::Web {
                position,
                owner_actor_group,
            } => {
                spawn_web(&mut commands, &assets, *position, *owner_actor_group);
            }
        }
    }
}
