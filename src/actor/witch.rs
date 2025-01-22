use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::actor::ActorSpriteGroup;
use crate::actor::ActorState;
use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::life::Life;
use crate::component::vertical::Vertical;
use crate::constant::*;
use crate::entity::bullet::HomingTarget;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
use crate::level::entities::SpawnWitchType;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use core::f32;
use std::collections::HashSet;

pub const WITCH_COLLIDER_RADIUS: f32 = 5.0;

#[derive(Default, Component, Reflect)]
pub struct ActorAnimationSprite;

#[derive(Default, Component, Reflect)]
pub struct WitchWandSprite;

#[derive(Component)]
pub struct Witch;

pub fn default_witch() -> (Actor, Life) {
    (
        Actor {
            extra: ActorExtra::Witch {
                witch_type: SpawnWitchType::Dummy,
                getting_up: false,
                name: "default".to_string(),
                discovered_spells: HashSet::new(),
            },
            ..default()
        },
        Life::new(60),
    )
}

pub fn spawn_witch(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    name_plate: Option<String>,
    res: &Res<LifeBarResource>,
    life_bar: bool,
    actor: Actor,
    life: Life,
) -> Entity {
    let actor_group = actor.actor_group;
    let mut entity = commands.spawn((
        Name::new("witch"),
        StateScoped(GameState::InGame),
        actor,
        Witch,
        life,
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
        Visibility::default(),
        (
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(WITCH_COLLIDER_RADIUS),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
            actor_group.to_groups(0.0, 0),
        ),
    ));

    entity.with_children(move |spawn_children| {
        spawn_children.spawn((
            Transform::from_translation(Vec2::new(0.0, -4.0).extend(SHADOW_LAYER_Z)),
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "entity_shadow".into(),
            },
        ));

        spawn_children
            .spawn(ActorSpriteGroup)
            .with_child((
                ActorAnimationSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.witch.clone(),
                    animation: "idle_r".into(),
                },
            ))
            .with_child((
                WitchWandSprite,
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "wand_cypress".into(),
                },
                Transform::from_xyz(0.0, 4.0, -0.0001),
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

fn update_witch_animation(
    witch_query: Query<&Actor, With<Witch>>,
    witch_sprite_group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut witch_animation_query: Query<
        (&Parent, &mut Sprite, &mut AseSpriteAnimation),
        With<ActorAnimationSprite>,
    >,
) {
    for (parent, mut sprite, mut animation) in witch_animation_query.iter_mut() {
        // 魔女エンティティは３階層になっていることに注意
        // 魔女スプライトから Actor を辿るのには２回 get が必要です
        let sprite_group_parent = witch_sprite_group_query.get(parent.get()).unwrap();
        let actor = witch_query.get(sprite_group_parent.get()).unwrap();

        if 0 < actor.drowning {
            if animation.animation.tag != Some("drown".to_string()) {
                animation.animation.tag = Some("drown".to_string());
            }
            continue;
        }

        if 0 < actor.staggered {
            if animation.animation.tag != Some("staggered".to_string()) {
                animation.animation.tag = Some("staggered".to_string());
            }
            continue;
        }

        let angle = actor.pointer.to_angle();
        let pi = std::f32::consts::PI;
        match actor.state {
            ActorState::Idle => {
                if angle < pi * -0.75 || pi * 0.75 < angle {
                    sprite.flip_x = true;
                    if animation.animation.tag != Some("idle_r".to_string()) {
                        animation.animation.tag = Some("idle_r".to_string());
                    }
                } else if pi * 0.25 < angle && angle < pi * 0.75 {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("idle_u".to_string()) {
                        animation.animation.tag = Some("idle_u".to_string());
                    }
                } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("idle_d".to_string()) {
                        animation.animation.tag = Some("idle_d".to_string());
                    }
                } else {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("idle_r".to_string()) {
                        animation.animation.tag = Some("idle_r".to_string());
                    }
                };
            }
            ActorState::Run => {
                if angle < pi * -0.75 || pi * 0.75 < angle {
                    sprite.flip_x = true;
                    if animation.animation.tag != Some("run_r".to_string()) {
                        animation.animation.tag = Some("run_r".to_string());
                    }
                } else if pi * 0.25 < angle && angle < pi * 0.75 {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("run_u".to_string()) {
                        animation.animation.tag = Some("run_u".to_string());
                    }
                } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("run_d".to_string()) {
                        animation.animation.tag = Some("run_d".to_string());
                    }
                } else {
                    sprite.flip_x = false;
                    if animation.animation.tag != Some("run_r".to_string()) {
                        animation.animation.tag = Some("run_r".to_string());
                    }
                };
            }
            ActorState::GettingUp => {
                sprite.flip_x = false;
                if animation.animation.tag != Some("get_up".to_string()) {
                    animation.animation.tag = Some("get_up".to_string());
                }
            }
        };
    }
}

fn update_wand(
    actor_query: Query<&Actor>,
    witch_sprite_group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut query: Query<
        (&Parent, &mut Transform, &mut Visibility),
        (With<WitchWandSprite>, Without<Actor>),
    >,
) {
    for (parent, mut transform, mut visibility) in query.iter_mut() {
        let group_parent = witch_sprite_group_query.get(parent.get()).unwrap();
        let actor = actor_query.get(group_parent.get()).unwrap();
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

        *visibility = if 0 < actor.drowning || 0 < actor.staggered {
            Visibility::Hidden
        } else {
            match actor.state {
                ActorState::GettingUp => Visibility::Hidden,
                _ => Visibility::Visible,
            }
        };
    }
}

fn land_se(witch_query: Query<(&Vertical, &Transform), With<Witch>>, mut se: EventWriter<SEEvent>) {
    for (vertical, transform) in witch_query.iter() {
        if vertical.just_landed {
            let position = transform.translation.truncate();
            se.send(SEEvent::pos(SE::Chakuchi, position));
        }
    }
}

pub struct WitchPlugin;

impl Plugin for WitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_witch_animation, update_wand, land_se).in_set(FixedUpdateGameActiveSet),
        );
    }
}
