use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorSpriteGroup;
use crate::actor::LifeBeingSprite;
use crate::collision::*;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::explosion::SpawnExplosion;
use crate::entity::fire::Burnable;
use crate::entity::piece::spawn_broken_piece;
use crate::inventory::InventoryItem;
use crate::registry::Registry;
use crate::se::SEEvent;

use crate::se::BREAK;
use crate::se::GLASS;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;
use rand::seq::IteratorRandom;
use serde::Deserialize;

const ENTITY_WIDTH: f32 = 8.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default, Debug, Deserialize)]
pub enum JarColor {
    #[default]
    Red,
    Blue,
    Green,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default, strum::EnumIter, Debug, Deserialize)]
pub enum ChestType {
    #[default]
    Chest,
    Crate,
    Barrel,
    BarrelBomb,
    Jar(JarColor),
}

pub const CHEST_OR_BARREL: [ChestType; 12] = [
    ChestType::Crate,
    ChestType::Crate,
    ChestType::Crate,
    ChestType::Crate,
    ChestType::Barrel,
    ChestType::Barrel,
    ChestType::Barrel,
    ChestType::Barrel,
    ChestType::BarrelBomb,
    ChestType::Jar(JarColor::Red),
    ChestType::Jar(JarColor::Blue),
    ChestType::Jar(JarColor::Green),
];

// pub const JARS: [ChestType; 3] = [
//     ChestType::Jar(JarColor::Red),
//     ChestType::Jar(JarColor::Blue),
//     ChestType::Jar(JarColor::Green),
// ];

#[derive(Component, Reflect, Debug)]
struct Chest;

#[derive(Reflect, Clone, Debug, Deserialize)]
pub enum ChestItem {
    Gold,
    Item(InventoryItem),
}

pub fn default_random_chest() -> Actor {
    let chest_type = *CHEST_OR_BARREL
        .iter()
        .choose(&mut rand::thread_rng())
        .unwrap();

    let golds = match chest_type {
        ChestType::Chest => 10,
        ChestType::Crate => 1,
        ChestType::Barrel => 1,
        ChestType::BarrelBomb => 3,
        ChestType::Jar(_) => 1,
    };

    chest_actor(chest_type, ChestItem::Gold, golds)
}

pub fn chest_actor(chest_type: ChestType, chest_item: ChestItem, golds: u32) -> Actor {
    Actor {
        actor_group: ActorGroup::Entity,
        extra: ActorExtra::Chest {
            chest_type,
            chest_item,
        },
        life: 30,
        max_life: 30,
        golds,
        ..default()
    }
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_chest(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    actor: Actor,
    chest_type: ChestType,
) -> Entity {
    commands
        .spawn((
            Name::new(format!("{:?}", actor.to_type())),
            actor,
            Chest,
            Burnable {
                life: 60 * 20 + rand::random::<u32>() % 30,
            },
            Transform::from_translation(position.extend(0.0)),
            match chest_type {
                ChestType::Jar(_) => Collider::ball(6.0),
                _ => Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
            },
            *ENTITY_GROUPS,
        ))
        .with_children(move |parent| {
            parent.spawn(ActorSpriteGroup).with_child((
                LifeBeingSprite,
                AseSpriteSlice {
                    aseprite: aseprite,
                    name: match chest_type {
                        ChestType::Chest => "chest",
                        ChestType::Crate => "crate",
                        ChestType::Barrel => "barrel",
                        ChestType::BarrelBomb => "barrel_bomb",
                        ChestType::Jar(JarColor::Red) => "jar",
                        ChestType::Jar(JarColor::Blue) => "jar_blue",
                        ChestType::Jar(JarColor::Green) => "jar_green",
                    }
                    .into(),
                },
            ));
        })
        .id()
}

fn break_chest(
    mut commands: Commands,
    query: Query<(&Transform, &Actor, &Burnable), With<Chest>>,
    registry: Registry,
    mut writer: EventWriter<SEEvent>,
    mut explosion: EventWriter<SpawnExplosion>,
) {
    for (transform, actor, burnable) in query.iter() {
        if actor.life <= 0 || burnable.life <= 0 {
            let position = transform.translation.truncate();
            let ActorExtra::Chest {
                chest_type,
                chest_item,
            } = &actor.extra
            else {
                panic!("ActorExtra::Chest is expected");
            };

            writer.send(SEEvent::pos(
                match chest_type {
                    ChestType::Chest => BREAK,
                    ChestType::Crate => BREAK,
                    ChestType::Barrel => BREAK,
                    ChestType::BarrelBomb => BREAK,
                    ChestType::Jar(_) => GLASS,
                },
                position,
            ));

            match chest_item {
                ChestItem::Gold => {}
                ChestItem::Item(item) => {
                    spawn_dropped_item(&mut commands, &registry, position, item);
                }
            }

            match chest_type {
                ChestType::Crate => {
                    for i in 0..4 {
                        spawn_jar_piece(
                            &mut commands,
                            &registry,
                            position,
                            "crate",
                            &JarColor::Red,
                            i,
                        );
                    }
                }
                ChestType::Barrel => {
                    for i in 0..4 {
                        spawn_jar_piece(
                            &mut commands,
                            &registry,
                            position,
                            "barrel",
                            &JarColor::Red,
                            i,
                        );
                    }
                }
                ChestType::BarrelBomb => {
                    explosion.send(SpawnExplosion {
                        position,
                        radius: 60.0,
                        impulse: 100000.0,
                        damage: 100,
                    });
                }
                ChestType::Jar(color) => {
                    for i in 0..4 {
                        spawn_jar_piece(&mut commands, &registry, position, "jar", color, i);
                    }
                }
                _ => {}
            }
        }
    }
}

fn spawn_jar_piece(
    commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    type_name: &str,
    color: &JarColor,
    index: u32,
) {
    spawn_broken_piece(
        commands,
        registry,
        position,
        format!(
            "{}_piece_{}_{}",
            type_name,
            match color {
                JarColor::Red => "red",
                JarColor::Blue => "blue",
                JarColor::Green => "green",
            },
            index
        )
        .as_str(),
    );
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (break_chest).in_set(FixedUpdateGameActiveSet));
        app.register_type::<Chest>();
    }
}
