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
use crate::component::metamorphosis::Metamorphosis;
use crate::component::vertical::Vertical;
use crate::constant::MAX_WANDS;
use crate::constant::TILE_SIZE;
use crate::controller::player::Player;
use crate::entity::bullet::Trigger;
use crate::entity::impact::SpawnImpact;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::Inventory;
use crate::inventory_item::InventoryItemType;
use crate::level::entities::SpawnEntity;
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
use bevy_rapier2d::prelude::ExternalForce;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_rapier2d::prelude::Group;
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

/// 自発的に移動し、攻撃の対象になる、プレイヤーキャラクターや敵モンスターを表します
/// Actorは外観の構造を規定しません。外観は各エンティティで具体的に実装するか、
/// BasicEnemyのような抽象化されたエンティティで実装しています
///
/// 移動
/// アクターの移動は、stateとmove_directionを設定することで行います
/// stateをActorState::Runに設定すると move_direction の方向に移動します
/// またこのとき、それぞれのActorの実装は歩行のアニメーションを再生します
///
#[derive(Component, Reflect, Debug)]
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
    /// 歩行と停止の切り替えは state を切り替えることで行うため、
    /// move_force を操作中に変更はしません
    pub move_force: f32,

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

    /// 0 より大きい場合は凍結状態
    /// 1フレームごとに減少します
    pub frozen: u32,

    pub defreeze: u32,

    pub levitation: u32,

    pub drowning: u32,

    pub auto_levitation: bool,
}

pub struct ActorProps {
    pub uuid: Uuid,
    pub angle: f32,
    pub point_light_radius: f32,
    pub current_wand: u8,
    pub actor_group: ActorGroup,
    pub golds: u32,
    pub wands: [Wand; MAX_WANDS],
    pub inventory: Inventory,
    pub radius: f32,
    pub move_force: f32,
    pub fire_resistance: bool,
    pub auto_levitation: bool,
}

impl Default for ActorProps {
    fn default() -> Self {
        ActorProps {
            uuid: Uuid::new_v4(),
            angle: 0.0,
            point_light_radius: 0.0,
            current_wand: 0,
            actor_group: ActorGroup::Neutral,
            golds: 0,
            wands: [Wand::empty(), Wand::empty(), Wand::empty(), Wand::empty()],
            inventory: Inventory::new(),
            radius: 8.0,
            move_force: 100000.0,
            fire_resistance: false,
            auto_levitation: false,
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
#[require(Visibility, Counter, Transform, LifeBeingSprite, ChildEntityDepth)]
pub struct ActorSpriteGroup;

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
            auto_levitation,
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
            wait: 30,
            trapped: 0,
            trap_moratorium: 0,
            floundering: 1,
            frozen: 0,
            defreeze: 1,
            levitation: 0,
            auto_levitation,
            drowning: 0,
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
    pub fn to_groups(&self, v: f32, drowning: u32) -> CollisionGroups {
        match self {
            ActorGroup::Player if 0 < drowning || 0.0 < v => *FLYING_PLAYER_GROUPS,
            ActorGroup::Player => *PLAYER_GROUPS,
            ActorGroup::Enemy if 0 < drowning || 0.0 < v => *FLYING_ENEMY_GROUPS,
            ActorGroup::Enemy => *ENEMY_GROUPS,
            ActorGroup::Neutral if 0 < drowning || 0.0 < v => *FLYING_NEUTRAL_GROUPS,
            ActorGroup::Neutral => *NEUTRAL_GROUPS,
        }
    }

    pub fn to_bullet_group(&self) -> CollisionGroups {
        match self {
            ActorGroup::Player => *PLAYER_BULLET_GROUP,
            ActorGroup::Enemy => *ENEMY_BULLET_GROUP,
            ActorGroup::Neutral => CollisionGroups::new(Group::NONE, Group::NONE), // 中立グループは弾丸を発射しません
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
    FatalFall {
        actor: Entity,
        position: Vec2,
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
    life_bar_resource: Res<LifeBarResource>,
    mut actor_query: Query<
        (
            Entity,
            &mut Actor,
            &mut Life,
            &mut Transform,
            &mut ExternalImpulse,
            &mut Vertical,
            Option<&Player>,
            Option<&Metamorphosis>,
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

    for (
        actor_entity,
        mut actor,
        mut actor_life,
        actor_transform,
        mut actor_impulse,
        mut actor_falling,
        player,
        morph,
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

        if actor.fire_state == ActorFireState::Fire {
            let wand = if morph.is_none() {
                actor.current_wand
            } else {
                0
            };
            cast_spell(
                &mut commands,
                &assets,
                &life_bar_resource,
                actor_entity,
                &mut actor,
                &mut actor_life,
                &actor_transform,
                &mut actor_impulse,
                &mut actor_falling,
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
                &life_bar_resource,
                actor_entity,
                &mut actor,
                &mut actor_life,
                &actor_transform,
                &mut actor_impulse,
                &mut actor_falling,
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
    mut query: Query<(&mut Actor, &mut ExternalForce, &Transform)>,
    mut se: EventWriter<SEEvent>,
) {
    for (mut actor, mut external_force, transform) in query.iter_mut() {
        // 凍結時は移動不能
        if 0 < actor.frozen {
            external_force.force = Vec2::ZERO;
            continue;
        }

        let ratio = if 0 < actor.drowning {
            0.2
        } else if actor.fire_state == ActorFireState::Fire
            || actor.fire_state_secondary == ActorFireState::Fire
        {
            0.5
        } else {
            1.0
        };

        let force = actor.move_direction * actor.get_total_move_force() * ratio;

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
    mut actor_query: Query<(Entity, &mut Actor, &Transform, &Vertical)>,
    mut damage: EventWriter<ActorEvent>,
    level: Res<LevelSetup>,
) {
    if let Some(ref chunk) = level.chunk {
        for (entity, mut actor, transform, vertical) in actor_query.iter_mut() {
            let position = transform.translation.truncate();
            let tile = chunk.get_tile_by_coords(position);

            match tile {
                Tile::Water => {
                    if 60 < actor.drowning {
                        damage.send(ActorEvent::Damaged {
                            actor: entity,
                            position: transform.translation.truncate(),
                            damage: 1,
                            fire: false,
                            impulse: Vec2::ZERO,
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
                        });
                        actor.drowning = 1;
                    }
                }
                Tile::Crack if vertical.v <= 0.0 => {
                    damage.send(ActorEvent::FatalFall {
                        actor: entity,
                        position: transform.translation.truncate(),
                    });
                }
                _ => {}
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
                drown,
                drown_damage,
            )
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
