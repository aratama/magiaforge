pub mod book_shelf;
pub mod chest;
pub mod chicken;
pub mod rabbit;
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
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::component::metamorphosis::Metamorphosed;
use crate::component::vertical::Vertical;
use crate::constant::ActorProps;
use crate::constant::GameActors;
use crate::constant::GameConstants;
use crate::constant::MAX_WANDS;
use crate::constant::TILE_SIZE;
use crate::controller::player::recovery;
use crate::controller::player::Player;
use crate::enemy::eyeball::default_eyeball;
use crate::enemy::huge_slime::default_huge_slime;
use crate::enemy::salamander::default_salamander;
use crate::enemy::shadow::default_shadow;
use crate::enemy::slime::default_slime;
use crate::enemy::spider::default_spider;
use crate::entity::bullet::Trigger;
use crate::entity::impact::SpawnImpact;
use crate::hud::life_bar::LifeBarResource;
use crate::interpreter::InterpreterEvent;
use crate::inventory::Inventory;
use crate::inventory_item::InventoryItemType;
use crate::level::entities::SpawnEntity;
use crate::level::entities::SpawnWitchType;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::ui::floating::FloatingContent;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::ExternalForce;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_rapier2d::prelude::Group;
use bevy_simple_websocket::ClientMessage;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;
use book_shelf::default_bookshelf;
use chest::default_random_chest;
use chest::ChestItem;
use chest::ChestType;
use rabbit::default_rabbit;
use rabbit::RabbitType;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::f32::consts::PI;
use stone_lantern::default_lantern;
use uuid::Uuid;
use witch::default_witch;

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
}

impl ActorType {
    pub fn to_props<'a>(&self, constants: &'a GameActors) -> &'a ActorProps {
        let name = format!("{:?}", self);
        &constants
            .actors
            .get(&format!("{:?}", self))
            .expect(format!("ActorType {:?} not found", name).as_str())
    }
}

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

    pub metamorphse: Option<ActorType>,

    pub dispel: u8,
}

#[derive(Reflect, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorState {
    #[default]
    Idle,
    Run,
    GettingUp,
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
#[derive(Component, Reflect, Debug, Clone)]
#[require(Vertical)]
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
    pub fire_state: ActorFireState,

    pub fire_state_secondary: ActorFireState,

    pub current_wand: u8,

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

    // アクター特性 ///////////////////////////////////////////////////////////////////////////////////////

    // 凍結から復帰する速度です
    // 通常は 1 ですが、ボスなどの特定のモンスターはより大きい値になることがあります
    pub defreeze: u32,

    // 常時浮遊のモンスターであることを表します
    // 通常は false ですが、アイボールなどの一部のモンスターは true になります
    pub auto_levitation: bool,

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

    /// 蜘蛛の巣から逃れる速度
    /// 毎ターンこの値が trapped から減算され、trappedが0になるとアクターが解放されます
    /// また、解放された瞬間に trap_moratorium が 180 に設定され、
    /// 3秒間は再びトラップにかからないようになります
    pub floundering: u32,

    pub fire_resistance: bool,

    /// 1フレームあたりの stagger の回復速度です
    pub poise: u32,

    pub invincibility_on_staggered: bool,

    // アクター状態異常 ////////////////////////////////////////////////////////////////////////////////////////////
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

    pub extra: ActorExtra,
}

impl Actor {
    pub fn to_type(&self) -> ActorType {
        self.extra.to_type()
    }
    pub fn to_props<'a>(&self, constants: &'a GameActors) -> &'a ActorProps {
        self.to_type().to_props(constants)
    }
}

/// Actorで種族固有の部分を格納します
#[derive(Clone, Debug, Reflect)]
pub enum ActorExtra {
    Witch {
        witch_type: SpawnWitchType,
        getting_up: bool,
        name: String,
        discovered_spells: HashSet<SpellType>,
    },
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
        }
    }
}

#[derive(Default, Component, Reflect)]
#[require(Visibility(||Visibility::Hidden), Transform)]
pub struct ActorLevitationEffect;

/// Actor のスプライトをまとめる子エンティティのマーカーです
/// ボスキャラクターなどの一部のActorはこのマーカーを使いませんが、
/// 通常のキャラクターはこのマーカーでスプライトをまとめます
/// このマーカーを使うと、その子には浮遊エフェクトの子が追加され、浮遊魔法での浮遊アニメーションが描画されるようになります
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
    pub fn get_item_icon(
        &self,
        constants: &GameConstants,
        index: FloatingContent,
    ) -> Option<String> {
        match index {
            FloatingContent::Inventory(index) => self
                .inventory
                .get(index)
                .map(|i| i.item_type.get_icon(&constants)),
            FloatingContent::WandSpell(w, s) => self.wands[w].slots[s]
                .map(|spell| spell.spell_type.to_props(&constants).icon.clone()),
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
    fn get_total_move_force(&self, constants: &GameActors) -> f32 {
        let mut force = self.to_props(&constants).move_force;

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

impl Default for Actor {
    fn default() -> Self {
        Actor {
            uuid: Uuid::new_v4(),
            pointer: Vec2::ZERO,
            point_light_radius: 0.0,
            radius: 8.0,
            current_wand: 0,
            actor_group: ActorGroup::Neutral,
            golds: 0,
            wands: [Wand::empty(), Wand::empty(), Wand::empty(), Wand::empty()],
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
            defreeze: 1,
            levitation: 0,
            auto_levitation: false,
            drowning: 0,
            staggered: 0,
            poise: 1,
            invincibility_on_staggered: false,
            extra: ActorExtra::Chicken,
        }
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
            // info!("despawn {} {}", file!(), line!());
        }
    }
}

/// 攻撃状態にあるアクターがスペルを詠唱します
fn fire_bullet(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ron: Res<Assets<GameConstants>>,
    life_bar_resource: Res<LifeBarResource>,
    mut actor_query: Query<
        (
            Entity,
            &mut Actor,
            &mut Life,
            &mut Transform,
            &mut ExternalImpulse,
            &mut Vertical,
            &mut CollisionGroups,
            Option<&Player>,
            Option<&Metamorphosed>,
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

    let constants = ron.get(assets.spells.id()).unwrap();

    for (
        actor_entity,
        mut actor,
        mut actor_life,
        actor_transform,
        mut actor_impulse,
        mut actor_falling,
        mut collision_groups,
        player,
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
                &assets,
                &constants,
                &life_bar_resource,
                actor_entity,
                &mut actor,
                &mut actor_life,
                &actor_transform,
                &mut actor_impulse,
                &mut actor_falling,
                &mut collision_groups,
                &actor_metamorphosis,
                player,
                online,
                &mut remote_writer,
                &mut se_writer,
                &mut impact_writer,
                &mut spawn,
                wand,
                Trigger::Primary,
            );
        }

        if actor.fire_state_secondary == ActorFireState::Fire {
            cast_spell(
                &mut commands,
                &assets,
                &constants,
                &life_bar_resource,
                actor_entity,
                &mut actor,
                &mut actor_life,
                &actor_transform,
                &mut actor_impulse,
                &mut actor_falling,
                &mut collision_groups,
                &actor_metamorphosis,
                player,
                online,
                &mut remote_writer,
                &mut se_writer,
                &mut impact_writer,
                &mut spawn,
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
    assets: Res<GameAssets>,
    constants: Res<Assets<GameActors>>,
    mut query: Query<(&mut Actor, &mut ExternalForce, &Transform, &Vertical)>,
    mut se: EventWriter<SEEvent>,
) {
    let constants = constants.get(assets.actors.id()).unwrap();

    for (mut actor, mut external_force, transform, vertical) in query.iter_mut() {
        let ratio = if 0 < actor.frozen {
            0.0
        } else if 0 < actor.drowning || 0 < actor.staggered || 0.0 < vertical.v {
            0.2
        } else if actor.fire_state == ActorFireState::Fire
            || actor.fire_state_secondary == ActorFireState::Fire
        {
            0.5
        } else {
            1.0
        };

        let force = actor.move_direction * actor.get_total_move_force(&constants) * ratio;

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

fn defreeze(mut query: Query<&mut Actor>) {
    for mut actor in query.iter_mut() {
        if actor.defreeze <= actor.frozen {
            actor.frozen -= actor.defreeze;
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

fn decrement_levitation(mut actor_query: Query<&mut Actor>) {
    for mut actor in actor_query.iter_mut() {
        if 0 < actor.levitation {
            actor.levitation -= 1;
        }
        if actor.levitation <= 240 && actor.auto_levitation {
            actor.levitation = 240;
        }
    }
}

fn levitation_effect(
    actor_query: Query<&Actor>,
    mut group_query: Query<(&Parent, &mut Counter), With<ActorSpriteGroup>>,
    mut effect_query: Query<(&Parent, &mut Visibility), With<ActorLevitationEffect>>,
) {
    for (parent, mut visibility) in effect_query.iter_mut() {
        let (group, mut counter) = group_query.get_mut(parent.get()).unwrap();
        let actor = actor_query.get(group.get()).unwrap();
        if actor.auto_levitation {
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
    mut query: Query<(&Vertical, &mut Damping, &Actor)>,
    assets: Res<GameAssets>,
    constants: Res<Assets<GameActors>>,
) {
    for (vertical, mut damping, actor) in query.iter_mut() {
        let constants = constants.get(assets.actors.id()).unwrap();
        let props = actor.to_props(&constants);
        damping.linear_damping = if 0.0 < vertical.v {
            1.0
        } else {
            props.linear_damping
        };
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
        &mut Life,
        Option<&Player>,
        Option<&Metamorphosed>,
    )>,
    mut damage: EventWriter<ActorEvent>,
    mut level: ResMut<LevelSetup>,
    mut commands: Commands,
    mut se: EventWriter<SEEvent>,
    mut interpreter: EventWriter<InterpreterEvent>,
) {
    for (entity, mut actor, transform, vertical, life, player, morph) in actor_query.iter_mut() {
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
                // info!("despawn {} {}", file!(), line!());
                se.send(SEEvent::pos(SE::Scene2, position));
                if let Some(player) = player {
                    recovery(&mut level, &mut interpreter, &morph, &player, &life, &actor);
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

pub fn get_default_actor(actor_type: ActorType) -> (Actor, Life) {
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
            )
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
