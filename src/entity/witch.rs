use super::actor::{ActorGroup, ActorState};
use super::EntityChildrenAutoDepth;
use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::constant::*;
use crate::controller::player::{Equipment, Player};
use crate::entity::actor::{Actor, ActorFireState};
use crate::entity::life::{Life, LifeBeingSprite};
use crate::hud::life_bar::{spawn_life_bar, LifeBarResource};
use crate::inventory::Inventory;
use crate::player_state::PlayerState;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::audio::Volume;
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
    golds: i32,
    wands: [Option<Wand>; 4],
    inventory: Inventory,
    equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
    controller: T,
    actor_group: ActorGroup,
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
            current_wand: 0,
            effects: default(),
            actor_group,
            golds,
            wands,
            inventory,
            equipments,
        },
        ActorState::default(),
        Witch,
        controller,
        Life {
            life,
            max_life,
            amplitude: 0.0,
        },
        // 足音
        // footsteps.rsで音量を調整
        AudioPlayer::new(assets.taiikukan.clone()),
        PlaybackSettings {
            volume: Volume::new(0.0),
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
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
                match actor_group {
                    ActorGroup::Player => WITCH_GROUP,
                    ActorGroup::Enemy => ENEMY_GROUP,
                },
                ENTITY_GROUP
                    | WALL_GROUP
                    | WITCH_GROUP
                    | WITCH_BULLET_GROUP
                    | ENEMY_GROUP
                    | ENEMY_BULLET_GROUP
                    | MAGIC_CIRCLE_GROUP
                    | SENSOR_GROUP,
            ),
        ),
    ));

    entity.with_children(move |spawn_children| {
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
            EntityChildrenAutoDepth { offset: 0.0 },
        ));

        spawn_children.spawn((
            WitchWandSprite,
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "wand_cypress".into(),
            },
            EntityChildrenAutoDepth { offset: -0.001 },
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

#[derive(Component)]
struct EnemyWitchController;

pub fn spawn_enemy_witch(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_res: &Res<LifeBarResource>,
    position: Vec2,
) {
    let player = PlayerState::from_config(&GameConfig::default());

    spawn_witch(
        commands,
        &assets,
        position,
        0.0,
        Uuid::new_v4(),
        None,
        200,
        200,
        &life_bar_res,
        false,
        3.0,
        10,
        player.wands,
        player.inventory,
        player.equipments,
        EnemyWitchController,
        ActorGroup::Enemy,
    );
}

fn update_enemy_witch_controller(
    mut query: Query<(&mut Actor, &Transform), With<EnemyWitchController>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut actor, witch_transform) in query.iter_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            if player_transform
                .translation
                .truncate()
                .distance(witch_transform.translation.truncate())
                < 128.0
            {
                actor.fire_state = ActorFireState::Fire;
            } else {
                actor.fire_state = ActorFireState::Idle;
            }
        } else {
            actor.fire_state = ActorFireState::Idle;
        }
    }
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

fn update_wand(
    actor_query: Query<&Actor>,
    mut query: Query<
        (&Parent, &mut Transform, &mut AseSpriteSlice),
        (With<WitchWandSprite>, Without<Actor>),
    >,
    assets: Res<GameAssets>,
) {
    for (parent, mut transform, mut slice) in query.iter_mut() {
        if let Ok(actor) = actor_query.get(parent.get()) {
            let direction = actor.pointer;
            let angle = direction.to_angle();
            let pi = std::f32::consts::PI;
            transform.rotation = Quat::from_rotation_z(angle);
            transform.translation.x = if pi * 0.25 < angle && angle < pi * 0.75 {
                4.0
            } else if angle < pi * -0.25 && pi * -0.75 < angle {
                -4.0
            } else {
                0.0
            };

            if let Some(wand) = &actor.wands[actor.current_wand] {
                *slice = AseSpriteSlice {
                    name: wand.wand_type.to_props().slice.to_string(),
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
            (update_witch_animation, update_wand).run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            FixedUpdate,
            update_enemy_witch_controller
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
