use super::actor::Actor;
use super::EntityDepth;
use crate::asset::GameAssets;
use crate::constant::*;
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

pub const WITCH_COLLIDER_RADIUS: f32 = 5.0;

pub fn spawn_witch<T: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    angle: f32,
    uuid: Uuid,
    name: Option<String>,
    life: i32,
    max_life: i32,
    res: &Res<LifeBarResource>,
    controller: T,
    life_bar: bool,
    intensity: f32,
) {
    let mut entity = commands.spawn((
        Name::new("witch"),
        StateScoped(GameState::InGame),
        Actor {
            uuid,
            cooltime: 0,
            life,
            max_life,
            latest_damage: 0,
            pointer: Vec2::from_angle(angle),
            intensity,
        },
        controller,
        EntityDepth,
        AsepriteAnimationBundle {
            aseprite: assets.player.clone(),
            transform: Transform::from_translation(position.extend(1.0)),
            animation: Animation::default().with_tag("idle").with_speed(0.2),
            sprite: Sprite {
                // flip_x: true,
                // ここもanchorは効かないことに注意。Aseprite側のpivotで設定
                // anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, 1.0)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::ball(WITCH_COLLIDER_RADIUS),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 6.0,
            angular_damping: 1.0,
        },
        ExternalForce::default(),
        ExternalImpulse::default(),
        CollisionGroups::new(
            ACTOR_GROUP,
            ENTITY_GROUP | ACTOR_GROUP | WALL_GROUP | BULLET_GROUP,
        ),
    ));

    entity.with_children(move |spawn_children| {
        // リモートプレイヤーの名前
        // 自分のプレイヤーキャラクターは名前を表示しません
        if let Some(name) = name {
            let mut sections = Vec::new();
            sections.push(TextSection {
                value: name,
                style: TextStyle {
                    color: Color::hsla(120.0, 1.0, 0.5, 0.3),
                    font_size: 10.0,
                    ..default()
                },
            });
            spawn_children.spawn(Text2dBundle {
                text: Text {
                    sections,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 20.0, 100.0),
                ..default()
            });
        }

        if life_bar {
            spawn_life_bar(spawn_children, &res);
        }
    });
}

pub struct WitchPlugin;

impl Plugin for WitchPlugin {
    fn build(&self, _app: &mut App) {}
}
