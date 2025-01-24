use super::LifeBeingSprite;
use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorGroup;
use crate::actor::ActorSpriteGroup;
use crate::collision::ENTITY_GROUPS;
use crate::component::counter::CounterAnimated;
use crate::component::falling::Falling;
use crate::component::point_light::WithPointLight;
use crate::entity::piece::spawn_broken_piece;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct StoneLantern;

pub fn default_lantern() -> Actor {
    Actor {
        extra: ActorExtra::Lantern,
        actor_group: ActorGroup::Entity,
        life: 50,
        max_life: 50,
        ..default()
    }
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_stone_lantern(
    commands: &mut Commands,
    registry: &Registry,
    position: Vec2,
    actor: Actor,
) -> Entity {
    commands
        .spawn((
            Name::new("stone_lantern"),
            StateScoped(GameState::InGame),
            actor,
            StoneLantern,
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            Falling,
            (
                RigidBody::Dynamic,
                Damping::default(),
                LockedAxes::ROTATION_LOCKED,
                Collider::cuboid(8.0, 8.0),
                ColliderMassProperties::Density(10.0),
                *ENTITY_GROUPS,
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
            ),
        ))
        .with_children(|parent| {
            parent.spawn(ActorSpriteGroup).with_child((
                LifeBeingSprite,
                Sprite {
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                    ..default()
                },
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: registry.assets.stone_lantern.clone(),
                    ..default()
                },
                AnimationState {
                    current_frame: rand::random::<u16>() % 4,
                    ..default()
                },
            ));
        })
        .id()
}

fn break_stone_lantern(
    mut commands: Commands,
    registry: Registry,
    query: Query<(Entity, &Actor, &Transform), With<StoneLantern>>,
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
                    &registry,
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
            break_stone_lantern.in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<StoneLantern>();
    }
}
