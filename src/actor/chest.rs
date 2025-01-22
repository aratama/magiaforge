use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorSpriteGroup;
use crate::asset::GameAssets;
use crate::collision::*;
use crate::component::falling::Falling;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::GameConstants;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::entity::explosion::SpawnExplosion;
use crate::entity::fire::Burnable;
use crate::entity::gold::spawn_gold;
use crate::entity::piece::spawn_broken_piece;
use crate::inventory::InventoryItem;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;
use rand::seq::IteratorRandom;

const ENTITY_WIDTH: f32 = 8.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default, Debug)]
pub enum JarColor {
    #[default]
    Red,
    Blue,
    Green,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default, strum::EnumIter, Debug)]
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

#[derive(Reflect, Clone, Debug, Copy)]
pub enum ChestItem {
    Gold(u32),
    Item(InventoryItem),
}

pub fn default_random_chest() -> (Actor, Life) {
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

    chest_actor(chest_type, item)
}

pub fn chest_actor(chest_type: ChestType, chest_item: ChestItem) -> (Actor, Life) {
    (
        Actor {
            actor_group: ActorGroup::Entity,
            extra: ActorExtra::Chest {
                chest_type,
                chest_item,
            },
            ..default()
        },
        Life::new(30),
    )
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_chest(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    actor: Actor,
    life: Life,
    chest_type: ChestType,
) -> Entity {
    commands
        .spawn((
            Name::new("chest"),
            StateScoped(GameState::InGame),
            actor,
            life,
            Chest,
            Burnable {
                life: 60 * 20 + rand::random::<u32>() % 30,
            },
            Transform::from_translation(position.extend(0.0)),
            GlobalTransform::default(),
            Visibility::default(),
            Falling,
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping::default(),
                match chest_type {
                    ChestType::Jar(_) => Collider::ball(6.0),
                    _ => Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
                },
                *ENTITY_GROUPS,
                ExternalImpulse::default(),
            ),
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
    query: Query<(Entity, &Life, &Transform, &Actor, &Burnable), With<Chest>>,
    assets: Res<GameAssets>,
    ron: Res<Assets<GameConstants>>,
    mut writer: EventWriter<SEEvent>,
    mut explosion: EventWriter<SpawnExplosion>,
) {
    let constants = ron.get(assets.spells.id()).unwrap();

    for (entity, breakabke, transform, actor, burnable) in query.iter() {
        if breakabke.life <= 0 || burnable.life <= 0 {
            let position = transform.translation.truncate();
            let ActorExtra::Chest {
                chest_type,
                chest_item,
            } = actor.extra
            else {
                panic!("ActorExtra::Chest is expected");
            };

            commands.entity(entity).despawn_recursive();
            
            writer.send(SEEvent::pos(
                match chest_type {
                    ChestType::Chest => SE::Break,
                    ChestType::Crate => SE::Break,
                    ChestType::Barrel => SE::Break,
                    ChestType::BarrelBomb => SE::Break,
                    ChestType::Jar(_) => SE::Glass,
                },
                position,
            ));

            match chest_item {
                ChestItem::Gold(gold) => {
                    for _ in 0..gold {
                        spawn_gold(&mut commands, &assets, position);
                    }
                }
                ChestItem::Item(item) => {
                    spawn_dropped_item(&mut commands, &assets, &constants, position, item);
                }
            }

            match chest_type {
                ChestType::Crate => {
                    for i in 0..4 {
                        spawn_jar_piece(
                            &mut commands,
                            &assets,
                            position,
                            "crate",
                            JarColor::Red,
                            i,
                        );
                    }
                }
                ChestType::Barrel => {
                    for i in 0..4 {
                        spawn_jar_piece(
                            &mut commands,
                            &assets,
                            position,
                            "barrel",
                            JarColor::Red,
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
                        spawn_jar_piece(&mut commands, &assets, position, "jar", color, i);
                    }
                }
                _ => {}
            }
        }
    }
}

fn spawn_jar_piece(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    type_name: &str,
    color: JarColor,
    index: u32,
) {
    spawn_broken_piece(
        commands,
        assets,
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
