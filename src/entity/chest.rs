use crate::{asset::GameAssets, constant::*, se::SEEvent, states::GameState};
use crate::{
    entity::{
        gold::spawn_gold,
        life::{Life, LifeBeingSprite},
        EntityDepth,
    },
    se::SE,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

const ENTITY_WIDTH: f32 = 8.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Clone, Copy, PartialEq, Eq, Reflect, Default, strum::EnumIter)]
pub enum ChestType {
    #[default]
    Chest,
    Crate,
    Barrel,
}

pub const CHEST_OR_BARREL: [ChestType; 2] = [ChestType::Crate, ChestType::Barrel];

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
    commands
        .spawn((
            Name::new("chest"),
            StateScoped(GameState::InGame),
            Life {
                life: 30,
                max_life: 30,
                amplitude: 0.0,
            },
            Chest {
                chest_type,
                golds: match chest_type {
                    ChestType::Chest => 10,
                    ChestType::Crate => 1,
                    ChestType::Barrel => 1,
                },
            },
            EntityDepth,
            Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 10.0,
                    angular_damping: 0.0,
                },
                Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
                CollisionGroups::new(
                    ENTITY_GROUP,
                    ENTITY_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | ENEMY_GROUP
                        | ENEMY_BULLET_GROUP
                        | WALL_GROUP,
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
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(SE::Break, transform.translation.truncate()));
            for _ in 0..chest.golds {
                spawn_gold(
                    &mut commands,
                    &assets,
                    transform.translation.x,
                    transform.translation.y,
                );
            }
        }
    }
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
