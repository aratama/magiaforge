pub mod bomb;
pub mod book_shelf;
pub mod chest;
pub mod chicken;
pub mod stone_lantern;
pub mod witch;

use crate::cast::cast_spell;
use crate::collision::ENEMY_BULLET_GROUP;
use crate::collision::ENEMY_GROUPS;
use crate::collision::FLYING_ENEMY_GROUPS;
use crate::collision::FLYING_NEUTRAL_GROUPS;
use crate::collision::FLYING_PLAYER_GROUPS;
use crate::collision::NEUTRAL_GROUPS;
use crate::collision::PLAYER_BULLET_GROUP;
use crate::collision::PLAYER_GROUPS;
use crate::collision::RABBIT_GROUPS;
use crate::collision::SHADOW_GROUPS;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::get_entity_z;
use crate::component::entity_depth::ChildEntityDepth;
use crate::component::metamorphosis::cast_metamorphosis;
use crate::component::metamorphosis::metamorphosis_effect;
use crate::component::metamorphosis::Metamorphosed;
use crate::constant::BLOOD_LAYER_Z;
use crate::constant::MAX_WANDS;
use crate::constant::TILE_SIZE;
use crate::constant::*;
use crate::controller::player::recovery;
use crate::controller::player::Player;
use crate::controller::player::PlayerControlled;
use crate::enemy::huge_slime::Boss;
use crate::enemy::huge_slime::HugeSlime;
use crate::enemy::huge_slime::HugeSlimeState;
use crate::entity::bullet::HomingTarget;
use crate::entity::bullet::Trigger;
use crate::entity::fire::Burnable;
use crate::entity::fire::Fire;
use crate::entity::gold::spawn_gold;
use crate::entity::impact::SpawnImpact;
use crate::hud::life_bar::spawn_life_bar;
use crate::interpreter::cmd::Cmd;
use crate::interpreter::cmd::Value;
use crate::interpreter::interpreter::InterpreterEvent;
use crate::inventory::Inventory;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::level::tile::Tile;
use crate::level::world::GameWorld;
use crate::level::world::LevelScoped;
use crate::registry::ActorCollider;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::BASHA2;
use crate::se::CRY;
use crate::se::DAMAGE;
use crate::se::SCENE2;
use crate::se::SHURIKEN;
use crate::se::SUNA;
use crate::se::ZOMBIE;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::Spell;
use crate::states::GameState;
use crate::strategy::Commander;
use crate::ui::floating::FloatingContent;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::ExternalForce;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_rapier2d::prelude::GravityScale;
use bevy_rapier2d::prelude::Group;
use bevy_rapier2d::prelude::LockedAxes;
use bevy_rapier2d::prelude::QueryFilter;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Velocity;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;
use bomb::Bomb;
use chest::Chest;
use chicken::Chicken;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::f32::consts::PI;
use uuid::Uuid;
use vleue_navigator::prelude::PrimitiveObstacle;
use witch::WitchWandSprite;

/// アクターの種類を表します
/// registry.actor.ron で種類ごとに移動速度やジャンプ力などが設定されます
#[derive(Reflect, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorType(pub String);

impl ActorType {
    pub fn new(name: &str) -> Self {
        ActorType(name.to_string())
    }
}

#[derive(Reflect, Clone, Default, Debug, Deserialize)]
pub struct CastEffects {
    // pub queue: Vec,
    pub bullet_speed_buff_factor: f32,
    // マルチキャストで待機中の弾丸
    // pub multicasts: Vec<SpawnBulletProps>,
    pub homing: f32,

    pub bullet_damage_buff_amount: i32,

    pub quick_cast: u32,

    pub precision: f32,

    pub metamorphse: Option<ActorType>,

    pub dispel: u8,

    pub web: u8,

    pub slash: Vec<u32>,

    pub levitation: u32,
}

#[derive(Reflect, Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum ActorState {
    #[default]
    Idle,
    Run,
}

#[derive(Component, Debug)]
pub struct ActorAppearanceSprite;

/// 自発的に移動し、攻撃の対象になる、プレイヤーキャラクターや敵モンスターを表します
/// Actorは外観の構造を規定しません。外観は各エンティティで具体的に実装するか、
/// BasicEnemyのような抽象化されたエンティティで実装しています
///
/// 移動
/// アクターの移動は、stateとmove_directionを設定することで行います
/// stateをActorState::Runに設定すると move_direction の方向に移動します
/// またこのとき、それぞれのActorの実装は歩行のアニメーションを再生します
///
#[derive(Component, Reflect, Debug, Clone, Deserialize)]
#[require(
    HomingTarget,
    StateScoped<GameState>(||StateScoped(GameState::InGame)),
    Visibility,
    Damping,
    LockedAxes(||LockedAxes::ROTATION_LOCKED),
    RigidBody(||RigidBody::Dynamic),
    GravityScale(||GravityScale(0.0)),
    ExternalForce,
    ExternalImpulse,
    Velocity,
    ActiveEvents(||ActiveEvents::COLLISION_EVENTS)
)]
pub struct Actor {
    pub actor_type: ActorType,

    pub uuid: Uuid,

    pub actor_group: ActorGroup,

    // ライフ //////////////////////////////////////////////////////////////////////////////////////
    pub life: u32,

    pub max_life: u32,

    /// ダメージを受けた時の振動の幅
    pub amplitude: f32,

    /// 次に炎でダメージを受けるまでの間隔
    pub fire_damage_wait: u32,

    // 操作 ///////////////////////////////////////////////////////////////////////////////////////
    /// プレイヤーの位置からの相対的なポインターの位置
    pub pointer: Vec2,

    /// アクターが移動しようとしている方向を表します
    /// 移動と停止を切り替えるときは move_direction を変更します
    /// この値と各アクターの移動力係数の積が、実際の ExternalForce になります
    /// プレイヤーキャラクターの場合はキーボードやゲームパッドの方向キーの入力、
    /// 敵キャラクターの場合は Enemy によって決定された移動方向を表します
    /// また、このベクトルがゼロでないときは歩行アニメーションになります
    pub move_direction: Vec2,

    /// 種族や装備によって決まる移動速度係数です
    pub fire_state: ActorFireState,

    pub fire_state_secondary: ActorFireState,

    pub current_wand: u8,

    pub commander: Commander,

    pub home_position: Vec2,

    // 装備とアイテム /////////////////////////////////////////////////////////////////////////////////////////
    /// アクターが所持している杖のリスト
    /// モンスターの呪文詠唱の仕組みもプレイヤーキャラクターと同一であるため、
    /// 内部的にはモンスターも杖を持っていることになっています
    pub wands: [Wand; MAX_WANDS],
    // pub queue: Vec,
    pub inventory: Inventory,

    pub golds: u32,

    // 外観 ///////////////////////////////////////////////////////////////////////////////////////////
    pub state: ActorState,

    // 定数 ////////////////////////////////////////////////////////////////////////////////////////////////

    // 効果と状態異常 ////////////////////////////////////////////////////////////////////////////////////////////
    /// 再び詠唱操作をできるようになるまでの待ち時間
    /// フキダシを閉じたあとや変身直後など、一定時間詠唱不能にしておかないと、
    /// 他の操作と同時に詠唱をしてしまう
    pub wait: u32,

    /// 蜘蛛の巣などの罠に引っかかって動けなくなっている場合は正の数
    /// 歩くと減少します
    pub trapped: u32,

    /// この値が正の数のときはトラップを回避できます
    /// 歩くと減少します
    pub trap_moratorium: u32,

    // 弾丸へのバフ効果
    // 一回発射するごとにリセットされます
    pub effects: CastEffects,

    /// 0 より大きい場合は凍結状態で、移動や詠唱ができません
    /// 1フレームごとに defreeze だけ減少します
    pub frozen: u32,

    // 0 より大きい場合はよろめき状態で、移動や詠唱ができません
    // 1フレームごとに 1 ずつ減少します
    pub staggered: u32,

    // 0 より大きい場合は浮遊状態で、コリジョングループが変更されて水面などに足を踏み入れることができます
    // 1フレームごとに 1 ずつ減少します
    pub levitation: u32,

    // 0 より大きい場合は溺れている状態で、詠唱ができません。また移動速度が低下します
    // 水中にいる間は 1フレームに1 づつ増加し、60を超えるとダメージを受け、0に戻ります
    pub drown: u32,

    /// 複製された場合の残り時間
    pub cloned: Option<u32>,

    /// ゼロより大きい場合は起き上がりアニメーション中
    pub getting_up: u32,

    /// 影の中に隠れた状態
    /// この状態では弾丸に当たらないが、壁やアクターには当たる
    pub hidden: bool,

    pub velocity: f32,
    pub gravity: f32,
    pub just_landed: bool,
    pub v: f32,

    pub navigation_path: Vec<Vec2>,
}

#[derive(Default, Component, Reflect)]
#[require(Visibility(||Visibility::Hidden), Transform)]
pub struct ActorLevitationEffect;

/// Actor のスプライトをまとめる子エンティティのマーカーです
/// ActorSpriteGroupにはx座標とy座標に応じて自動的にz座標が割り当てられます
///
/// 通常、キャラクターのスプライトはルートのエンティティに直接アタッチされず、
/// この ActorSpriteGroup の子、ルートのエンティティ孫としてアタッチされます
/// これは、打撃アニメーションや浮遊アニメーションにおいて、本体の位置をそのままにスプタイトの位置だけ揺動させて表現するためです
#[derive(Default, Component, Reflect)]
#[require(Visibility, Counter, Transform, LifeBeingSprite, ChildEntityDepth)]
pub struct ActorSpriteGroup;

impl Actor {
    #[allow(dead_code)]
    pub fn get_item_icon(&self, registry: Registry, index: FloatingContent) -> Option<String> {
        match index {
            FloatingContent::Inventory(index) => self.inventory.get(index).as_ref().map(|i| {
                let props = registry.get_spell_props(&i.spell);
                props.icon.clone()
            }),
            FloatingContent::WandSpell(w, s) => self.wands[w].slots[s].as_ref().map(
                |WandSpell {
                     spell: ref spell_type,
                     ..
                 }| { registry.get_spell_props(spell_type).icon.clone() },
            ),
        }
    }

    pub fn get_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        self.wands[wand_index].slots[spell_index].clone()
    }

    pub fn get_wand_spell(&self, wand_index: usize, spell_index: usize) -> Option<WandSpell> {
        self.wands[wand_index].slots[spell_index].clone()
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
    fn get_total_move_force(&self, registry: &Registry) -> f32 {
        let props = registry.get_actor_props(&self.actor_type);
        let force = props.move_force;

        // もともとはここで装備による速度上昇を計算していましたが、
        // 常時発動型の魔法は単調で工夫の余地がないのでボツにしました

        force
    }

    pub fn get_total_scale_factor(&self) -> f32 {
        let mut scale_factor: f32 = -1.0;

        // todo

        for wand in self.wands.iter() {
            for slot in &wand.slots {
                scale_factor += match slot {
                    Some(WandSpell {
                        spell: spell_type, ..
                    }) if *spell_type == Spell::new("Telescope") => 0.5,
                    Some(WandSpell {
                        spell: spell_type, ..
                    }) if *spell_type == Spell::new("Magnifier") => -1.0,
                    _ => 0.0,
                }
            }
        }

        scale_factor.max(-2.0).min(1.0)
    }

    pub fn get_owned_spell_types(&self) -> HashSet<Spell> {
        let mut discovered_spells: HashSet<Spell> = HashSet::new();
        for item in self.inventory.0.iter() {
            if let Some(ref item) = item {
                if item.price == 0 {
                    let _ = discovered_spells.insert(item.spell.clone());
                }
            }
        }
        for wand in self.wands.iter() {
            for item in wand.slots.iter() {
                if let Some(ref item) = &item {
                    if item.price == 0 {
                        let _ = discovered_spells.insert(item.spell.clone());
                    }
                }
            }
        }
        discovered_spells
    }

    pub fn contains_in_slot(&self, spell: &Spell) -> bool {
        for wand in self.wands.iter() {
            for slot in wand.slots.iter() {
                if let Some(slot) = slot {
                    if slot.spell == *spell {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl Default for Actor {
    fn default() -> Self {
        let default_life = 100;
        Actor {
            actor_type: ActorType::new("Chicken"),
            uuid: Uuid::new_v4(),
            life: default_life,
            max_life: default_life,
            amplitude: 0.0,
            fire_damage_wait: 0,
            pointer: Vec2::ZERO,
            current_wand: 0,
            actor_group: ActorGroup::Neutral,
            golds: 0,
            wands: [
                Wand::default(),
                Wand::default(),
                Wand::default(),
                Wand::default(),
            ],
            commander: Commander::default(),
            inventory: Inventory::new(),
            move_direction: Vec2::ZERO,
            fire_state: ActorFireState::Idle,
            fire_state_secondary: ActorFireState::Idle,
            effects: default(),
            state: ActorState::default(),
            wait: 30,
            home_position: Vec2::ZERO,
            trapped: 0,
            trap_moratorium: 0,
            frozen: 0,
            levitation: 0,
            drown: 0,
            staggered: 0,
            cloned: None,
            getting_up: 0,
            hidden: false,
            velocity: 0.0,
            gravity: -0.2,
            just_landed: false,
            v: 0.0,
            navigation_path: vec![],
        }
    }
}

#[derive(Reflect, Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum ActorFireState {
    Idle,

    /// Actorのpointerに向かって弾丸を発射します
    Fire,
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ActorGroup {
    #[default]
    Friend,

    /// 中立のアクターです
    /// 敵と味方の両方の攻撃を受け、敵から攻撃対象として狙われます
    Neutral,

    Enemy,

    /// 本棚などの設備のアクターです
    /// 敵と味方の両方の攻撃を受けますが、敵から攻撃の対象として狙われません
    Entity,
}

impl ActorGroup {
    pub fn to_groups(&self, v: f32, drowning: u32) -> CollisionGroups {
        match self {
            ActorGroup::Friend if 0 < drowning || 0.0 < v => *FLYING_PLAYER_GROUPS,
            ActorGroup::Friend => *PLAYER_GROUPS,
            ActorGroup::Enemy if 0 < drowning || 0.0 < v => *FLYING_ENEMY_GROUPS,
            ActorGroup::Enemy => *ENEMY_GROUPS,
            ActorGroup::Neutral if 0 < drowning || 0.0 < v => *FLYING_NEUTRAL_GROUPS,
            ActorGroup::Neutral => *NEUTRAL_GROUPS,
            ActorGroup::Entity => *NEUTRAL_GROUPS,
        }
    }

    pub fn to_bullet_group(&self) -> CollisionGroups {
        match self {
            ActorGroup::Friend => *PLAYER_BULLET_GROUP,
            ActorGroup::Enemy => *ENEMY_BULLET_GROUP,
            ActorGroup::Neutral => CollisionGroups::new(Group::NONE, Group::NONE), // 中立グループは弾丸を発射しません
            ActorGroup::Entity => CollisionGroups::new(Group::NONE, Group::NONE), // 設備は弾丸を発射しません
        }
    }
}

#[derive(Event, Debug, Clone)]
pub enum ActorEvent {
    /// アクターにダメージを与えます
    /// このイベント以外を通じてアクターのライフを変更した場合は、ダメージの数値が画面に表示されません
    Damaged {
        actor: Entity,
        position: Vec2,
        damage: u32,
        fire: bool,
        impulse: Vec2,
        stagger: u32,
        metamorphose: Option<ActorType>,
        dispel: bool,
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

fn update_actor_light(
    registry: Registry,
    mut query: Query<(&mut Actor, Option<&Player>, &mut PointLight2d)>,
) {
    for (actor, player, mut point_light) in query.iter_mut() {
        let mut point_light_radius_by_item: f32 = 0.0;
        // 複製されたアクターは光源を0とする
        // プレイヤーキャラクターは明るい光源を装備していることが多く、
        // 大量に複製すると明るくなりすぎるため
        if actor.cloned.is_none() {
            for wand in actor.wands.iter() {
                for slot in &wand.slots {
                    match slot {
                        Some(WandSpell {
                            spell: spell_type, ..
                        }) if *spell_type == Spell::new("Lantern") => {
                            point_light_radius_by_item += 160.0;
                        }
                        _ => {}
                    }
                }
            }
        }

        let props = registry.get_actor_props(&actor.actor_type);
        if 0.0 < props.point_light_radius && 0.0 < props.point_light_intensity {
            // アクター種別ごとの明るさが設定されている場合
            point_light.intensity = props.point_light_intensity;
            point_light.color = Color::hsla(
                props.point_light_color.0,
                props.point_light_color.1,
                props.point_light_color.2,
                props.point_light_color.3,
            );
            point_light.radius = props.point_light_radius;
            point_light.falloff = props.point_light_falloff;
        } else if 0.0 < point_light_radius_by_item {
            // 装備している呪文による明るさ
            point_light.intensity = 1.0;
            point_light.color = Color::hsv(0.0, 0.0, 1.0);
            point_light.radius = point_light_radius_by_item;
            point_light.falloff = 1.0;
        } else if player.is_some() {
            // プレイヤーキャラクターのみ、アクター種別や呪文装備による明るさがゼロでも、
            // 薄く青い光を発する
            point_light.intensity = 1.0;
            point_light.color = Color::hsv(240.0, 0.5, 1.0);
            point_light.radius = TILE_SIZE * 3.0;
            point_light.falloff = 1.0;
        };
    }
}

/// 攻撃状態にあるアクターがスペルを詠唱します
fn fire_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Registry,
    level: Res<GameWorld>,
    websocket: Res<WebSocketState>,

    mut remote_writer: EventWriter<ClientMessage>,
    mut se_writer: EventWriter<SEEvent>,
    mut impact_writer: EventWriter<SpawnImpact>,
    mut spawn: EventWriter<SpawnEvent>,

    mut actor_query: Query<
        (
            Entity,
            &mut Actor,
            &mut Transform,
            &mut ExternalImpulse,
            &Velocity,
            &mut CollisionGroups,
            Option<&Player>,
            Option<&PlayerControlled>,
            Option<&Metamorphosed>,
        ),
        Without<Camera2d>,
    >,
) {
    let online = websocket.ready_state == ReadyState::OPEN;

    for (
        actor_entity,
        mut actor,
        actor_transform,
        mut actor_impulse,
        actor_velocty,
        mut collision_groups,
        player,
        player_controlled,
        actor_metamorphosis,
    ) in actor_query.iter_mut()
    {
        if 0 < actor.wait {
            actor.wait -= 1;
            continue;
        }

        // 凍結時は攻撃不可
        if 0 < actor.frozen {
            continue;
        }

        if 0 < actor.staggered {
            continue;
        }

        if actor.fire_state == ActorFireState::Fire {
            let wand = actor.current_wand;
            cast_spell(
                &mut commands,
                &asset_server,
                &registry,
                &level,
                &mut remote_writer,
                &mut se_writer,
                &mut impact_writer,
                &mut spawn,
                actor_entity,
                &mut actor,
                &actor_transform,
                &mut actor_impulse,
                &actor_velocty,
                &mut collision_groups,
                &actor_metamorphosis,
                player,
                player_controlled.is_some(),
                online,
                wand,
                Trigger::Primary,
            );
        }

        if actor.fire_state_secondary == ActorFireState::Fire {
            cast_spell(
                &mut commands,
                &asset_server,
                &registry,
                &level,
                &mut remote_writer,
                &mut se_writer,
                &mut impact_writer,
                &mut spawn,
                actor_entity,
                &mut actor,
                &actor_transform,
                &mut actor_impulse,
                &actor_velocty,
                &mut collision_groups,
                &actor_metamorphosis,
                player,
                player_controlled.is_some(),
                online,
                MAX_WANDS as u8 - 1,
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
    registry: Registry,
    mut query: Query<(&mut Actor, &mut ExternalForce, &Transform)>,
    mut se: EventWriter<SEEvent>,
    level: Res<GameWorld>,
) {
    let constants = registry.actor();

    for (mut actor, mut external_force, transform) in query.iter_mut() {
        let position = transform.translation.truncate();
        let props = registry.get_actor_props(&actor.actor_type);

        let on_ice = match level.get_tile_by_coords(position).0.as_str() {
            "Ice" => true,
            _ => false,
        };

        let ratio = if 0 < actor.getting_up {
            0.0
        } else if 0 < actor.frozen {
            0.0
        } else if 0 < actor.staggered {
            0.0
        } else if 0 < actor.drown {
            constants.acceleration_on_drowning
        } else if 0.0 < actor.v {
            constants.acceleration_on_air
        } else if on_ice {
            constants.acceleration_on_ice
        } else if actor.fire_state == ActorFireState::Fire
            || actor.fire_state_secondary == ActorFireState::Fire
        {
            constants.acceleration_on_firing
        } else {
            1.0
        };

        let force = actor.move_direction * actor.get_total_move_force(&registry) * ratio;

        if 0.0 < force.length() {
            if 0 < actor.trapped {
                external_force.force = Vec2::ZERO;
                actor.trapped -= props.floundering;
                if actor.trapped == 0 {
                    actor.trap_moratorium = 180;
                    se.send(SEEvent::pos(ZOMBIE, position));
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

fn defreeze(registry: Registry, mut query: Query<&mut Actor>) {
    for mut actor in query.iter_mut() {
        let props = registry.get_actor_props(&actor.actor_type);
        if props.defreeze <= actor.frozen {
            actor.frozen -= props.defreeze;
        } else if 0 < actor.frozen {
            actor.frozen = 0;
        }
    }
}

pub fn collision_group_by_actor(mut query: Query<(&Actor, &mut CollisionGroups)>) {
    for (actor, mut groups) in query.iter_mut() {
        *groups = if actor.hidden {
            *SHADOW_GROUPS
        } else if actor.actor_type == ActorType::new("Rabbit") {
            *RABBIT_GROUPS
        } else {
            actor.actor_group.to_groups(actor.v, actor.drown)
        };
    }
}

fn decrement_levitation(mut actor_query: Query<&mut Actor>, registry: Registry) {
    for mut actor in actor_query.iter_mut() {
        let props = registry.get_actor_props(&actor.actor_type);
        if 0 < actor.levitation {
            actor.levitation -= 1;
        }
        if actor.levitation <= 240 && props.auto_levitation {
            actor.levitation = 240;
        }
    }
}

fn levitation_effect(
    registry: Registry,
    actor_query: Query<&Actor>,
    mut group_query: Query<(&Parent, &mut Counter), With<ActorSpriteGroup>>,
    mut effect_query: Query<(&Parent, &mut Visibility), With<ActorLevitationEffect>>,
) {
    for (parent, mut visibility) in effect_query.iter_mut() {
        let (group, mut counter) = group_query.get_mut(parent.get()).unwrap();
        let actor = actor_query.get(group.get()).unwrap();
        let props = registry.get_actor_props(&actor.actor_type);
        if props.auto_levitation {
            *visibility = Visibility::Hidden;
        } else if 120 < actor.levitation {
            *visibility = Visibility::Inherited;
        } else if 0 < actor.levitation {
            *visibility = if (counter.count / 4) % 2 == 0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        } else {
            *visibility = Visibility::Hidden;
            counter.count = 0;
        }
    }
}

fn apply_v(
    mut actor_query: Query<&mut Actor>,
    mut group_query: Query<(&Parent, &mut Transform, &Counter), With<ActorSpriteGroup>>,
) {
    for (parent, mut transform, counter) in group_query.iter_mut() {
        let mut actor = actor_query.get_mut(parent.get()).unwrap();
        if 0 < actor.levitation {
            // 上下の揺動が常に一番下の -1 から始まるように、cos(PI) から始めていることに注意
            let v = 6.0 + (std::f32::consts::PI + (counter.count as f32 * 0.08)).cos() * 4.0;
            transform.translation.y = v;
            actor.v = v;
            actor.gravity = 0.0;
        } else {
            actor.gravity = -0.2;
            transform.translation.y = actor.v;
        }
    }
}

fn apply_z(
    actor_query: Query<&Transform, With<Actor>>,
    mut group_query: Query<(&Parent, &mut Transform), (With<ActorSpriteGroup>, Without<Actor>)>,
) {
    for (parent, mut group_transform) in group_query.iter_mut() {
        let actor_transform = actor_query.get(parent.get()).unwrap();
        group_transform.translation.z = get_entity_z(actor_transform.translation.y);
    }
}

fn apply_damping(
    registry: Registry,
    mut query: Query<(&mut Damping, &Actor, &Transform)>,
    level: Res<GameWorld>,
) {
    let constants = registry.actor();

    for (mut damping, actor, transform) in query.iter_mut() {
        let on_ice = match level
            .get_tile_by_coords(transform.translation.truncate())
            .0
            .as_str()
        {
            "Ice" => true,
            _ => false,
        };

        let props = registry.get_actor_props(&actor.actor_type);
        damping.linear_damping = props.linear_damping
            * if 0.0 < actor.v {
                constants.dumping_on_air
            } else if on_ice {
                constants.dumping_on_ice
            } else {
                1.0
            };
    }
}

fn drown(
    mut actor_query: Query<(&mut Actor, &Transform)>,
    level: Res<GameWorld>,
    mut se: EventWriter<SEEvent>,
) {
    for (mut actor, transform) in actor_query.iter_mut() {
        if actor.levitation == 0 && actor.v == 0.0 {
            let position = transform.translation.truncate();
            let tile = level.get_tile_by_coords(position);
            if tile == Tile::new("Water") || tile == Tile::new("Lava") {
                if actor.drown == 0 {
                    se.send(SEEvent::pos(BASHA2, position));
                }
                actor.drown += 1;
            } else {
                actor.drown = 0;
            }
        } else {
            actor.drown = 0;
        }
    }
}

fn drown_damage(
    mut actor_query: Query<(
        Entity,
        &mut Actor,
        &Transform,
        Option<&Player>,
        Option<&Metamorphosed>,
    )>,
    mut damage: EventWriter<ActorEvent>,
    mut level: ResMut<GameWorld>,
    mut commands: Commands,
    mut se: EventWriter<SEEvent>,
    mut interpreter: EventWriter<InterpreterEvent>,
) {
    for (entity, mut actor, transform, player, morph) in actor_query.iter_mut() {
        let position = transform.translation.truncate();
        let tile = level.get_tile_by_coords(position);
        match tile.0.as_str() {
            "Water" => {
                if 60 < actor.drown {
                    damage.send(ActorEvent::Damaged {
                        actor: entity,
                        position: transform.translation.truncate(),
                        damage: 3,
                        fire: false,
                        impulse: Vec2::ZERO,
                        stagger: 0,
                        metamorphose: None,
                        dispel: false,
                    });
                    actor.drown = 1;
                }
            }
            "Lava" => {
                if 20 < actor.drown {
                    damage.send(ActorEvent::Damaged {
                        actor: entity,
                        position: transform.translation.truncate(),
                        damage: 10,
                        fire: false,
                        impulse: Vec2::ZERO,
                        stagger: 0,
                        metamorphose: None,
                        dispel: false,
                    });
                    actor.drown = 1;
                }
            }
            "Crack" if actor.v <= 0.0 => {
                let position = transform.translation.truncate();
                commands.entity(entity).despawn_recursive();

                se.send(SEEvent::pos(SCENE2, position));
                if let Some(player) = player {
                    recovery(&mut level, &mut interpreter, &morph, &player, &actor);
                }
            }
            _ => {}
        }
    }
}

fn stagger(registry: Registry, mut actor_query: Query<&mut Actor>) {
    for mut actor in actor_query.iter_mut() {
        let props = registry.get_actor_props(&actor.actor_type);
        if props.poise < actor.staggered {
            actor.staggered -= props.poise;
        } else {
            actor.staggered = 0;
        }
    }
}

pub fn jump_actor(
    se: &mut EventWriter<SEEvent>,

    actor: &mut Actor,
    actor_impulse: &mut ExternalImpulse,
    collision_groups: &mut CollisionGroups,
    actor_transform: &Transform,

    velocity: f32,
    impulse: f32,
) -> bool {
    if actor.v == 0.0 {
        actor.v = actor.v.max(0.01);
        actor.velocity = velocity;
        actor_impulse.impulse += actor.move_direction.normalize_or_zero() * impulse;
        *collision_groups = actor.actor_group.to_groups(actor.v, actor.drown);
        let position = actor_transform.translation.truncate();
        se.send(SEEvent::pos(SUNA, position));
        true
    } else {
        false
    }
}

pub fn get_default_actor(registry: &Registry, actor_type: &ActorType) -> Actor {
    let props = registry.get_actor_props(&actor_type);
    Actor {
        actor_type: actor_type.clone(),
        actor_group: props.actor_group,
        wands: Wand::from_vec(&props.wands),
        life: props.life,
        max_life: props.life,
        ..default()
    }
}

/// ダメージを受けた時に振動するスプライト
#[derive(Default, Component, Reflect)]
pub struct LifeBeingSprite;

fn vibrate_breakabke_sprite(
    time: Res<Time>,
    mut breakable_query: Query<(&mut Actor, &Children)>,
    mut breakable_sprite_query: Query<&mut Transform, With<LifeBeingSprite>>,
) {
    for (mut breakable, children) in breakable_query.iter_mut() {
        for child in children {
            if let Ok(mut transform) = breakable_sprite_query.get_mut(*child) {
                transform.translation.x = (time.elapsed_secs() * 56.0).sin() * breakable.amplitude;
            }
            breakable.amplitude *= 0.9;
        }
    }
}

/// 付近に炎がある場合、ダメージを受けます
/// ただし、Burnableである場合はダメージを受けませんが、その代わりに引火することがあり、
/// 引火したあとで Burnable の life がゼロになった場合はエンティティは消滅します
fn fire_damage(
    registry: Registry,
    mut actor_query: Query<(Entity, &mut Actor, &Transform), Without<Burnable>>,
    fire_query: Query<&mut Transform, (With<Fire>, Without<Actor>)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut actor_event: EventWriter<ActorEvent>,
) {
    for (actor_entity, mut actor, actor_transform) in actor_query.iter_mut() {
        let props = registry.get_actor_props(&actor.actor_type);
        if actor.fire_damage_wait <= 0 && !props.fire_resistance {
            let mut detected_fires = Vec::<Entity>::new();
            let context = rapier_context.single();

            // 各アクターは、自分の周囲に対して炎を検索します
            // 炎は多数になる可能性があることや、
            // アクターはダメージの待機時間があり大半のフレームでは判定を行わないため、
            // 炎側からアクター側へ判定を行うのではなく、アクター側から炎側へ判定を行ったほうが効率が良くなります
            context.intersections_with_shape(
                actor_transform.translation.truncate(),
                0.0,
                &Collider::ball(12.0),
                QueryFilter {
                    groups: Some(actor.actor_group.to_groups(0.0, 0)),
                    ..default()
                },
                |entity| {
                    if fire_query.contains(entity) {
                        if actor.fire_damage_wait <= 0 {
                            detected_fires.push(entity);

                            // 一度炎ダメージを受けたらそれ以上他の炎からダメージを受けることはないため、
                            // 探索を打ち切る
                            return false;
                        }
                    }
                    true // 交差図形の検索を続ける
                },
            );

            for _ in detected_fires {
                actor_event.send(ActorEvent::Damaged {
                    actor: actor_entity,
                    damage: 4,
                    position: actor_transform.translation.truncate(),
                    fire: true,
                    impulse: Vec2::ZERO,
                    stagger: 0,
                    metamorphose: None,
                    dispel: false,
                });
            }
        }

        if 0 < actor.fire_damage_wait {
            actor.fire_damage_wait -= 1;
        }
    }
}

fn damage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    registry: Registry,
    mut spawn: EventWriter<SpawnEvent>,
    mut query: Query<(
        &mut Actor,
        &Transform,
        Option<&mut ExternalImpulse>,
        Option<&Player>,
        Option<&mut Metamorphosed>,
    )>,
    mut reader: EventReader<ActorEvent>,
    mut se: EventWriter<SEEvent>,
) {
    for event in reader.read() {
        match event {
            ActorEvent::Damaged {
                actor: actor_entity,
                damage,
                position,
                fire,
                impulse,
                stagger,
                metamorphose,
                dispel,
            } => {
                let Ok((mut actor, life_transform, life_impulse, _, mut actor_metamorphosis)) =
                    query.get_mut(*actor_entity)
                else {
                    continue;
                };

                actor.life = (actor.life as i32 - *damage as i32).max(0) as u32;
                actor.staggered = (actor.staggered + stagger).min(120);

                actor.amplitude = 6.0;

                se.send(SEEvent::pos(DAMAGE, *position));

                if *fire {
                    actor.fire_damage_wait = 60 + (rand::random::<u32>() % 60);
                }

                if let Some(mut life_impulse) = life_impulse {
                    life_impulse.impulse += *impulse;
                }

                if let Some(morphing_to) = metamorphose {
                    if 0 < actor.life {
                        let position = life_transform.translation.truncate();
                        cast_metamorphosis(
                            &mut commands,
                            &asset_server,
                            &registry,
                            &mut se,
                            &mut spawn,
                            actor_entity,
                            actor.clone(),
                            &actor_metamorphosis.as_deref(),
                            position,
                            morphing_to,
                        );
                    }
                } else if let Some(ref mut actor_metamorphosis) = actor_metamorphosis {
                    if *dispel {
                        actor_metamorphosis.count = 0;
                    }
                }
            }
        }
    }
}

fn decrement_cloned(mut query: Query<&mut Actor>) {
    for mut actor in query.iter_mut() {
        if let Some(ref mut cloned) = &mut actor.cloned {
            if 0 < *cloned {
                *cloned = *cloned - 1;
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    registry: Registry,
    world: Res<GameWorld>,
    mut interpreter: EventWriter<InterpreterEvent>,
    mut se: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEvent>,
    query: Query<(
        Entity,
        &Actor,
        &Transform,
        Option<&Player>,
        Option<&Boss>,
        Option<&Burnable>,
    )>,
) {
    for (entity, actor, transform, player, boss, burnable) in query.iter() {
        let position = transform.translation.truncate();

        if actor.cloned.map(|c| c == 0).unwrap_or(false) {
            // 分身の時間切れによる消滅
            commands.entity(entity).despawn_recursive();
            se.send(SEEvent::pos(SHURIKEN, position));
            spawn.send(SpawnEvent {
                position,
                spawn: Spawn::Particle {
                    particle: metamorphosis_effect(),
                },
            });
        } else if actor.life <= 0 || burnable.map(|b| b.life <= 0).unwrap_or(false) {
            commands.entity(entity).despawn_recursive();

            let props = registry.get_actor_props(&actor.actor_type);

            // 悲鳴
            if props.cry {
                se.send(SEEvent::pos(CRY, position));
            }

            let Some(level) = world.get_level_by_position(position) else {
                continue;
            };

            // ゴールドをばらまく
            // ただしプレイヤーキャラクターのみ、極端に大量にゴールドを持っているため
            // ゴールドばらまきは行わない
            if player.is_none() {
                for _ in 0..actor.golds {
                    spawn_gold(&mut commands, &registry, &level, position);
                }
            }

            // 血痕
            // todo 溺れた場合など原因によっては血痕を残さないほうがいいかも
            if let Some(blood) = props.bloods.choose(&mut rand::thread_rng()) {
                let position = transform.translation.truncate();
                commands.spawn((
                    Name::new("blood"),
                    LevelScoped(level.clone()),
                    StateScoped(GameState::InGame),
                    AseSpriteSlice {
                        aseprite: registry.assets.atlas.clone(),
                        name: blood.clone(),
                    },
                    Transform::from_translation(position.extend(BLOOD_LAYER_Z))
                        .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                ));
            }

            // ボス用の消滅シナリオ実行
            if let Some(boss) = boss {
                let mut cmds = registry.get_senario(&boss.on_despawn).clone();
                cmds.insert(
                    0,
                    Cmd::Set {
                        name: "position".to_string(),
                        value: Value::Vec2 {
                            x: position.x,
                            y: position.y,
                        },
                    },
                );
                interpreter.send(InterpreterEvent::Play { commands: cmds });
            }
        }
    }
}

fn count_up_broken_chests(mut player_0query: Query<&mut Player>, actor_query: Query<&Actor>) {
    if let Ok(mut player) = player_0query.get_single_mut() {
        for actor in actor_query.iter() {
            if actor.life == 0 {
                player.broken_chests += 1;
            }
        }
    }
}

// fn add_life_bar(
//     mut commands: Commands,
//     query: Query<
//         (
//             Entity,
//             &Actor,
//             Option<&Player>,
//             Option<&Metamorphosed>,
//             Option<&Boss>,
//         ),
//         Added<Actor>,
//     >,
//     life_bar_resource: Res<LifeBarResource>,
// ) {
//     for (entity, actor, player, morphed, boss) in query.iter() {
//         if player.is_none()
//             && morphed.is_none()
//             && boss.is_none()
//             && actor.actor_group != ActorGroup::Entity
//         {
//             commands.entity(entity).with_children(|spawn_children| {
//                 spawn_life_bar(spawn_children, &life_bar_resource);
//             });
//         }
//     }
// }

fn getting_up(mut query: Query<&mut Actor>) {
    for mut actor in query.iter_mut() {
        if 0 < actor.getting_up {
            actor.getting_up -= 1;
        }
    }
}

/// 指定した位置にアクターの実体を生成します
pub fn spawn_actor(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    registry: &Registry,
    position: Vec2,
    mut actor: Actor,
) -> Entity {
    let actor_type = actor.actor_type.clone();
    let actor_group = actor.actor_group;
    let props = registry.get_actor_props(&actor.actor_type);
    let v = actor.v;
    actor.home_position = position;

    let mut builder = commands.spawn((
        Name::new(format!("{:?}", &actor.actor_type)),
        actor,
        Transform::from_translation(position.extend(0.0)),
        Counter::default(),
        match props.collider {
            ActorCollider::Ball(radius) => (
                Collider::ball(radius),
                // PrimitiveObstacle::Circle(Circle::new(radius)),
            ),
            ActorCollider::Cuboid(width, height) => (
                Collider::cuboid(width, height),
                // PrimitiveObstacle::Rectangle(Rectangle::new(width * 2.0, height * 2.0)),
            ),
        },
        actor_group.to_groups(0.0, 0),
        PointLight2d {
            radius: 0.0,
            intensity: 0.0,
            ..default()
        },
    ));

    if actor_group == ActorGroup::Entity {
        let scale = 1.0;
        builder.insert(match props.collider {
            ActorCollider::Ball(radius) => {
                (PrimitiveObstacle::Circle(Circle::new(radius * scale)),)
            }
            ActorCollider::Cuboid(width, height) => (PrimitiveObstacle::Rectangle(Rectangle::new(
                width * 2.0 * scale,
                height * 2.0 * scale,
            )),),
        });
    }

    builder.with_children(|mut parent| {
        // 影
        if let Some(shadow) = &props.shadow {
            parent.spawn((
                AseSpriteSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: shadow.clone(),
                },
                Transform::from_xyz(0.0, 0.0, SHADOW_LAYER_Z),
            ));
        }

        spawn_life_bar(&mut parent, &registry.life_bar_resource);

        parent
            .spawn((ActorSpriteGroup, Transform::from_xyz(0.0, v, 0.0)))
            .with_children(|parent| {
                // 浮遊効果の輪
                parent.spawn((
                    ActorLevitationEffect,
                    AseSpriteSlice {
                        aseprite: registry.assets.atlas.clone(),
                        name: "levitation".into(),
                    },
                    Transform::from_xyz(0.0, 0.0, -0.0002),
                ));

                // 本体
                parent.spawn((
                    ActorAppearanceSprite,
                    CounterAnimated,
                    AseSpriteAnimation {
                        aseprite: asset_server.load(props.aseprite.clone()),
                        animation: Animation::default().with_tag(props.animations.idle_r.clone()),
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));

                if actor_type == ActorType::new("Witch") {
                    parent.spawn((
                        WitchWandSprite,
                        AseSpriteSlice {
                            aseprite: registry.assets.atlas.clone(),
                            name: "wand_cypress".into(),
                        },
                        Transform::from_xyz(0.0, 4.0, -0.0001),
                    ));
                }
            });
    });

    if actor_type == ActorType::new("Witch") {
        builder.insert((
            // 足音
            // footsteps.rsで音量を調整
            AudioPlayer::new(registry.assets.taiikukan.clone()),
            PlaybackSettings {
                volume: Volume::new(0.0),
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            },
        ));
    }

    // if let Some(owner) = master {
    //     builder.insert(Servant { master: owner });
    // }

    match actor_type.0.as_str() {
        "HugeSlime" => {
            builder.insert(HugeSlime {
                state: HugeSlimeState::Growl,
                promoted: false,
            });
        }
        "Chicken" => {
            builder.insert(Chicken::default());
        }
        "Chest" => {
            builder.insert((Chest::random(), Burnable { life: 30 }));
        }
        "Bookshelf" => {
            builder.insert(Burnable { life: 30 });
        }
        "Bomb" => {
            builder.insert(Bomb);
        }
        _ => {}
    }

    builder.id()
}

/// frozen, staggerd, run, idle のみからなる基本的なアニメーションを実装します
/// これ以外の表現が必要な場合は各アクターで個別に実装して上書きします
pub fn basic_animate(
    query: Query<&Actor>,
    registry: Registry,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<
        (&Parent, &mut Sprite, &mut AseSpriteAnimation),
        With<ActorAppearanceSprite>,
    >,
) {
    for (parent, mut sprite, mut animation) in sprite_query.iter_mut() {
        if let Ok(group) = group_query.get(parent.get()) {
            if let Ok(actor) = query.get(group.get()) {
                let props = registry.get_actor_props(&actor.actor_type);

                let angle = actor.pointer.to_angle();
                let pi = std::f32::consts::PI;

                animation.animation.repeat = AnimationRepeat::Loop;
                animation.animation.tag = Some(if 0 < actor.frozen {
                    props.animations.frozen.clone()
                } else if 0 < actor.drown {
                    props.animations.drown.clone()
                } else if 0 < actor.staggered {
                    props.animations.staggered.clone()
                } else if 0 < actor.getting_up {
                    props.animations.get_up.clone()
                } else {
                    match actor.state {
                        ActorState::Idle => {
                            if angle < pi * -0.75 || pi * 0.75 < angle {
                                props.animations.idle_r.clone()
                            } else if pi * 0.25 < angle && angle < pi * 0.75 {
                                props.animations.idle_u.clone()
                            } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                                props.animations.idle_d.clone()
                            } else {
                                props.animations.idle_r.clone()
                            }
                        }
                        ActorState::Run => {
                            if angle < pi * -0.75 || pi * 0.75 < angle {
                                props.animations.run_r.clone()
                            } else if pi * 0.25 < angle && angle < pi * 0.75 {
                                props.animations.run_u.clone()
                            } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                                props.animations.run_d.clone()
                            } else {
                                props.animations.run_r.clone()
                            }
                        }
                    }
                });

                sprite.flip_x = match actor.state {
                    ActorState::Idle => {
                        if angle < pi * -0.75 || pi * 0.75 < angle {
                            true
                        } else if pi * 0.25 < angle && angle < pi * 0.75 {
                            false
                        } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                            false
                        } else {
                            false
                        }
                    }
                    ActorState::Run => {
                        if angle < pi * -0.75 || pi * 0.75 < angle {
                            true
                        } else if pi * 0.25 < angle && angle < pi * 0.75 {
                            false
                        } else if pi * -0.75 <= angle && angle <= pi * -0.25 {
                            false
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

fn flip(
    actor_query: Query<&Actor>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<
        (&Parent, &mut Sprite),
        (With<ActorAppearanceSprite>, Without<ActorSpriteGroup>),
    >,
) {
    for (parent, mut sprite) in sprite_query.iter_mut() {
        let parent = group_query.get(parent.get()).unwrap();
        let chicken = actor_query.get(parent.get()).unwrap();
        sprite.flip_x = chicken.pointer.x < 0.0;
    }
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>();
        app.add_event::<ActorEvent>();
        app.add_systems(
            FixedUpdate,
            (
                // 操作
                (
                    apply_external_force,
                    fire_bullet,
                    collision_group_by_actor,
                    apply_damping,
                ),
                // 状態異常
                (
                    drown,
                    drown_damage,
                    stagger,
                    damage,
                    fire_damage,
                    despawn,
                    decrement_cloned,
                    defreeze,
                    (decrement_levitation, levitation_effect).chain(),
                ),
                // 外観
                (
                    apply_v,
                    apply_z,
                    update_sprite_flip,
                    update_actor_light,
                    getting_up,
                    basic_animate,
                    flip,
                    vibrate_breakabke_sprite,
                    count_up_broken_chests,
                ),
            )
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
