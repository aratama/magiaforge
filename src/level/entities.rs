use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::message_rabbit::MessageRabbit;
use crate::controller::message_rabbit::MessageRabbitInnerSensor;
use crate::controller::message_rabbit::MessageRabbitOuterSensor;
use crate::controller::message_rabbit::SpellListRabbit;
use crate::controller::shop_rabbit::ShopRabbit;
use crate::controller::shop_rabbit::ShopRabbitOuterSensor;
use crate::controller::shop_rabbit::ShopRabbitSensor;
use crate::enemy::huge_slime::spawn_huge_slime;
use crate::enemy::sandbug::spawn_sandbag;
use crate::entity::bgm::spawn_bgm_switch;
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
use crate::entity::GameEntity;
use crate::equipment::EquipmentType;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::InventoryItem;
use crate::inventory_item::InventoryItemType;
use crate::level::map::LevelChunk;
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
use crate::spell::SpellType;
use crate::theater::Act;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use strum::IntoEnumIterator;

pub fn spawn_entities(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_resource: &Res<LifeBarResource>,
    chunk: &LevelChunk,
    discovered_spells: &HashSet<SpellType>,
) {
    let mut rng = rand::thread_rng();

    let mut shop_items: Vec<InventoryItem> = discovered_spells
        .iter()
        .map(|s| InventoryItem {
            item_type: InventoryItemType::Spell(*s),
            price: s.to_props().price,
        })
        .collect();
    shop_items.extend(
        EquipmentType::iter()
            .filter(|e| e.to_props().rank == 0)
            .map(|e| InventoryItem {
                item_type: InventoryItemType::Equipment(e),
                price: e.to_props().price,
            }),
    );

    shop_items.shuffle(&mut rng);

    // エンティティの生成
    for (entity, x, y) in &chunk.entities {
        let tx = TILE_SIZE * *x as f32;
        let ty = TILE_SIZE * -*y as f32;
        match entity {
            GameEntity::BookShelf => {
                spawn_book_shelf(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_SIZE,
                    ty - TILE_HALF,
                );
            }
            GameEntity::Chest => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    ChestType::Chest,
                );
            }
            GameEntity::Crate => {
                spawn_chest(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    ChestType::Crate,
                );
            }
            GameEntity::CrateOrBarrel => {
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
            GameEntity::MagicCircle => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    MagicCircleDestination::NextLevel,
                );
            }
            GameEntity::MagicCircleHome => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    MagicCircleDestination::Home,
                );
            }
            GameEntity::MultiPlayArenaMagicCircle => {
                spawn_magic_circle(
                    &mut commands,
                    &assets,
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                    MagicCircleDestination::MultiplayArena,
                );
            }
            GameEntity::BrokenMagicCircle => {
                spawn_broken_magic_circle(
                    &mut commands,
                    assets.atlas.clone(),
                    tx + TILE_HALF,
                    ty - TILE_HALF,
                );
            }
            GameEntity::StoneLantern => {
                spawn_stone_lantern(&mut commands, &assets, tx + TILE_HALF, ty - TILE_HALF);
            }
            GameEntity::Usage => {
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
            GameEntity::Routes => {
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
            GameEntity::ShopSpell => {
                if let Some(item) = shop_items.pop() {
                    spawn_dropped_item(
                        &mut commands,
                        &assets,
                        Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                        item,
                    );
                }
            }
            GameEntity::HugeSlime => {
                spawn_huge_slime(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                );
            }
            GameEntity::ShopRabbit => {
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
            GameEntity::TrainingRabbit => {
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
            GameEntity::SinglePlayRabbit => {
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
            GameEntity::GuideRabbit => {
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
            GameEntity::MultiplayerRabbit => {
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
            GameEntity::ReadingRabbit => {
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
            GameEntity::SpellListRabbit => {
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
            GameEntity::Sandbug => {
                spawn_sandbag(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                    life_bar_resource,
                );
            }
            GameEntity::ShopDoor => {
                spawn_shop_door(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                );
            }
            GameEntity::BGM => {
                spawn_bgm_switch(
                    &mut commands,
                    &assets,
                    Vec2::new(tx + TILE_HALF, ty - TILE_HALF),
                );
            }
        }
    }
}
