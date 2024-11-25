use super::actor::ActorState;
use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::breakable::{Breakable, BreakableSprite};
use crate::entity::EntityDepth;
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::interaction_sensor::spawn_interaction_sensor;
use crate::states::GameState;
use crate::wand::Wand;
use crate::wand_props::wand_to_props;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;
use uuid::Uuid;

pub const WITCH_COLLIDER_RADIUS: f32 = 5.0;

pub const PLAYER_MOVE_FORCE: f32 = 40000.0;

#[derive(Default, Component, Reflect)]
pub struct WitchWandSprite;

#[derive(Component)]
pub struct Witch;

pub fn spawn_witch<T: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    angle: f32,
    uuid: Uuid,
    name_plate: Option<String>,
    life: i32,
    max_life: i32,
    res: &Res<LifeBarResource>,
    life_bar: bool,
    intensity: f32,
    interaction: bool,
    wands: [Option<Wand>; 4],
    controller: T,
) -> Entity {
    let mut entity = commands.spawn((
        Name::new("witch"),
        StateScoped(GameState::InGame),
        Actor {
            uuid,
            spell_delay: 0,
            mana: 1000,
            max_mana: 1000,
            life,
            max_life,
            pointer: Vec2::from_angle(angle),
            intensity,
            move_direction: Vec2::ZERO,
            move_force: PLAYER_MOVE_FORCE,
            fire_state: ActorFireState::Idle,
            group: WITCH_GROUP,
            filter: ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | ENEMY_GROUP,
            current_wand: 0,
            effects: default(),
            wands,
        },
        ActorState::default(),
        Witch,
        controller,
        EntityDepth,
        Breakable {
            life: 0,
            amplitude: 0.0,
        },
        Transform::from_translation(position.extend(1.0)),
        GlobalTransform::default(),
        InheritedVisibility::default(),
        (
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
                WITCH_GROUP,
                ENTITY_GROUP
                    | WALL_GROUP
                    | WITCH_GROUP
                    | WITCH_BULLET_GROUP
                    | ENEMY_GROUP
                    | ENEMY_BULLET_GROUP
                    | MAGIC_CIRCLE_GROUP,
            ),
        ),
    ));

    entity.with_children(move |mut spawn_children| {
        if interaction {
            spawn_interaction_sensor(&mut spawn_children);
        }

        spawn_children.spawn((
            BreakableSprite,
            AsepriteAnimationBundle {
                aseprite: assets.player.clone(),
                animation: Animation::default().with_tag("idle"),
                sprite: Sprite {
                    // flip_x: true,
                    // ここもanchorは効かないことに注意。Aseprite側のpivotで設定
                    // anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, 1.0)),
                    ..default()
                },
                ..default()
            },
        ));

        spawn_children.spawn((
            WitchWandSprite,
            AsepriteSliceBundle {
                aseprite: assets.atlas.clone(),
                slice: "wand_cypress".into(),
                ..default()
            },
        ));

        // リモートプレイヤーの名前
        // 自分のプレイヤーキャラクターは名前を表示しません
        if let Some(name) = name_plate {
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

    return entity.id();
}

fn update_witch_animation(
    witch_query: Query<&ActorState, Changed<ActorState>>,
    mut witch_animation_query: Query<(&Parent, &mut Animation)>,
) {
    for (parent, mut animation) in witch_animation_query.iter_mut() {
        if let Ok(state) = witch_query.get(**parent) {
            // アニメーションを切り替えても現在のフレーム位置が巻き戻らない？
            // https://github.com/Lommix/bevy_aseprite_ultra/issues/14
            animation.play(
                match state {
                    ActorState::Idle => "idle",
                    ActorState::Walk => "run",
                },
                AnimationRepeat::Loop,
            );
        }
    }
}

fn update_wand(
    actor_query: Query<&Actor>,
    mut query: Query<
        (&Parent, &mut Transform, &mut Sprite, &mut AsepriteSlice),
        With<WitchWandSprite>,
    >,
) {
    for (parent, mut transform, mut sprite, mut slice) in query.iter_mut() {
        if let Ok(actor) = actor_query.get(parent.get()) {
            let direction = actor.pointer;
            let angle = direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle);
            transform.translation = Vec3::new(0.0, 0.0, -0.01);
            sprite.flip_y = if angle.abs() < PI / 2.0 { false } else { true };

            if let Some(wand) = &actor.wands[actor.current_wand] {
                let props = wand_to_props(wand.wand_type);
                *slice = AsepriteSlice::new(props.slice);
            } else {
                *slice = AsepriteSlice::new("empty");
            }
        }
    }
}

pub struct WitchPlugin;

impl Plugin for WitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_witch_animation, update_wand).run_if(in_state(GameState::InGame)),
        );
    }
}
