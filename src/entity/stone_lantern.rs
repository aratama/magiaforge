use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::component::point_light::WithPointLight;
use crate::constant::*;
use crate::entity::piece::spawn_broken_piece;
use crate::entity::EntityDepth;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct StoneLantern;

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_stone_lantern(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            Name::new("stone_lantern"),
            StateScoped(GameState::InGame),
            Life::new(50),
            StoneLantern,
            EntityDepth::new(),
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
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
                PIECE_GROUP
                    | ENTITY_GROUP
                    | NEUTRAL_GROUP
                    | WITCH_GROUP
                    | WITCH_BULLET_GROUP
                    | ENEMY_GROUP
                    | ENEMY_BULLET_GROUP
                    | WALL_GROUP
                    | RABBIT_GROUP
                    | DROPPED_ITEM_GROUP,
            ),
            ExternalImpulse::default(),
            WithPointLight {
                radius: 64.0,
                intensity: 1.0,
                falloff: 10.0,
                color: Color::hsl(42.0, 1.0, 0.71),
                animation_offset: rand::random::<u32>() % 1000,
                speed: 0.43,
                amplitude: 0.1,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                LifeBeingSprite,
                Sprite {
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                    ..default()
                },
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.stone_lantern.clone(),
                    ..default()
                },
            ));
        });
}

fn break_stone_lantern(
    mut commands: Commands,
    assets: Res<GameAssets>,
    query: Query<(Entity, &Life, &Transform), With<StoneLantern>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(SE::Break, position));

            for i in 0..4 {
                spawn_broken_piece(
                    &mut commands,
                    &assets,
                    position,
                    &format!("stone_lantern_piece_{}", i),
                );
            }
        }
    }
}

pub struct StoneLanternPlugin;

impl Plugin for StoneLanternPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            break_stone_lantern
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<StoneLantern>();
    }
}
