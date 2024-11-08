use crate::{
    asset::GameAssets, command::GameCommand, controller::remote::RemoteMessage, states::GameState,
    world::CurrentLevel,
};
use bevy::prelude::*;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_rapier2d::prelude::Group;
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};
use rand::random;
use std::f32::consts::PI;
use uuid::Uuid;

use super::{
    breakable::BreakableSprite,
    bullet::{spawn_bullet, BulletType, BULLET_RADIUS, BULLET_SPAWNING_MARGIN},
    witch::WITCH_COLLIDER_RADIUS,
};

// 魔法の拡散
const BULLET_SCATTERING: f32 = 0.4;

/// 次の魔法を発射するまでの待機時間
/// この値は全アクター共通で、アクターのreload_speedが上昇すると再発射までの時間が短くなります
const BULLET_MAX_COOLTIME: i32 = 1000;

// 一度に発射する弾丸の数
const BULLETS_PER_FIRE: u32 = 1;

const BULLET_MANA_COST: i32 = 50;

/// ライフを持ち、弾丸のダメージの対象となるエンティティを表します
#[derive(Component)]
pub struct Actor {
    pub uuid: Uuid,

    /// 次の魔法を発射できるまでのクールタイム
    pub cooltime: i32,

    pub reload_speed: i32,

    pub mana: i32,

    pub max_mana: i32,

    /// 魔法弾の速度
    /// pixels_per_meter が 100.0 に設定されているので、
    /// 200は1フレームに2ピクセル移動する速度です
    pub bullet_speed: f32,

    pub bullet_lifetime: u32,

    pub life: i32,
    pub max_life: i32,

    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,

    pub intensity: f32,

    pub move_state: ActorMoveState,

    pub fire_state: ActorFireState,

    /// 弾丸の発射をリモートに通知するかどうか
    /// プレイヤーキャラクターはtrue、敵キャラクターはfalseにします
    pub online: bool,

    pub group: Group,

    pub filter: Group,

    pub bullet_type: BulletType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActorMoveState {
    Idle,
    Run,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActorFireState {
    Idle,

    /// Actorのpointerに向かって弾丸を発射します
    Fire,
}

fn update_sprite_flip(
    actor_query: Query<&Actor>,
    mut sprite_query: Query<(&Parent, &mut Sprite), With<BreakableSprite>>,
) {
    for (parent, mut sprite) in sprite_query.iter_mut() {
        if let Ok(actor) = actor_query.get(parent.get()) {
            // プレイヤーの向き
            let angle = actor.pointer.y.atan2(actor.pointer.x);
            if angle < -PI * 0.5 || PI * 0.5 < angle {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
    }
}

fn recovery_mana(mut actor_query: Query<(&mut Actor, &Transform), Without<Camera2d>>) {
    for (mut actor, _) in actor_query.iter_mut() {
        actor.mana = (actor.mana + 1).min(actor.max_mana);
    }
}

#[derive(Component)]
pub struct ActorLight {
    owner: Entity,
}

fn update_actor_light(
    mut commands: Commands,
    mut light_query: Query<(Entity, &ActorLight, &mut PointLight2d, &mut Transform)>,
    actor_query: Query<(Entity, &Actor, &Transform), Without<ActorLight>>,
) {
    for (actor_entity, actor, transform) in actor_query.iter() {
        if light_query
            .iter()
            .find(|(_, light, _, _)| light.owner == actor_entity)
            .is_none()
        {
            // SpriteBundle に PointLight2d を追加すると、画面外に出た時に Sprite が描画されなくなり、
            // ライトも描画されず不自然になるため、別で追加する
            // https://github.com/jgayfer/bevy_light_2d/issues/26
            commands.spawn((
                ActorLight {
                    owner: actor_entity,
                },
                PointLight2dBundle {
                    transform: transform.clone(),
                    point_light: PointLight2d {
                        radius: 150.0,
                        intensity: actor.intensity,
                        falloff: 10.0,
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }

    for (light_entity, light, mut point_light, mut light_transform) in light_query.iter_mut() {
        if let Ok((_, actor, actor_transform)) = actor_query.get(light.owner) {
            point_light.intensity = actor.intensity;
            light_transform.translation.x = actor_transform.translation.x;
            light_transform.translation.y = actor_transform.translation.y;
        } else {
            commands.entity(light_entity).despawn_recursive();
        }
    }
}

fn fire_bullet(
    mut actor_query: Query<(&mut Actor, &mut Transform), Without<Camera2d>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut writer: EventWriter<ClientMessage>,
    current: Res<CurrentLevel>,
    mut se_writer: EventWriter<GameCommand>,
    websocket: Res<WebSocketState>,
) {
    for (mut actor, actor_transform) in actor_query.iter_mut() {
        if actor.life <= 0 {
            return;
        }

        if actor.fire_state == ActorFireState::Fire
            && actor.cooltime == 0
            && BULLET_MANA_COST <= actor.mana
        {
            actor.mana = (actor.mana - BULLET_MANA_COST).max(0);

            let bullet_type = actor.bullet_type;

            let normalized = actor.pointer.normalize();
            let angle = actor.pointer.to_angle();
            for _ in 0..BULLETS_PER_FIRE {
                let angle_with_random = angle + (random::<f32>() - 0.5) * BULLET_SCATTERING;
                let direction = Vec2::from_angle(angle_with_random);
                let range = WITCH_COLLIDER_RADIUS + BULLET_RADIUS + BULLET_SPAWNING_MARGIN;
                let bullet_position = actor_transform.translation.truncate() + range * normalized;

                spawn_bullet(
                    &mut commands,
                    assets.asset.clone(),
                    bullet_position,
                    direction * actor.bullet_speed,
                    actor.bullet_lifetime,
                    Some(actor.uuid),
                    &mut se_writer,
                    actor.group,
                    actor.filter,
                    bullet_type,
                );

                if actor.online && websocket.ready_state == ReadyState::OPEN {
                    if let Some(level) = current.0 {
                        let serialized = bincode::serialize(&RemoteMessage::Fire {
                            sender: actor.uuid,
                            uuid: actor.uuid,
                            level,
                            x: bullet_position.x,
                            y: bullet_position.y,
                            vx: direction.x * actor.bullet_speed,
                            vy: direction.y * actor.bullet_speed,
                            bullet_lifetime: actor.bullet_lifetime,
                            bullet_type: bullet_type,
                        })
                        .unwrap();
                        writer.send(ClientMessage::Binary(serialized));
                    }
                }
            }

            actor.cooltime = BULLET_MAX_COOLTIME;
        } else {
            actor.cooltime = (actor.cooltime - actor.reload_speed).max(0);
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_sprite_flip, update_actor_light).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            (fire_bullet, recovery_mana).run_if(in_state(GameState::InGame)),
        );
    }
}
