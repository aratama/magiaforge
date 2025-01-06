use crate::asset::GameAssets;
use crate::cast::cast_spell;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::ENEMY_GROUPS;
use crate::constant::MAX_WANDS;
use crate::constant::NEUTRAL_GROUPS;
use crate::constant::PLAYER_GROUPS;
use crate::constant::TILE_SIZE;
use crate::controller::player::Player;
use crate::entity::bullet::Trigger;
use crate::entity::impact::SpawnImpact;
use crate::inventory::Inventory;
use crate::inventory_item::InventoryItemType;
use crate::level::entities::SpawnEntity;
use crate::se::SEEvent;
use crate::se::SE;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::floating::FloatingContent;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::plugin::PhysicsSet;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::ExternalForce;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
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

    pub quick_cast: u32,

    pub precision: f32,
}

#[derive(Reflect, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorState {
    #[default]
    Idle,
    Run,
    GettingUp,
}

/// ライフを持ち、弾丸のダメージの対象となるエンティティを表します
///
/// 移動
/// アクターの移動は、stateとmove_directionを設定することで行います
/// stateをActorState::Runに設定すると move_direction の方向に移動します
/// またこのとき、それぞれのActorの実装は歩行のアニメーションを再生します
///
#[derive(Component, Reflect, Debug)]
pub struct Actor {
    pub uuid: Uuid,

    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,

    pub point_light_radius: f32,

    /// アクターが移動しようとしている方向を表します
    /// 移動と停止を切り替えるときは move_direction を変更します
    /// この値と各アクターの移動力係数の積が、実際の ExternalForce になります
    /// プレイヤーキャラクターの場合はキーボードやゲームパッドの方向キーの入力、
    /// 敵キャラクターの場合は Enemy によって決定された移動方向を表します
    /// また、このベクトルがゼロでないときは歩行アニメーションになります
    pub move_direction: Vec2,

    /// 種族や装備によって決まる移動速度係数です
    /// 歩行と停止の切り替えは state を切り替えることで行うため、
    /// move_force を操作中に変更はしません
    pub move_force: f32,

    pub fire_state: ActorFireState,

    pub fire_state_secondary: ActorFireState,

    pub current_wand: usize,

    /// アクターが所持している杖のリスト
    /// モンスターの呪文詠唱の仕組みもプレイヤーキャラクターと同一であるため、
    /// 内部的にはモンスターも杖を持っていることになっています
    pub wands: [Wand; MAX_WANDS],
    // pub queue: Vec,
    pub inventory: Inventory,

    // 弾丸へのバフ効果
    // 一回発射するごとにリセットされます
    pub effects: CastEffects,

    pub actor_group: ActorGroup,

    pub golds: u32,

    // コリジョンの半径です
    // 大型のモンスターは半径が大きいので、中心間の距離では攻撃が当たるかどうか判定できません
    pub radius: f32,

    pub state: ActorState,

    /// 再び詠唱操作をできるようになるまでの待ち時間
    /// フキダシを閉じたあとなど、一定時間詠唱不能にしておかないと、
    /// フキダシを閉じると同時に詠唱をしてしまう
    pub wait: u32,

    /// 蜘蛛の巣などの罠に引っかかって動けなくなっている場合は正の数
    /// 歩くと減少します
    pub trapped: u32,

    /// この値が正の数のときはトラップを回避できます
    /// 歩くと減少します
    pub trap_moratorium: u32,

    /// 蜘蛛の巣から逃れる速度
    /// 毎ターンこの値が trapped から減算され、trappedが0になるとアクターが解放されます
    /// また、解放された瞬間に trap_moratorium が 180 に設定され、
    /// 3秒間は再びトラップにかからないようになります
    pub floundering: u32,

    pub fire_resistance: bool,
}

pub struct ActorProps {
    pub uuid: Uuid,
    pub angle: f32,
    pub point_light_radius: f32,
    pub current_wand: usize,
    pub actor_group: ActorGroup,
    pub golds: u32,
    pub wands: [Wand; MAX_WANDS],
    pub inventory: Inventory,
    pub radius: f32,
    pub move_force: f32,
    pub fire_resistance: bool,
}

impl Actor {
    pub fn new(
        ActorProps {
            uuid,
            angle,
            point_light_radius,
            current_wand,
            actor_group,
            golds,
            wands,
            inventory,
            radius,
            move_force,
            fire_resistance,
        }: ActorProps,
    ) -> Self {
        Actor {
            uuid,
            pointer: Vec2::from_angle(angle),
            point_light_radius,
            radius,
            current_wand,
            actor_group,
            golds,
            wands,
            inventory,
            move_force,
            fire_resistance,

            move_direction: Vec2::ZERO,
            fire_state: ActorFireState::Idle,
            fire_state_secondary: ActorFireState::Idle,
            effects: default(),
            state: ActorState::default(),
            wait: 0,
            trapped: 0,
            trap_moratorium: 0,
            floundering: 1,
        }
    }

    #[allow(dead_code)]
    pub fn get_item_icon(&self, index: FloatingContent) -> Option<&str> {
        match index {
            FloatingContent::Inventory(index) => {
                self.inventory.get(index).map(|i| i.item_type.get_icon())
            }
            FloatingContent::WandSpell(w, s) => {
                self.wands[w].slots[s].map(|spell| spell.spell_type.to_props().icon)
            }
        }
    }

    pub fn get_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        self.wands[wand_index].slots[spell_index]
    }

    pub fn get_wand_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        self.wands[wand_index].slots[spell_index]
    }

    /// 現在所持している有料呪文の合計金額を返します
    pub fn dept(&self) -> u32 {
        let mut dept = self.inventory.dept();

        let w: u32 = self.wands.iter().map(|wand| wand.dept()).sum();

        dept += w;

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

        for wand in self.wands.iter_mut() {
            for s in wand.slots.iter_mut() {
                if let Some(ref mut spell) = s {
                    spell.price = 0;
                }
            }
        }

        true
    }

    /// 装備を含めた移動力の合計を返します
    /// ただし魔法発射中のペナルティは含まれません
    fn get_total_move_force(&self) -> f32 {
        let mut force = self.move_force;

        //todo

        for wand in self.wands.iter() {
            for slot in wand.slots {
                force += match slot {
                    Some(WandSpell {
                        spell_type: SpellType::SpikeBoots,
                        ..
                    }) => 40000.0,
                    _ => 0.0,
                }
            }
        }
        force
    }

    pub fn get_total_scale_factor(&self) -> f32 {
        let mut scale_factor: f32 = -1.0;

        // todo

        for wand in self.wands.iter() {
            for slot in wand.slots {
                scale_factor += match slot {
                    Some(WandSpell {
                        spell_type: SpellType::Telescope,
                        ..
                    }) => 0.5,
                    Some(WandSpell {
                        spell_type: SpellType::Magnifier,
                        ..
                    }) => -0.5,
                    _ => 0.0,
                }
            }
        }

        scale_factor.max(-2.0).min(1.0)
    }

    pub fn get_owned_spell_types(&self) -> HashSet<SpellType> {
        let mut discovered_spells = HashSet::new();
        for item in self.inventory.0.iter() {
            if let Some(ref item) = item {
                match item.item_type {
                    InventoryItemType::Spell(spell) if item.price == 0 => {
                        let _ = discovered_spells.insert(spell);
                    }
                    _ => {}
                };
            }
        }
        for wand in self.wands.iter() {
            for item in wand.slots.iter() {
                if let Some(ref item) = item {
                    if item.price == 0 {
                        let _ = discovered_spells.insert(item.spell_type);
                    }
                }
            }
        }
        discovered_spells
    }
}

#[derive(Reflect, Debug, PartialEq, Clone, Copy)]
pub enum ActorFireState {
    Idle,

    /// Actorのpointerに向かって弾丸を発射します
    Fire,
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorGroup {
    Player,
    Enemy,

    /// 中立のアクターは敵と味方の両方の攻撃を受けます
    Neutral,
}

impl ActorGroup {
    pub fn to_groups(&self) -> CollisionGroups {
        match self {
            ActorGroup::Player => *PLAYER_GROUPS,
            ActorGroup::Enemy => *ENEMY_GROUPS,
            ActorGroup::Neutral => *NEUTRAL_GROUPS,
        }
    }
}

#[derive(Event, Debug, Clone)]
pub enum ActorEvent {
    Damaged {
        actor: Entity,
        position: Vec2,
        damage: u32,
        fire: bool,
        impulse: Vec2,
    },
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
    actor_query: Query<(Entity, &Actor, &Transform, Option<&Player>), Without<ActorLight>>,
) {
    for (actor_entity, actor, transform, _) in actor_query.iter() {
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
                    radius: actor.point_light_radius,
                    intensity: 1.0,
                    falloff: 10.0,
                    ..default()
                },
            ));
        }
    }

    for (light_entity, light, mut point_light, mut light_transform) in light_query.iter_mut() {
        if let Ok((_, actor, actor_transform, player)) = actor_query.get(light.owner) {
            if 0.0 < actor.point_light_radius {
                point_light.intensity = 2.5;
                point_light.color = Color::WHITE;
                point_light.radius = actor.point_light_radius;
            } else if player.is_some() {
                point_light.intensity = 1.0;
                point_light.color = Color::hsv(240.0, 0.5, 1.0);
                point_light.radius = TILE_SIZE * 3.0;
            };
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
            Option<&Player>,
        ),
        Without<Camera2d>,
    >,
    mut remote_writer: EventWriter<ClientMessage>,
    mut se_writer: EventWriter<SEEvent>,
    mut impact_writer: EventWriter<SpawnImpact>,
    mut spawn: EventWriter<SpawnEntity>,
    websocket: Res<WebSocketState>,
) {
    let online = websocket.ready_state == ReadyState::OPEN;

    for (actor_entity, mut actor, mut actor_life, actor_transform, mut actor_impulse, player) in
        actor_query.iter_mut()
    {
        if actor.fire_state == ActorFireState::Fire {
            if 0 < actor.wait {
                actor.wait -= 1;
                continue;
            }

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
                &mut impact_writer,
                &mut spawn,
                current_wand,
                player.is_some(),
                Trigger::Primary,
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
                &mut impact_writer,
                &mut spawn,
                MAX_WANDS - 1,
                player.is_some(),
                Trigger::Secondary,
            );
        }

        for wand in actor.wands.iter_mut() {
            wand.delay = (wand.delay as i32 - 1).max(0) as u32;
        }
    }
}

/// actor.move_direction の値に従って、アクターに外力を適用します
/// 魔法の発射中は移動速度が低下します
fn apply_external_force(
    mut query: Query<(&mut Actor, &mut ExternalForce, &Transform)>,
    mut se: EventWriter<SEEvent>,
) {
    for (mut actor, mut external_force, transform) in query.iter_mut() {
        let force = actor.move_direction
            * actor.get_total_move_force()
            * if actor.fire_state == ActorFireState::Fire
                || actor.fire_state_secondary == ActorFireState::Fire
            {
                0.5
            } else {
                1.0
            };

        if 0.0 < force.length() {
            if 0 < actor.trapped {
                external_force.force = Vec2::ZERO;
                actor.trapped -= actor.floundering;
                if actor.trapped == 0 {
                    actor.trap_moratorium = 180;
                    se.send(SEEvent::pos(SE::Zombie, transform.translation.truncate()));
                }
            } else {
                external_force.force = force;
            }

            if 0 < actor.trap_moratorium {
                actor.trap_moratorium -= 1;
            }
        } else {
            external_force.force = Vec2::ZERO;
        }
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>();
        app.add_event::<ActorEvent>();
        app.add_systems(
            Update,
            (update_sprite_flip, update_actor_light)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
        app.add_systems(
            FixedUpdate,
            (apply_external_force, fire_bullet)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
