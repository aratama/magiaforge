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
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::sandbug::spawn_sandbag;
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
use crate::entity::shop::spawn_shop_door;
use crate::entity::stone_lantern::spawn_stone_lantern;
use crate::entity::EntityType;
use crate::hud::life_bar::LifeBarResource;
use crate::message::HELLO;
use crate::message::HELLO_RABBITS;
use crate::message::HUGE_SLIME;
use crate::message::HUGE_SLIME2;
use crate::message::HUGE_SLIME3;
use crate::message::HUGE_SLIME4;
use crate::message::HUGE_SLIME5;
use crate::message::MULTIPLAY;
use crate::message::SINGLEPLAY;
use crate::message::SPELL_LIST1;
use crate::message::SPELL_LIST2;
use crate::message::SPELL_LIST3;
use crate::message::TRAINING_RABBIT;
use crate::message::WITCHES_ARE;
use crate::page::in_game::new_shop_item_queue;
use crate::page::in_game::LevelSetup;
use crate::theater::Act;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::seq::IteratorRandom;

/// エンティティを生成する汎用のイベントです
/// これは cast のようなシステムで必要なシステムパラメータが増えすぎないようにするためです
/// これは生成時に必要なシステムパラメータが少ないものに使われており、
/// 生成時に必要なシステムパラメータが多いエンティティは専用のイベントを定義します
#[derive(Event)]
pub enum SpawnEntity {
    Spawn { entity: EntityType, position: Vec2 },
}

pub fn spawn_entity(
    mut commands: Commands,
    assets: Res<GameAssets>,
    life_bar_resource: Res<LifeBarResource>,
    mut setup: ResMut<LevelSetup>,
    mut reader: EventReader<SpawnEntity>,
) {
    for spawn in reader.read() {
        match spawn {
            SpawnEntity::Spawn { entity, position } => {
                if setup.shop_items.is_empty() {
                    setup.shop_items = new_shop_item_queue(
                        setup.next_state.discovered_spells.iter().cloned().collect(),
                    )
                }

                let tx = position.x;
                let ty = position.y;
                match entity {
                    EntityType::BookShelf => {
                        spawn_book_shelf(
                            &mut commands,
                            assets.atlas.clone(),
                            Vec2::new(tx + TILE_SIZE, ty - TILE_HALF),
                        );
                    }
                    EntityType::Chest => {
                        spawn_chest(
                            &mut commands,
                            assets.atlas.clone(),
                            tx + TILE_HALF,
                            ty - TILE_HALF,
                            ChestType::Chest,
                        );
                    }
                    EntityType::Crate => {
                        spawn_chest(
                            &mut commands,
                            assets.atlas.clone(),
                            tx + TILE_HALF,
                            ty - TILE_HALF,
                            ChestType::Crate,
                        );
                    }
                    EntityType::CrateOrBarrel => {
                        if rand::random::<u32>() % 4 != 0 {
                            spawn_chest(
                                &mut commands,
                                assets.atlas.clone(),
                                tx + TILE_HALF,
                                ty - TILE_HALF,
                                *CHEST_OR_BARREL
                                    .iter()
                                    .choose(&mut rand::thread_rng())
                                    .unwrap(),
                            );
                        }
                    }
                    EntityType::MagicCircle => {
                        spawn_magic_circle(
                            &mut commands,
                            &assets,
                            tx + TILE_HALF,
                            ty - TILE_HALF,
                            MagicCircleDestination::NextLevel,
                        );
                    }
                    EntityType::MagicCircleHome => {
                        spawn_magic_circle(
                            &mut commands,
                            &assets,
                            tx + TILE_HALF,
                            ty - TILE_HALF,
                            MagicCircleDestination::Home,
                        );
                    }
                    EntityType::MultiPlayArenaMagicCircle => {
                        spawn_magic_circle(
                            &mut commands,
                            &assets,
                            tx + TILE_HALF,
                            ty - TILE_HALF,
                            MagicCircleDestination::MultiplayArena,
                        );
                    }
                    EntityType::BrokenMagicCircle => {
                        spawn_broken_magic_circle(
                            &mut commands,
                            assets.atlas.clone(),
                            tx + TILE_HALF,
                            ty - TILE_HALF,
                        );
                    }
                    EntityType::StoneLantern => {
                        spawn_stone_lantern(
                            &mut commands,
                            &assets,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        );
                    }
                    EntityType::Usage => {
                        commands.spawn((
                            Name::new("usage"),
                            Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
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
                    EntityType::Routes => {
                        commands.spawn((
                            Name::new("routes"),
                            Transform::from_translation(Vec3::new(tx, ty, PAINT_LAYER_Z)),
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
                    EntityType::ShopSpell => {
                        if let Some(item) = setup.shop_items.pop() {
                            spawn_dropped_item(
                                &mut commands,
                                &assets,
                                Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                                item,
                            );
                        }
                    }
                    EntityType::HugeSlime => {
                        spawn_huge_slime(
                            &mut commands,
                            &assets,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        );
                    }
                    EntityType::ShopRabbit => {
                        spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_yellow,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                            ShopRabbit,
                            ShopRabbitSensor,
                            ShopRabbitOuterSensor,
                        );
                    }
                    EntityType::TrainingRabbit => {
                        spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_red,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                            MessageRabbit {
                                messages: vec![Act::Speech(TRAINING_RABBIT.to_string())],
                            },
                            MessageRabbitInnerSensor,
                            MessageRabbitOuterSensor,
                        );
                    }
                    EntityType::SinglePlayRabbit => {
                        spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_white,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                            MessageRabbit {
                                messages: vec![Act::Speech(SINGLEPLAY.to_string())],
                            },
                            MessageRabbitInnerSensor,
                            MessageRabbitOuterSensor,
                        );
                    }
                    EntityType::GuideRabbit => {
                        spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_blue,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
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
                    EntityType::MultiplayerRabbit => {
                        spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_black,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                            MessageRabbit {
                                messages: vec![Act::Speech(MULTIPLAY.to_string())],
                            },
                            MessageRabbitInnerSensor,
                            MessageRabbitOuterSensor,
                        );
                    }
                    EntityType::ReadingRabbit => {
                        spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_green,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                            MessageRabbit {
                                messages: vec![
                                    Act::Speech(WITCHES_ARE.to_string()),
                                    Act::Speech(HUGE_SLIME.to_string()),
                                    Act::Speech(HUGE_SLIME2.to_string()),
                                    Act::Speech(HUGE_SLIME3.to_string()),
                                    Act::Speech(HUGE_SLIME4.to_string()),
                                    Act::Speech(HUGE_SLIME5.to_string()),
                                ],
                            },
                            MessageRabbitInnerSensor,
                            MessageRabbitOuterSensor,
                        );
                    }
                    EntityType::SpellListRabbit => {
                        let entity = spawn_rabbit(
                            &mut commands,
                            &assets,
                            &assets.rabbit_blue,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
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
                    EntityType::Sandbug => {
                        spawn_sandbag(
                            &mut commands,
                            &assets,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                            &life_bar_resource,
                        );
                    }
                    EntityType::ShopDoor => {
                        spawn_shop_door(
                            &mut commands,
                            &assets,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        );
                    }
                    EntityType::BGM => {
                        spawn_bgm_switch(
                            &mut commands,
                            &assets,
                            Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        );
                    }
                    EntityType::Bomb => {
                        spawn_bomb(&mut commands, &assets, *position);
                    }
                    EntityType::Chiken => {
                        spawn_chiken(&mut commands, &assets, &life_bar_resource, *position, false);
                    }
                }
            }
        }
    }
}
