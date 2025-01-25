use super::basic::BasicActor;
use super::basic::BasicActorSprite;
use super::ActorSpriteGroup;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
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
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use serde::Deserialize;

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

#[derive(Component, Reflect, Debug)]
pub struct Chest;

#[derive(Reflect, Clone, Debug, Deserialize)]
pub enum ChestItem {
    Gold,
    Item(InventoryItem),
}

pub fn update_sprite(
    asset_server: Res<AssetServer>,
    mut query: Query<(&Parent, &mut AseSpriteAnimation), With<BasicActorSprite>>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    actor_query: Query<&Actor, With<BasicActor>>,
) {
    for (parent, mut animation) in query.iter_mut() {
        let parent = group_query.get(parent.get()).unwrap();
        let actor = actor_query.get(parent.get()).unwrap();
        let ActorExtra::Chest { chest_type, .. } = &actor.extra else {
            continue;
        };
        animation.aseprite = asset_server.load(format!(
            "entity/{}.aseprite",
            match chest_type {
                ChestType::Chest => "chest",
                ChestType::Crate => "crate",
                ChestType::Barrel => "barrel",
                ChestType::BarrelBomb => "barrel_bomb",
                ChestType::Jar(JarColor::Red) => "jar_red",
                ChestType::Jar(JarColor::Blue) => "jar_blue",
                ChestType::Jar(JarColor::Green) => "jar_green",
            }
        ));
    }
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
        app.add_systems(
            FixedUpdate,
            (break_chest, update_sprite).in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Chest>();
    }
}
