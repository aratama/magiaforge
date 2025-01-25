pub mod bomb;
pub mod book_shelf;
pub mod chest;
pub mod chicken;
pub mod rabbit;
pub mod rock;
pub mod sandbug;
pub mod stone_lantern;
pub mod witch;

use crate::actor::chicken::default_chiken;
use crate::actor::sandbug::default_sandbag;
use crate::asset::GameAssets;
use crate::cast::cast_spell;
use crate::collision::ENEMY_BULLET_GROUP;
use crate::collision::ENEMY_GROUPS;
use crate::collision::FLYING_ENEMY_GROUPS;
use crate::collision::FLYING_NEUTRAL_GROUPS;
use crate::collision::FLYING_PLAYER_GROUPS;
use crate::collision::NEUTRAL_GROUPS;
use crate::collision::PLAYER_BULLET_GROUP;
use crate::collision::PLAYER_GROUPS;
use crate::component::counter::Counter;
use crate::component::entity_depth::get_entity_z;
use crate::component::entity_depth::ChildEntityDepth;
use crate::component::metamorphosis::cast_metamorphosis;
use crate::component::metamorphosis::metamorphosis_effect;
use crate::component::metamorphosis::Metamorphosed;
use crate::component::vertical::Vertical;
use crate::constant::BLOOD_LAYER_Z;
use crate::constant::MAX_WANDS;
use crate::constant::TILE_SIZE;
use crate::controller::player::recovery;
use crate::controller::player::Player;
use crate::controller::player::PlayerControlled;
use crate::enemy::eyeball::default_eyeball;
use crate::enemy::huge_slime::default_huge_slime;
use crate::enemy::salamander::default_salamander;
use crate::enemy::shadow::default_shadow;
use crate::enemy::slime::default_slime;
use crate::enemy::spider::default_spider;
use crate::entity::bullet::Trigger;
use crate::entity::fire::Burnable;
use crate::entity::fire::Fire;
use crate::entity::gold::spawn_gold;
use crate::entity::impact::SpawnImpact;
use crate::hud::life_bar::LifeBarResource;
use crate::interpreter::InterpreterEvent;
use crate::inventory::Inventory;
use crate::inventory_item::InventoryItemType;
use crate::level::entities::add_default_behavior;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::Spell;
use crate::states::GameState;
use crate::ui::floating::FloatingContent;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::ExternalForce;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_rapier2d::prelude::Group;
use bevy_rapier2d::prelude::QueryFilter;
use bevy_rapier2d::prelude::Velocity;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;
use bomb::default_bomb;
use book_shelf::default_bookshelf;
use chest::default_random_chest;
use chest::ChestItem;
use chest::ChestType;
use rabbit::default_rabbit;
use rabbit::RabbitType;
use rock::default_rock;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::f32::consts::PI;
use stone_lantern::default_lantern;
use uuid::Uuid;
use witch::default_witch;

/// アクターの種類を表します
/// registry.actor.ron で種類ごとに移動速度やジャンプ力などが設定されます
#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorType {
    Witch,
    Chicken,
    EyeBall,
    Sandbag,
    Slime,
    HugeSlime,
    Salamander,
    Shadow,
    Spider,
    Lantern,
    Chest,
    BookShelf,
    Rabbit,
    Rock,
    Bomb,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Deserialize)]
pub enum Blood {
    Red,
    Blue,
}

fn state_scoped_in_game() -> StateScoped<GameState> {
    StateScoped(GameState::InGame)
}

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
#[require(Vertical, StateScoped<GameState>(state_scoped_in_game), Visibility)]
pub struct Actor {
    /// このアクターの種族を表すとともに、種族特有の情報を格納します
    pub extra: ActorExtra,

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

    pub blood: Option<Blood>,

    // 定数 ////////////////////////////////////////////////////////////////////////////////////////////////
    /// 1フレームあたりの stagger の回復速度です
    pub poise: u32,

    pub invincibility_on_staggered: bool,

    pub point_light_radius: f32,

    /// 蜘蛛の巣から逃れる速度
    /// 毎ターンこの値が trapped から減算され、trappedが0になるとアクターが解放されます
    /// また、解放された瞬間に trap_moratorium が 180 に設定され、
    /// 3秒間は再びトラップにかからないようになります
    pub floundering: u32,

    pub fire_resistance: bool,

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
    pub drowning: u32,

    /// 複製された場合の残り時間
    pub cloned: Option<u32>,
}

impl Actor {
    pub fn to_type(&self) -> ActorType {
        self.extra.to_type()
    }
}

/// Actorで種族固有の部分を格納します
#[derive(Clone, Debug, Reflect, Deserialize)]
pub enum ActorExtra {
    Witch,
    Slime,
    HugeSlime,
    Eyeball,
    Shadow,
    Spider,
    Salamander,
    Chicken,
    Sandbag,
    Lantern,
    Chest {
        chest_type: ChestType,
        chest_item: ChestItem,
    },
    BookShelf,
    Rabbit {
        rabbit_type: RabbitType,
    },
    Rock,
    Bomb,
}

impl ActorExtra {
    pub fn to_type(&self) -> ActorType {
        match self {
            ActorExtra::Witch { .. } => ActorType::Witch,
            ActorExtra::Chicken => ActorType::Chicken,
            ActorExtra::Eyeball => ActorType::EyeBall,
            ActorExtra::Sandbag => ActorType::Sandbag,
            ActorExtra::Slime => ActorType::Slime,
            ActorExtra::HugeSlime => ActorType::HugeSlime,
            ActorExtra::Shadow => ActorType::Shadow,
            ActorExtra::Spider => ActorType::Spider,
            ActorExtra::Salamander => ActorType::Salamander,
            ActorExtra::Lantern => ActorType::Lantern,
            ActorExtra::Chest { .. } => ActorType::Chest,
            ActorExtra::BookShelf => ActorType::BookShelf,
            ActorExtra::Rabbit { .. } => ActorType::Rabbit,
            ActorExtra::Rock => ActorType::Rock,
            ActorExtra::Bomb => ActorType::Rock,
        }
    }
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
#[require(
    Visibility,
    Counter,
    Transform,
    LifeBeingSprite,
    ChildEntityDepth,
    Vertical
)]
pub struct ActorSpriteGroup;

impl Actor {
    #[allow(dead_code)]
    pub fn get_item_icon(&self, registry: Registry, index: FloatingContent) -> Option<String> {
        match index {
            FloatingContent::Inventory(index) => self
                .inventory
                .get(index)
                .as_ref()
                .map(|i| i.item_type.get_icon(&registry)),
            FloatingContent::WandSpell(w, s) => {
                self.wands[w].slots[s]
                    .as_ref()
                    .map(|WandSpell { ref spell_type, .. }| {
                        registry.get_spell_props(spell_type).icon.clone()
                    })
            }
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
        let props = registry.get_actor_props(self.to_type());
        let mut force = props.move_force;

        //todo

        for wand in self.wands.iter() {
            for slot in &wand.slots {
                force += match slot {
                    Some(WandSpell { spell_type, .. })
                        if *spell_type == Spell::new("SpikeBoots") =>
                    {
                        40000.0
                    }
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
            for slot in &wand.slots {
                scale_factor += match slot {
                    Some(WandSpell { spell_type, .. })
                        if *spell_type == Spell::new("Telescope") =>
                    {
                        0.5
                    }
                    Some(WandSpell { spell_type, .. })
                        if *spell_type == Spell::new("Magnifier") =>
                    {
                        -0.5
                    }
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
                match &item.item_type {
                    InventoryItemType::Spell(spell) if item.price == 0 => {
                        let _ = discovered_spells.insert(spell.clone());
                    }
                    _ => {}
                };
            }
        }
        for wand in self.wands.iter() {
            for item in wand.slots.iter() {
                if let Some(ref item) = &item {
                    if item.price == 0 {
                        let _ = discovered_spells.insert(item.spell_type.clone());
                    }
                }
            }
        }
        discovered_spells
    }
}

impl Default for Actor {
    fn default() -> Self {
        let default_life = 100;
        Actor {
            uuid: Uuid::new_v4(),
            life: default_life,
            max_life: default_life,
            amplitude: 0.0,
            fire_damage_wait: 0,
            pointer: Vec2::ZERO,
            point_light_radius: 0.0,
            current_wand: 0,
            actor_group: ActorGroup::Neutral,
            golds: 0,
            wands: [
                Wand::default(),
                Wand::default(),
                Wand::default(),
                Wand::default(),
            ],
            blood: None,
            inventory: Inventory::new(),
            fire_resistance: false,
            move_direction: Vec2::ZERO,
            fire_state: ActorFireState::Idle,
            fire_state_secondary: ActorFireState::Idle,
            effects: default(),
            state: ActorState::default(),
            wait: 30,
            trapped: 0,
            trap_moratorium: 0,
            floundering: 1,
            frozen: 0,
            levitation: 0,
            drowning: 0,
            staggered: 0,
            poise: 1,
            invincibility_on_staggered: false,
            extra: ActorExtra::Chicken,
            cloned: None,
        }
    }
}

#[derive(Reflect, Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum ActorFireState {
    Idle,

    /// Actorのpointerに向かって弾丸を発射します
    Fire,
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorGroup {
    Friend,
    Enemy,

    /// 中立のアクターです
    /// 敵と味方の両方の攻撃を受け、敵から攻撃対象として狙われます
    Neutral,

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

#[derive(Component)]
pub struct ActorLight {
    owner: Entity,
}

fn update_actor_light(
    mut commands: Commands,
    mut light_query: Query<(Entity, &ActorLight, &mut PointLight2d, &mut Transform)>,
    mut actor_query: Query<(Entity, &mut Actor, &Transform, Option<&Player>), Without<ActorLight>>,
) {
    for (actor_entity, mut actor, transform, _) in actor_query.iter_mut() {
        // 光源の明るさ(というか半径)を計算
        let mut point_light_radius: f32 = 0.0;

        // 複製されたアクターは光源を0とする
        // プレイヤーキャラクターは明るい光源を装備していることが多く、
        // 大量に複製すると明るくなりすぎるため
        if actor.cloned.is_none() {
            for wand in actor.wands.iter() {
                for slot in &wand.slots {
                    match slot {
                        Some(WandSpell { spell_type, .. })
                            if *spell_type == Spell::new("Lantern") =>
                        {
                            point_light_radius += 160.0;
                        }
                        _ => {}
                    }
                }
            }
        }

        actor.point_light_radius = point_light_radius;

        // 光源がないアクターに光源を追加
        if 0.0 < point_light_radius {
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
    }

    // 光源の明るさを更新
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
    registry: Registry,
    life_bar_resource: Res<LifeBarResource>,
    level: Res<LevelSetup>,
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
            &mut Vertical,
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
        mut actor_falling,
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
            let wand = if actor_metamorphosis.is_none() {
                actor.current_wand
            } else {
                0
            };
            cast_spell(
                &mut commands,
                &registry,
                &life_bar_resource,
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
                &mut actor_falling,
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
                &registry,
                &life_bar_resource,
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
                &mut actor_falling,
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
    mut query: Query<(&mut Actor, &mut ExternalForce, &Transform, &Vertical)>,
    mut se: EventWriter<SEEvent>,
    level: Res<LevelSetup>,
) {
    let constants = registry.actor();

    if let Some(ref chunk) = level.chunk {
        for (mut actor, mut external_force, transform, vertical) in query.iter_mut() {
            let position = transform.translation.truncate();

            let on_ice = match chunk.get_tile_by_coords(position) {
                Tile::Ice => true,
                _ => false,
            };

            let ratio = if 0 < actor.frozen {
                0.0
            } else if 0 < actor.staggered {
                0.0
            } else if 0 < actor.drowning {
                constants.acceleration_on_drowning
            } else if 0.0 < vertical.v {
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
                    actor.trapped -= actor.floundering;
                    if actor.trapped == 0 {
                        actor.trap_moratorium = 180;
                        se.send(SEEvent::pos(SE::Zombie, position));
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
}

fn defreeze(registry: Registry, mut query: Query<&mut Actor>) {
    for mut actor in query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());
        if props.defreeze <= actor.frozen {
            actor.frozen -= props.defreeze;
        } else if 0 < actor.frozen {
            actor.frozen = 0;
        }
    }
}

pub fn collision_group_by_actor(mut query: Query<(&Actor, &Vertical, &mut CollisionGroups)>) {
    for (actor, vertical, mut groups) in query.iter_mut() {
        *groups = actor.actor_group.to_groups(vertical.v, actor.drowning);
    }
}

fn decrement_levitation(mut actor_query: Query<&mut Actor>, registry: Registry) {
    for mut actor in actor_query.iter_mut() {
        let props = registry.get_actor_props(actor.to_type());
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
        let props = registry.get_actor_props(actor.to_type());
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

fn add_levitation_effect(
    mut commands: Commands,
    assets: Res<GameAssets>,
    query: Query<Entity, Added<ActorSpriteGroup>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_child((
            ActorLevitationEffect,
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "levitation".into(),
            },
            Transform::from_xyz(0.0, -4.0, -0.0002),
        ));
    }
}

fn apply_v(
    mut actor_query: Query<(&Actor, &mut Vertical)>,
    mut group_query: Query<(&Parent, &mut Transform, &Counter), With<ActorSpriteGroup>>,
) {
    for (parent, mut transform, counter) in group_query.iter_mut() {
        let (actor, mut vertical) = actor_query.get_mut(parent.get()).unwrap();
        if 0 < actor.levitation {
            // 上下の揺動が常に一番下の -1 から始まるように、cos(PI) から始めていることに注意
            let v = 6.0 + (std::f32::consts::PI + (counter.count as f32 * 0.08)).cos() * 4.0;
            transform.translation.y = v;
            vertical.v = v;
            vertical.gravity = 0.0;
        } else {
            vertical.gravity = -0.2;
            transform.translation.y = vertical.v;
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
    mut query: Query<(&Vertical, &mut Damping, &Actor, &Transform)>,
    level: Res<LevelSetup>,
) {
    let constants = registry.actor();

    if let Some(ref chunk) = level.chunk {
        for (vertical, mut damping, actor, transform) in query.iter_mut() {
            let on_ice = match chunk.get_tile_by_coords(transform.translation.truncate()) {
                Tile::Ice => true,
                _ => false,
            };

            let props = registry.get_actor_props(actor.to_type());
            damping.linear_damping = props.linear_damping
                * if 0.0 < vertical.v {
                    constants.dumping_on_air
                } else if on_ice {
                    constants.dumping_on_ice
                } else {
                    1.0
                };
        }
    }
}

fn drown(
    mut actor_query: Query<(&mut Actor, &Vertical, &Transform)>,
    level: Res<LevelSetup>,
    mut se: EventWriter<SEEvent>,
) {
    if let Some(ref chunk) = level.chunk {
        for (mut actor, vertical, transform) in actor_query.iter_mut() {
            if actor.levitation == 0 && vertical.v == 0.0 {
                let position = transform.translation.truncate();
                let tile = chunk.get_tile_by_coords(position);
                if tile == Tile::Water || tile == Tile::Lava {
                    if actor.drowning == 0 {
                        se.send(SEEvent::pos(SE::Basha2, position));
                    }
                    actor.drowning += 1;
                } else {
                    actor.drowning = 0;
                }
            } else {
                actor.drowning = 0;
            }
        }
    }
}

fn drown_damage(
    mut actor_query: Query<(
        Entity,
        &mut Actor,
        &Transform,
        &Vertical,
        Option<&Player>,
        Option<&Metamorphosed>,
    )>,
    mut damage: EventWriter<ActorEvent>,
    mut level: ResMut<LevelSetup>,
    mut commands: Commands,
    mut se: EventWriter<SEEvent>,
    mut interpreter: EventWriter<InterpreterEvent>,
) {
    for (entity, mut actor, transform, vertical, player, morph) in actor_query.iter_mut() {
        let position = transform.translation.truncate();
        let tile = if let Some(ref chunk) = level.chunk {
            chunk.get_tile_by_coords(position)
        } else {
            Tile::Blank
        };
        match tile {
            Tile::Water => {
                if 60 < actor.drowning {
                    damage.send(ActorEvent::Damaged {
                        actor: entity,
                        position: transform.translation.truncate(),
                        damage: 1,
                        fire: false,
                        impulse: Vec2::ZERO,
                        stagger: 0,
                        metamorphose: None,
                        dispel: false,
                    });
                    actor.drowning = 1;
                }
            }
            Tile::Lava => {
                if 20 < actor.drowning {
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
                    actor.drowning = 1;
                }
            }
            Tile::Crack if vertical.v <= 0.0 => {
                let position = transform.translation.truncate();
                commands.entity(entity).despawn_recursive();

                se.send(SEEvent::pos(SE::Scene2, position));
                if let Some(player) = player {
                    recovery(&mut level, &mut interpreter, &morph, &player, &actor);
                }
            }
            _ => {}
        }
    }
}

fn stagger(mut actor_query: Query<&mut Actor>) {
    for mut actor in actor_query.iter_mut() {
        if actor.poise < actor.staggered {
            actor.staggered -= actor.poise;
        } else {
            actor.staggered = 0;
        }
    }
}

pub fn jump_actor(
    se: &mut EventWriter<SEEvent>,

    actor: &mut Actor,
    actor_falling: &mut Vertical,
    actor_impulse: &mut ExternalImpulse,
    collision_groups: &mut CollisionGroups,
    actor_transform: &Transform,

    velocity: f32,
    impulse: f32,
) -> bool {
    if actor_falling.v == 0.0 {
        actor_falling.v = actor_falling.v.max(0.01);
        actor_falling.velocity = velocity;
        actor_impulse.impulse += actor.move_direction.normalize_or_zero() * impulse;
        *collision_groups = actor.actor_group.to_groups(actor_falling.v, actor.drowning);
        let position = actor_transform.translation.truncate();
        se.send(SEEvent::pos(SE::Suna, position));
        true
    } else {
        false
    }
}

pub fn get_default_actor(actor_type: ActorType) -> Actor {
    match actor_type {
        ActorType::Witch => default_witch(),
        ActorType::HugeSlime => default_huge_slime(),
        ActorType::Slime => default_slime(),
        ActorType::EyeBall => default_eyeball(),
        ActorType::Shadow => default_shadow(),
        ActorType::Spider => default_spider(),
        ActorType::Salamander => default_salamander(),
        ActorType::Chicken => default_chiken(),
        ActorType::Sandbag => default_sandbag(),
        ActorType::Lantern => default_lantern(),
        ActorType::Chest => default_random_chest(),
        ActorType::BookShelf => default_bookshelf(),
        ActorType::Rabbit => default_rabbit(RabbitType::Guide),
        ActorType::Rock => default_rock(),
        ActorType::Bomb => default_bomb(),
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
    mut actor_query: Query<(Entity, &mut Actor, &Transform), Without<Burnable>>,
    fire_query: Query<&mut Transform, (With<Fire>, Without<Actor>)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut actor_event: EventWriter<ActorEvent>,
) {
    for (actor_entity, mut actor, actor_transform) in actor_query.iter_mut() {
        if actor.fire_damage_wait <= 0 && !actor.fire_resistance {
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
    registry: Registry,
    life_bar_resource: Res<LifeBarResource>,
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

                if actor.staggered == 0 || !actor.invincibility_on_staggered {
                    actor.life = (actor.life as i32 - *damage as i32).max(0) as u32;
                    actor.staggered = (actor.staggered + stagger).min(120);
                }

                actor.amplitude = 6.0;

                se.send(SEEvent::pos(SE::Damage, *position));

                if *fire {
                    actor.fire_damage_wait = 60 + (rand::random::<u32>() % 60);
                }

                if let Some(mut life_impulse) = life_impulse {
                    life_impulse.impulse += *impulse;
                }

                if let Some(morphing_to) = metamorphose {
                    if 0 < actor.life {
                        let position = life_transform.translation.truncate();
                        let entity = cast_metamorphosis(
                            &mut commands,
                            &registry,
                            &life_bar_resource,
                            &mut se,
                            &mut spawn,
                            actor_entity,
                            actor.clone(),
                            &actor_metamorphosis.as_deref(),
                            position,
                            *morphing_to,
                        );
                        add_default_behavior(&mut commands, *morphing_to, position, entity);
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
    mut se: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEvent>,
    query: Query<(Entity, &Actor, &Transform, Option<&Player>)>,
) {
    for (entity, actor, transform, player) in query.iter() {
        let position = transform.translation.truncate();

        if actor.cloned.map(|c| c == 0).unwrap_or(false) {
            // 分身の時間切れによる消滅
            commands.entity(entity).despawn_recursive();
            se.send(SEEvent::pos(SE::Shuriken, position));
            spawn.send(SpawnEvent {
                position,
                entity: Spawn::Particle {
                    particle: metamorphosis_effect(),
                },
            });
        } else if actor.life <= 0 {
            commands.entity(entity).despawn_recursive();

            let props = registry.get_actor_props(actor.to_type());

            // 悲鳴
            if props.cry {
                se.send(SEEvent::pos(SE::Cry, position));
            }

            // ゴールドをばらまく
            // ただしプレイヤーキャラクターのみ、極端に大量にゴールドを持っているため
            // ゴールドばらまきは行わない
            if player.is_none() {
                for _ in 0..actor.golds {
                    spawn_gold(&mut commands, &registry, position);
                }
            }

            // 血痕
            // todo 溺れた場合など原因によっては血痕を残さないほうがいいかも
            if let Some(ref blood) = props.blood {
                let position = transform.translation.truncate();
                commands.spawn((
                    Name::new("blood"),
                    StateScoped(GameState::InGame),
                    AseSpriteSlice {
                        aseprite: registry.assets.atlas.clone(),
                        name: match blood {
                            Blood::Red => format!("blood_{}", rand::random::<u8>() % 3),
                            Blood::Blue => format!("slime_blood_{}", rand::random::<u8>() % 3),
                        },
                    },
                    Transform::from_translation(position.extend(BLOOD_LAYER_Z))
                        .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                ));
            }
        }
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
                update_sprite_flip,
                update_actor_light,
                apply_external_force,
                fire_bullet,
                defreeze,
                collision_group_by_actor,
                (
                    add_levitation_effect,
                    decrement_levitation,
                    levitation_effect,
                )
                    .chain(),
                apply_v,
                apply_z,
                apply_damping,
                drown,
                drown_damage,
                stagger,
                damage,
                vibrate_breakabke_sprite,
                fire_damage,
                decrement_cloned,
                despawn,
            )
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
