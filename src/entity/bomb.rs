use core::f32;

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
struct Bomb {
    pub lifetime: u32,
}

#[derive(Event)]
pub struct SpawnBomb {
    pub position: Vec2,
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_bomb(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut reader: EventReader<SpawnBomb>,
) {
    for SpawnBomb { position } in reader.read() {
        let mut atlas = assets.atlas.clone();
        let life = commands
            .spawn((
                Name::new("bomb"),
                StateScoped(GameState::InGame),
                Life::new(10),
                Bomb { lifetime: 300 },
                EntityDepth,
                Transform::from_translation(position.extend(0.0)),
                GlobalTransform::default(),
                Visibility::default(),
                (
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Damping {
                        linear_damping: 1.0,
                        angular_damping: 0.0,
                    },
                    Collider::ball(6.0),
                    CollisionGroups::new(
                        ENTITY_GROUP,
                        PIECE_GROUP
                            | ENTITY_GROUP
                            | WITCH_GROUP
                            | WITCH_BULLET_GROUP
                            | ENEMY_GROUP
                            | ENEMY_BULLET_GROUP
                            | WALL_GROUP
                            | RABBIT_GROUP,
                    ),
                    ExternalImpulse::default(),
                ),
            ))
            .with_children(move |parent| {
                parent.spawn((
                    LifeBeingSprite,
                    AseSpriteSlice {
                        aseprite: atlas.clone(),
                        name: "bomb_icon".into(), // TODO
                    },
                ));
            });
    }
}

fn timeout_bomb(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform, &Bomb)>,
    assets: Res<GameAssets>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform, chest) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn break_bomb(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Bomb)>,
    assets: Res<GameAssets>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, transform, mut bomb) in query.iter_mut() {
        if bomb.lifetime <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();

            commands.spawn((
                StateScoped(GameState::InGame),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "scorch_mark".into(),
                },
                Transform::from_translation(position.extend(0.0)),
            ));

            writer.send(SEEvent::pos(SE::Bakuhatsu, position));
        } else {
            bomb.lifetime -= 1;
        }
    }
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (spawn_bomb, break_bomb, timeout_bomb)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Bomb>();
    }
}
