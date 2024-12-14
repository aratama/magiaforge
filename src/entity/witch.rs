use super::actor::{ActorGroup, ActorState};
use super::get_entity_z;
use crate::asset::GameAssets;
use crate::constant::*;
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::life::{Life, LifeBeingSprite};
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::interaction_sensor::spawn_interaction_sensor;
use crate::states::GameState;
use crate::wand::Wand;
use crate::wand_props::wand_to_props;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

pub const WITCH_COLLIDER_RADIUS: f32 = 5.0;

pub const PLAYER_MOVE_FORCE: f32 = 40000.0;

#[derive(Default, Component, Reflect)]
pub struct WitchWandSprite;

#[derive(Default, Component, Reflect)]
pub struct WitchAnimationSprite;

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

            pointer: Vec2::from_angle(angle),
            intensity,
            move_direction: Vec2::ZERO,
            move_force: PLAYER_MOVE_FORCE,
            fire_state: ActorFireState::Idle,
            group: WITCH_GROUP,
            filter: ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | ENEMY_GROUP,
            current_wand: 0,
            effects: default(),
            actor_group: ActorGroup::Player,
            wands,
        },
        ActorState::default(),
        Witch,
        controller,
        Life {
            life,
            max_life,
            amplitude: 0.0,
        },
        // XとYは親エンティティ側で設定しますが、
        // キャラクター画像とシャドウでz座標の計算が異なるため、
        // 親のzは常に0にします
        Transform::from_translation(position.extend(0.0)),
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
            Transform::from_translation(Vec2::new(0.0, -8.0).extend(SHADOW_LAYER_Z)),
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "entity_shadow".into(),
            },
        ));

        spawn_children.spawn((
            WitchAnimationSprite,
            LifeBeingSprite,
            Sprite {
                // flip_x: true,
                // ここもanchorは効かないことに注意。Aseprite側のpivotで設定
                // anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, 1.0)),
                ..default()
            },
            AseSpriteAnimation {
                aseprite: assets.player.clone(),
                animation: Animation::default().with_tag("idle_r"),
            },
        ));

        spawn_children.spawn((
            WitchWandSprite,
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "wand_cypress".into(),
            },
        ));

        // リモートプレイヤーの名前
        // 自分のプレイヤーキャラクターは名前を表示しません
        if let Some(name) = name_plate {
            spawn_children.spawn((
                Transform::from_xyz(0.0, 20.0, 100.0),
                Text2d::new(name),
                TextColor(Color::hsla(120.0, 1.0, 0.5, 0.3)),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
            ));
        }

        if life_bar {
            spawn_life_bar(spawn_children, &res);
        }
    });

    return entity.id();
}

fn update_witch_animation(
    witch_query: Query<(&Actor, &ActorState), With<Witch>>,
    mut witch_animation_query: Query<
        (
            &Parent,
            &mut Sprite,
            &mut AseSpriteAnimation,
            &mut AnimationState,
        ),
        With<WitchAnimationSprite>,
    >,
) {
    for (parent, mut sprite, mut animation, mut animation_state) in witch_animation_query.iter_mut()
    {
        if let Ok((actor, state)) = witch_query.get(**parent) {
            let angle = actor.pointer.to_angle();
            let pi = std::f32::consts::PI;
            match state {
                ActorState::Idle => {
                    if angle < pi * -0.75 || pi * 0.75 < angle {
                        sprite.flip_x = true;
                        if animation.animation.tag != Some("idle_r".to_string()) {
                            animation.animation.tag = Some("idle_r".to_string());
                            animation_state.current_frame = 0;
                        }
                    } else if pi * 0.25 < angle && angle < pi * 0.75 {
                        sprite.flip_x = false;
                        if animation.animation.tag != Some("idle_u".to_string()) {
                            animation.animation.tag = Some("idle_u".to_string());
                            animation_state.current_frame = 6;
                        }
                    } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                        sprite.flip_x = false;
                        if animation.animation.tag != Some("idle_d".to_string()) {
                            animation.animation.tag = Some("idle_d".to_string());
                            animation_state.current_frame = 3;
                        }
                    } else {
                        sprite.flip_x = false;
                        if animation.animation.tag != Some("idle_r".to_string()) {
                            animation.animation.tag = Some("idle_r".to_string());
                            animation_state.current_frame = 0;
                        }
                    };
                }
                ActorState::Run => {
                    if angle < pi * -0.75 || pi * 0.75 < angle {
                        sprite.flip_x = true;
                        if animation.animation.tag != Some("run_r".to_string()) {
                            animation.animation.tag = Some("run_r".to_string());
                            animation_state.current_frame = 9;
                        }
                    } else if pi * 0.25 < angle && angle < pi * 0.75 {
                        sprite.flip_x = false;
                        if animation.animation.tag != Some("run_u".to_string()) {
                            animation.animation.tag = Some("run_u".to_string());
                            animation_state.current_frame = 13;
                        }
                    } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                        sprite.flip_x = false;
                        if animation.animation.tag != Some("run_d".to_string()) {
                            animation.animation.tag = Some("run_d".to_string());
                            animation_state.current_frame = 17;
                        }
                    } else {
                        sprite.flip_x = false;
                        if animation.animation.tag != Some("run_r".to_string()) {
                            animation.animation.tag = Some("run_r".to_string());
                            animation_state.current_frame = 9;
                        }
                    };
                }
            };
        } else {
            warn!("parent of witch not found");
        }
    }
}

/// 魔女スプライトのZを設定します
/// X,Y座標は親にあり、Z座標は親のY座標から計算されることに注意します
/// X,Y座標とZ座標を設定するコンポーネントがそれぞれ異なるため、EntityDepthは使えません
fn update_witch_z(
    witch_query: Query<&Transform, With<Witch>>,
    mut sprite_query: Query<
        (&Parent, &mut Transform),
        (With<WitchAnimationSprite>, Without<Witch>),
    >,
) {
    for (parent, mut transform) in sprite_query.iter_mut() {
        let witch_transform = witch_query.get(parent.get()).unwrap();
        transform.translation.z = get_entity_z(witch_transform.translation.y);
    }
}

fn update_wand(
    actor_query: Query<(&Actor, &Transform)>,
    mut query: Query<
        (&Parent, &mut Transform, &mut AseSpriteSlice),
        (With<WitchWandSprite>, Without<Actor>),
    >,
    assets: Res<GameAssets>,
) {
    for (parent, mut transform, mut slice) in query.iter_mut() {
        if let Ok((actor, actor_transform)) = actor_query.get(parent.get()) {
            let direction = actor.pointer;
            let angle = direction.to_angle();
            let pi = std::f32::consts::PI;
            transform.rotation = Quat::from_rotation_z(angle);
            transform.translation = Vec3::new(
                if pi * 0.25 < angle && angle < pi * 0.75 {
                    4.0
                } else if angle < pi * -0.25 && pi * -0.75 < angle {
                    -4.0
                } else {
                    0.0
                },
                0.0,
                // 杖と腕は身体の奥に描画するため、 -0.1 でずらしています
                get_entity_z(actor_transform.translation.y) - 0.1,
            );

            if let Some(wand) = &actor.wands[actor.current_wand] {
                let props = wand_to_props(wand.wand_type);
                *slice = AseSpriteSlice {
                    name: props.slice.to_string(),
                    aseprite: assets.atlas.clone(),
                };
            } else {
                *slice = AseSpriteSlice {
                    name: "empty".to_string(),
                    aseprite: assets.atlas.clone(),
                };
            }
        }
    }
}

pub struct WitchPlugin;

impl Plugin for WitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_witch_animation, update_wand, update_witch_z)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
