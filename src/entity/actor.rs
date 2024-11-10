use crate::config::GameConfig;
use crate::constant::MAX_WANDS;
use crate::entity::breakable::BreakableSprite;
use crate::spell::cast_spell;
use crate::wand::Wand;
use crate::{asset::GameAssets, command::GameCommand, states::GameState};
use bevy::prelude::*;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_rapier2d::plugin::PhysicsSet;
use bevy_rapier2d::prelude::{ExternalForce, Group};
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};
use std::f32::consts::PI;
use uuid::Uuid;

/// ライフを持ち、弾丸のダメージの対象となるエンティティを表します
#[derive(Component)]
pub struct Actor {
    pub uuid: Uuid,

    /// 次の魔法を発射できるまでのクールタイム
    pub spell_delay: i32,

    pub mana: i32,

    pub max_mana: i32,

    pub life: i32,
    pub max_life: i32,

    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,

    pub intensity: f32,

    /// アクターが移動しようとしている方向を表します
    /// この値と各アクターの移動力係数の積が、実際の ExternalForce になります
    /// プレイヤーキャラクターの場合はキーボードやゲームパッドの方向キーの入力、
    /// 敵キャラクターの場合は Enemy によって決定された移動方向を表します
    /// また、このベクトルがゼロでないときは歩行アニメーションになります
    pub move_direction: Vec2,

    /// 種族や装備によって決まる移動速度係数
    /// あとで修正する
    pub move_force: f32,

    pub fire_state: ActorFireState,

    pub group: Group,

    pub filter: Group,

    pub current_wand: usize,

    pub wands: [Option<Wand>; MAX_WANDS],
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

/// 攻撃状態にあるアクターがスペルを詠唱します
fn fire_bullet(
    mut actor_query: Query<(&mut Actor, &mut Transform), Without<Camera2d>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut writer: EventWriter<ClientMessage>,
    mut se_writer: EventWriter<GameCommand>,
    websocket: Res<WebSocketState>,
    config: Res<GameConfig>,
) {
    let online = config.online && websocket.ready_state == ReadyState::OPEN;

    for (mut actor, actor_transform) in actor_query.iter_mut() {
        if actor.life <= 0 {
            return;
        }

        if let Some(wand) = &actor.wands[actor.current_wand] {
            if let Some(spell) = wand.slots[0] {
                if actor.fire_state == ActorFireState::Fire {
                    cast_spell(
                        &mut commands,
                        &assets,
                        &mut writer,
                        &mut se_writer,
                        &mut actor,
                        &actor_transform,
                        spell,
                        online,
                    );
                }
            }
        }

        actor.spell_delay = (actor.spell_delay - 1).max(0);
    }
}

/// actor.move_direction の値に従って、アクターに外力を適用します
fn apply_external_force(mut player_query: Query<(&Actor, &mut ExternalForce)>) {
    for (actor, mut force) in player_query.iter_mut() {
        force.force = actor.move_direction * actor.move_force;
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
            (apply_external_force, fire_bullet, recovery_mana)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
