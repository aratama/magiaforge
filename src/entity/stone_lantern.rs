use crate::{asset::GameAssets, constant::*, se::SECommand, states::GameState};
use crate::{
    entity::{
        life::{Life, LifeBeingSprite},
        EntityDepth,
    },
    se::SE,
};
use bevy::{core::FrameCount, prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct StoneLantern;

#[derive(Component, Reflect)]
struct LanternParent(Entity);

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_stone_lantern(commands: &mut Commands, assets: &Res<GameAssets>, x: f32, y: f32) {
    let tx = x;
    let ty = y;

    let entity = commands.spawn_empty().id();

    commands
        .entity(entity)
        .insert((
            Name::new("stone_lantern"),
            StateScoped(GameState::InGame),
            Life {
                life: 50,
                max_life: 50,
                amplitude: 0.0,
            },
            StoneLantern,
            EntityDepth,
            Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            (
                RigidBody::Dynamic,
                Damping {
                    linear_damping: 80.0,
                    angular_damping: 0.0,
                },
                LockedAxes::ROTATION_LOCKED,
                Collider::cuboid(8.0, 8.0),
                ColliderMassProperties::Density(10.0),
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
        .with_children(|parent| {
            parent.spawn((
                LifeBeingSprite,
                Sprite {
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                    ..default()
                },
                AseSpriteAnimation {
                    aseprite: assets.stone_lantern.clone(),
                    ..default()
                },
            ));
        });

    commands.spawn((
        Name::new("stone lantern light"),
        StateScoped(GameState::InGame),
        LanternParent(entity),
        Transform::from_translation(Vec3::new(tx, ty, 0.0)),
        PointLight2d {
            radius: 64.0,
            intensity: 1.0,
            falloff: 10.0,
            color: Color::hsl(42.0, 1.0, 0.71),
            ..default()
        },
    ));
}

fn update_lantern(
    mut commands: Commands,
    parent_query: Query<&Transform, With<StoneLantern>>,
    mut child_query: Query<
        (Entity, &LanternParent, &mut PointLight2d, &mut Transform),
        Without<StoneLantern>,
    >,
    frame_count: Res<FrameCount>,
) {
    for (entity, child, mut light, mut transform) in child_query.iter_mut() {
        light.intensity = 1.0 + ((frame_count.0 as f32 * 0.5).cos()) * 0.1;
        if let Ok(lantern_transform) = parent_query.get(child.0) {
            transform.translation = lantern_transform.translation;
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn break_stone_lantern(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform), With<StoneLantern>>,
    mut writer: EventWriter<SECommand>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            commands.entity(entity).despawn_recursive();
            writer.send(SECommand::pos(SE::Break, transform.translation.truncate()));
        }
    }
}

pub struct StoneLanternPlugin;

impl Plugin for StoneLanternPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_lantern.run_if(in_state(GameState::InGame)));
        app.add_systems(
            FixedUpdate,
            break_stone_lantern
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<StoneLantern>();
    }
}
