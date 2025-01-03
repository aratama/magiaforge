use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::ChildEntityDepth;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::*;
use crate::controller::player::Equipment;
use crate::controller::training_dummy::TraningDummyController;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorState;
use crate::entity::bullet::HomingTarget;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
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
    point_light_radius: f32,
    golds: u32,
    wands: [Wand; 4],
    inventory: Inventory,
    equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
    controller: T,
    actor_group: ActorGroup,
    current_wand: usize,
) -> Entity {
    let mut entity = commands.spawn((
        Name::new("witch"),
        StateScoped(GameState::InGame),
        Actor {
            uuid,
            pointer: Vec2::from_angle(angle),
            point_light_radius,
            radius: WITCH_COLLIDER_RADIUS,
            move_direction: Vec2::ZERO,
            move_force: PLAYER_MOVE_FORCE,
            fire_state: ActorFireState::Idle,
            fire_state_secondary: ActorFireState::Idle,
            current_wand,
            effects: default(),
            actor_group,
            golds,
            wands,
            inventory,
            equipments,
            state: ActorState::default(),
            wait: 0,
        },
        Witch,
        controller,
        Life {
            life,
            max_life,
            amplitude: 0.0,
            fire_damage_wait: 0,
        },
        HomingTarget,
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
        Visibility::default(),
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
            actor_group.to_groups(),
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
            CounterAnimated,
            AseSpriteAnimation {
                aseprite: assets.witch.clone(),
                animation: Animation::default().with_tag("idle_r"),
            },
            ChildEntityDepth { offset: 0.0 },
        ));

        spawn_children.spawn((
            WitchWandSprite,
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "wand_cypress".into(),
            },
            ChildEntityDepth { offset: -0.001 },
        ));

        // リモートプレイヤーの名前
        // 自分のプレイヤーキャラクターは名前を表示しません
        if let Some(name) = name_plate {
            spawn_children.spawn((
                Transform::from_xyz(0.0, 20.0, 100.0),
                Text2d::new(name),
                TextColor(Color::hsla(120.0, 1.0, 0.5, 0.3)),
                TextFont {
                    font: assets.noto_sans_jp.clone(),
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

#[allow(dead_code)]
pub fn spawn_enemy_witch(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    life_bar_res: &Res<LifeBarResource>,
    position: Vec2,
) {
    let player = PlayerState::default();

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
        TraningDummyController {
            home: position,
            fire: false,
        },
        ActorGroup::Enemy,
        0,
    );
}

fn update_witch_animation(
    witch_query: Query<&Actor, With<Witch>>,
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
        if let Ok(actor) = witch_query.get(parent.get()) {
            let angle = actor.pointer.to_angle();
            let pi = std::f32::consts::PI;
            match actor.state {
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
                ActorState::GettingUp => {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("get_up".to_string()) {
                        animation.animation.tag = Some("get_up".to_string());
                        animation_state.current_frame = 26;
                    }
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
        (&Parent, &mut Transform, &mut Visibility),
        (With<WitchWandSprite>, Without<Actor>),
    >,
) {
    for (parent, mut transform, mut visibility) in query.iter_mut() {
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

            *visibility = match actor.state {
                ActorState::GettingUp => Visibility::Hidden,
                _ => Visibility::Visible,
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
