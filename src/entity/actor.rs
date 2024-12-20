use crate::cast::cast_spell;
use crate::constant::{MAX_ITEMS_IN_EQUIPMENT, MAX_WANDS};
use crate::controller::player::Equipment;
use crate::entity::life::Life;
use crate::entity::life::LifeBeingSprite;
use crate::entity::slime_seed::SpawnSlimeSeed;
use crate::equipment::EquipmentType;
use crate::inventory::Inventory;
use crate::ui::floating::FloatingContent;
use crate::wand::{Wand, WandSpell};
use crate::{asset::GameAssets, se::SEEvent, states::GameState};
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::plugin::PhysicsSet;
use bevy_rapier2d::prelude::{ExternalForce, ExternalImpulse};
use bevy_simple_websocket::{ClientMessage, ReadyState, WebSocketState};
use std::f32::consts::PI;
use uuid::Uuid;

#[derive(Reflect, Clone, Copy, Default, Debug)]
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
#[derive(Component, Reflect, Debug)]
pub struct Actor {
    pub uuid: Uuid,

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

    pub fire_state_secondary: ActorFireState,

    pub current_wand: usize,

    /// アクターが所持している杖のリスト
    /// モンスターの呪文詠唱の仕組みもプレイヤーキャラクターと同一であるため、
    /// 内部的にはモンスターも杖を持っていることになっています
    pub wands: [Option<Wand>; MAX_WANDS],
    // pub queue: Vec,
    pub inventory: Inventory,

    pub equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],

    // 弾丸へのバフ効果
    // 一回発射するごとにリセットされます
    pub effects: CastEffects,

    pub actor_group: ActorGroup,

    pub golds: u32,
}

impl Actor {
    #[allow(dead_code)]
    pub fn get_item_icon(&self, index: FloatingContent) -> Option<&str> {
        match index {
            FloatingContent::Inventory(index) => {
                self.inventory.get(index).map(|i| i.item_type.get_icon())
            }
            FloatingContent::Equipment(index) => {
                self.equipments[index].map(|i| i.equipment_type.to_props().icon)
            }
            FloatingContent::Wand(index) => self.wands[index]
                .as_ref()
                .map(|i| i.wand_type.to_props().icon),
            FloatingContent::WandSpell(w, s) => self.wands[w]
                .as_ref()
                .and_then(|wand| wand.slots[s].map(|spell| spell.spell_type.to_props().icon)),
        }
    }

    pub fn get_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        if let Some(ref wand) = self.wands[wand_index] {
            return wand.slots[spell_index];
        }
        None
    }

    pub fn get_wand(&self, wand_index: usize) -> Option<&Wand> {
        self.wands[wand_index].as_ref()
    }

    pub fn get_wand_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        self.wands[wand_index]
            .as_ref()
            .and_then(|w| w.slots[spell_index])
    }

    /// 現在所持している有料呪文の合計金額を返します
    pub fn dept(&self) -> u32 {
        let mut dept = self.inventory.dept();

        let w: u32 = self
            .wands
            .iter()
            .filter_map(|wand| wand.as_ref())
            .map(|wand| wand.price + wand.dept())
            .sum();

        dept += w;

        for e in self.equipments {
            if let Some(equipment) = e {
                dept += equipment.price;
            }
        }

        return dept;
    }

    /// 清算します
    /// いま所持している有料のスペルをすべて無料に戻します
    pub fn liquidate(&mut self) -> bool {
        let dept = self.dept();
        if self.golds < dept {
            return false;
        }

        self.golds -= dept;

        for item in self.inventory.0.iter_mut() {
            if let Some(item) = item {
                item.price = 0;
            }
        }

        for e in self.equipments.iter_mut() {
            if let Some(equipment) = e {
                equipment.price = 0;
            }
        }

        for w in self.wands.iter_mut() {
            if let Some(ref mut wand) = w {
                wand.price = 0;
                for s in wand.slots.iter_mut() {
                    if let Some(mut spell) = s {
                        spell.price = 0;
                    }
                }
            }
        }

        true
    }

    /// 装備を含めた移動力の合計を返します
    /// ただし魔法発射中のペナルティは含まれません
    fn get_total_move_force(&self) -> f32 {
        let mut force = self.move_force;
        for equipment in self.equipments {
            force += match equipment {
                Some(Equipment {
                    equipment_type: EquipmentType::SpikeBoots,
                    ..
                }) => 40000.0,
                _ => 0.0,
            }
        }
        force
    }

    pub fn get_total_scale_factor(&self) -> f32 {
        let mut scale_factor: f32 = -1.0;
        for equipment in self.equipments {
            scale_factor += match equipment {
                Some(Equipment {
                    equipment_type: EquipmentType::Telescope,
                    ..
                }) => 0.5,
                Some(Equipment {
                    equipment_type: EquipmentType::Magnifier,
                    ..
                }) => -0.5,
                _ => 0.0,
            }
        }
        scale_factor.max(-2.0).min(1.0)
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
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut actor_query: Query<
        (
            Entity,
            &mut Actor,
            &mut Life,
            &mut Transform,
            &mut ExternalImpulse,
        ),
        Without<Camera2d>,
    >,
    mut remote_writer: EventWriter<ClientMessage>,
    mut se_writer: EventWriter<SEEvent>,
    mut slime_writer: EventWriter<SpawnSlimeSeed>,
    websocket: Res<WebSocketState>,
) {
    let online = websocket.ready_state == ReadyState::OPEN;

    for (actor_entity, mut actor, mut actor_life, actor_transform, mut actor_impulse) in
        actor_query.iter_mut()
    {
        if actor.fire_state == ActorFireState::Fire {
            let current_wand = actor.current_wand;
            cast_spell(
                &mut commands,
                &assets,
                actor_entity,
                &mut actor,
                &mut actor_life,
                &actor_transform,
                &mut actor_impulse,
                online,
                &mut remote_writer,
                &mut se_writer,
                &mut slime_writer,
                current_wand,
            );
        }

        if actor.fire_state_secondary == ActorFireState::Fire {
            cast_spell(
                &mut commands,
                &assets,
                actor_entity,
                &mut actor,
                &mut actor_life,
                &actor_transform,
                &mut actor_impulse,
                online,
                &mut remote_writer,
                &mut se_writer,
                &mut slime_writer,
                MAX_WANDS - 1,
            );
        }

        for wand_option in actor.wands.iter_mut() {
            if let Some(ref mut wand) = wand_option {
                wand.delay = (wand.delay as i32 - 1).max(0) as u32;
            }
        }
    }
}

/// actor.move_direction の値に従って、アクターに外力を適用します
/// 魔法の発射中は移動速度が低下します
fn apply_external_force(mut player_query: Query<(&Actor, &mut ExternalForce)>) {
    for (actor, mut force) in player_query.iter_mut() {
        force.force = actor.move_direction
            * actor.get_total_move_force()
            * if actor.fire_state == ActorFireState::Fire
                || actor.fire_state_secondary == ActorFireState::Fire
            {
                0.5
            } else {
                1.0
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
