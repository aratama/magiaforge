use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::gold::spawn_gold;
use crate::entity::life::Life;
use crate::entity::life::LifeBeingSprite;
use crate::entity::piece::spawn_broken_piece;
use crate::entity::EntityDepth;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;

const ENTITY_WIDTH: f32 = 8.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum JarColor {
    #[default]
    Red,
    Blue,
    Green,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default, strum::EnumIter)]
pub enum ChestType {
    #[default]
    Chest,
    Crate,
    Barrel,
    Jar(JarColor),
}

pub const CHEST_OR_BARREL: [ChestType; 11] = [
    ChestType::Crate,
    ChestType::Crate,
    ChestType::Crate,
    ChestType::Crate,
    ChestType::Barrel,
    ChestType::Barrel,
    ChestType::Barrel,
    ChestType::Barrel,
    ChestType::Jar(JarColor::Red),
    ChestType::Jar(JarColor::Blue),
    ChestType::Jar(JarColor::Green),
];

#[derive(Default, Component, Reflect)]
struct Chest {
    pub chest_type: ChestType,
    pub golds: i32,
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_chest(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    x: f32,
    y: f32,
    chest_type: ChestType,
) {
    let tx = x + ENTITY_WIDTH - TILE_SIZE / 2.0;
    let ty = y - ENTITY_HEIGHT + TILE_SIZE / 2.0;
    let life = match chest_type {
        ChestType::Chest => 30,
        ChestType::Crate => 30,
        ChestType::Barrel => 20,
        ChestType::Jar(_) => 1,
    };
    commands
        .spawn((
            Name::new("chest"),
            StateScoped(GameState::InGame),
            Life::new(life),
            Chest {
                chest_type,
                golds: match chest_type {
                    ChestType::Chest => 10,
                    ChestType::Crate => 1,
                    ChestType::Barrel => 1,
                    ChestType::Jar(_) => 1,
                },
            },
            EntityDepth,
            Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            GlobalTransform::default(),
            Visibility::default(),
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
                CollisionGroups::new(
                    ENTITY_GROUP,
                    PIECE_GROUP
                        | ENTITY_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | ENEMY_GROUP
                        | ENEMY_BULLET_GROUP
                        | WALL_GROUP
                        | RABBIT_GROUP
                        | DROPPED_ITEM_GROUP,
                ),
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
    query: Query<(Entity, &Life, &Transform, &Chest)>,
    assets: Res<GameAssets>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform, chest) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(
                match chest.chest_type {
                    ChestType::Chest => SE::Break,
                    ChestType::Crate => SE::Break,
                    ChestType::Barrel => SE::Break,
                    ChestType::Jar(_) => SE::Glass,
                },
                position,
            ));
            for _ in 0..chest.golds {
                spawn_gold(
                    &mut commands,
                    &assets,
                    transform.translation.x,
                    transform.translation.y,
                );
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
        app.add_systems(
            FixedUpdate,
            break_chest
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Chest>();
    }
}
