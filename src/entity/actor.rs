use crate::cast::cast_spell;
use crate::constant::MAX_WANDS;
use crate::entity::life::LifeBeingSprite;
use crate::wand::{Wand, WandSpell};
use crate::{asset::GameAssets, se::SECommand, states::GameState};
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::plugin::PhysicsSet;
use bevy_rapier2d::prelude::ExternalForce;
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};
use std::f32::consts::PI;
use uuid::Uuid;

use super::life::Life;
use super::slime_seed::SpawnSlimeSeed;

#[derive(Reflect, Clone, Copy, Default)]
pub struct CastEffects {
    // pub queue: Vec,
    pub bullet_speed_buff_factor: f32,
    // マルチキャストで待機中の弾丸
    // pub multicasts: Vec<SpawnBulletProps>,
    pub homing: f32,

    pub bullet_damage_buff_amount: i32,
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorState {
    #[default]
    Idle,
    Run,
}

/// ライフを持ち、弾丸のダメージの対象となるエンティティを表します
#[derive(Component, Reflect)]
pub struct Actor {
    pub uuid: Uuid,

    /// 次の魔法を発射できるまでのクールタイム
    pub spell_delay: i32,

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

    pub current_wand: usize,

    /// アクターが所持している杖のリスト
    /// モンスターの呪文詠唱の仕組みもプレイヤーキャラクターと同一であるため、
    /// 内部的にはモンスターも杖を持っていることになっています
    pub wands: [Option<Wand>; MAX_WANDS],
    // pub queue: Vec,

    // 弾丸へのバフ効果
    // 一回発射するごとにリセットされます
    pub effects: CastEffects,

    pub actor_group: ActorGroup,
}

impl Actor {
    pub fn get_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        if let Some(ref wand) = self.wands[wand_index] {
            return wand.slots[spell_index];
        }
        None
    }

    pub fn dept(&self) -> u32 {
        self.wands
            .iter()
            .filter_map(|wand| wand.as_ref())
            .map(|wand| wand.price + wand.dept())
            .sum()
    }
}

#[derive(Reflect, Debug, PartialEq, Clone, Copy)]
pub enum ActorFireState {
    Idle,

    /// Actorのpointerに向かって弾丸を発射します
    Fire,
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorGroup {
    Player,
    Enemy,
}

fn update_sprite_flip(
    actor_query: Query<&Actor>,
    mut sprite_query: Query<(&Parent, &mut Sprite), With<LifeBeingSprite>>,
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
                Name::new("actor light"),
                StateScoped(GameState::InGame),
                ActorLight {
                    owner: actor_entity,
                },
                transform.clone(),
                PointLight2d {
                    radius: 160.0,
                    intensity: actor.intensity,
                    falloff: 10.0,
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
    mut actor_query: Query<(Entity, &mut Actor, &mut Life, &mut Transform), Without<Camera2d>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut writer: EventWriter<ClientMessage>,
    mut se_writer: EventWriter<SECommand>,
    websocket: Res<WebSocketState>,
    mut slime_writer: EventWriter<SpawnSlimeSeed>,
) {
    let online = websocket.ready_state == ReadyState::OPEN;

    for (actor_entity, mut actor, mut actor_life, actor_transform) in actor_query.iter_mut() {
        if actor_life.life <= 0 {
            return;
        }

        if actor.fire_state == ActorFireState::Fire {
            while actor.spell_delay == 0 {
                let delay = cast_spell(
                    &mut commands,
                    &assets,
                    &mut writer,
                    &mut se_writer,
                    actor_entity,
                    &mut actor,
                    &mut actor_life,
                    &actor_transform,
                    online,
                    &mut slime_writer,
                );

                actor.spell_delay += delay.max(1);
            }
        }

        actor.spell_delay = (actor.spell_delay - 1).max(0);
    }
}

/// actor.move_direction の値に従って、アクターに外力を適用します
/// 魔法の発射中は移動速度が低下します
fn apply_external_force(mut player_query: Query<(&Actor, &mut ExternalForce)>) {
    for (actor, mut force) in player_query.iter_mut() {
        force.force = actor.move_direction
            * actor.move_force
            * match actor.fire_state {
                ActorFireState::Idle => 1.0,
                ActorFireState::Fire => 0.5,
            };
    }
}

fn update_actor_state(mut witch_query: Query<(&Actor, &mut ActorState)>) {
    for (actor, mut state) in witch_query.iter_mut() {
        if actor.move_direction.length() < 0.01 {
            if *state != ActorState::Idle {
                *state = ActorState::Idle;
            }
        } else {
            if *state != ActorState::Run {
                *state = ActorState::Run;
            }
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>();
        app.add_systems(
            Update,
            (update_sprite_flip, update_actor_light, update_actor_state)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            (apply_external_force, fire_bullet)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
