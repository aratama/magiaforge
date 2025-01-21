use crate::asset::GameAssets;
use crate::collision::*;
use crate::component::entity_depth::EntityDepth;
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

pub const JARS: [ChestType; 3] = [
    ChestType::Jar(JarColor::Red),
    ChestType::Jar(JarColor::Blue),
    ChestType::Jar(JarColor::Green),
];

#[derive(Component, Reflect, Debug)]
struct Chest {
    pub chest_type: ChestType,
    pub golds: ChestItem,
}

#[derive(Reflect, Clone, Debug, Copy)]
pub enum ChestItem {
    Gold(u32),
    Item(InventoryItem),
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_chest(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    chest_type: ChestType,
    item: ChestItem,
) {
    let life = match chest_type {
        ChestType::Chest => 30,
        ChestType::Crate => 30,
        ChestType::Barrel => 20,
        ChestType::BarrelBomb => 20,
        ChestType::Jar(_) => 1,
    };
    commands
        .spawn((
            Name::new("chest"),
            StateScoped(GameState::InGame),
            Life::new(life),
            Chest {
                chest_type,
                golds: item,
            },
            Burnable {
                life: 60 * 20 + rand::random::<u32>() % 30,
            },
            EntityDepth::new(),
            Transform::from_translation(position.extend(0.0)),
            GlobalTransform::default(),
            Visibility::default(),
            Falling,
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 10.0,
                    angular_damping: 0.0,
                },
                match chest_type {
                    ChestType::Jar(_) => Collider::ball(6.0),
                    _ => Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
                },
                *ENTITY_GROUPS,
                ExternalImpulse::default(),
            ),
        ))
        .with_children(move |parent| {
            parent.spawn((
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
        });
}

fn break_chest(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform, &Chest, &Burnable)>,
    assets: Res<GameAssets>,
    ron: Res<Assets<GameConstants>>,
    mut writer: EventWriter<SEEvent>,
    mut explosion: EventWriter<SpawnExplosion>,
) {
    let constants = ron.get(assets.spells.id()).unwrap();

    for (entity, breakabke, transform, chest, burnable) in query.iter() {
        if breakabke.life <= 0 || burnable.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
            writer.send(SEEvent::pos(
                match chest.chest_type {
                    ChestType::Chest => SE::Break,
                    ChestType::Crate => SE::Break,
                    ChestType::Barrel => SE::Break,
                    ChestType::BarrelBomb => SE::Break,
                    ChestType::Jar(_) => SE::Glass,
                },
                position,
            ));

            match chest.golds {
                ChestItem::Gold(gold) => {
                    for _ in 0..gold {
                        spawn_gold(&mut commands, &assets, position);
                    }
                }
                ChestItem::Item(item) => {
                    spawn_dropped_item(&mut commands, &assets, &constants, position, item);
                }
            }

            match chest.chest_type {
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
        app.add_systems(FixedUpdate, break_chest.in_set(FixedUpdateGameActiveSet));
        app.register_type::<Chest>();
    }
}
