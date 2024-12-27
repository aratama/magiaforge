use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::counter::Counter;
use crate::entity::life::Life;
use crate::entity::life::LifeBeingSprite;
use crate::entity::EntityDepth;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use super::counter::CounterAnimated;
use super::explosion::SpawnExplosion;

#[derive(Default, Component, Reflect)]
struct Bomb;

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
    mut se: EventWriter<SEEvent>,
) {
    if !reader.is_empty() {
        se.send(SEEvent::new(SE::PickUp));
    }

    for SpawnBomb { position } in reader.read() {
        let aseprite = assets.bomb.clone();
        commands
            .spawn((
                Name::new("bomb"),
                StateScoped(GameState::InGame),
                Life::new(10),
                Bomb,
                Counter::new(),
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
                    CounterAnimated,
                    AseSpriteAnimation {
                        aseprite: aseprite.clone(),
                        animation: "default".into(), // TODO
                    },
                ));
            });
    }
}

fn explode_bomb(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Life, &Counter), With<Bomb>>,
    mut explosion_writer: EventWriter<SpawnExplosion>,
) {
    for (entity, transform, life, counter) in query.iter() {
        if life.life <= 0 || 180 <= counter.count {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            explosion_writer.send(SpawnExplosion {
                position,
                radius: 60.0,
                impulse: 100000.0,
                damage: 100,
            });
        }
    }
}

fn set_bomb_rotation(
    mut query: Query<(&Children, &Transform), With<Bomb>>,
    mut sprite_query: Query<&mut Transform, (With<AseSpriteAnimation>, Without<Bomb>)>, // TODO
) {
    for (children, transform) in query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut child) = sprite_query.get_mut(*child) {
                child.rotation = Quat::from_rotation_z(transform.translation.x * -0.1);
            }
        }
    }
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBomb>();
        app.add_systems(
            FixedUpdate,
            (spawn_bomb, explode_bomb, set_bomb_rotation)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Bomb>();
    }
}
