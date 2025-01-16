use bevy_rapier2d::prelude::*;
use std::cell::LazyCell;

/// 木箱などの通常のエンティティのメンバーシップです
/// ハイド状態のシャドウなどを除きほとんどのアクターと衝突し、弾丸にも衝突します
const ENTITY_MEMBERSHIPS: Group = Group::GROUP_1;

/// ツボの破片などのグループ
/// 壁やチェストにはめり込まないで散らばるが、プレイヤーキャラクターなどには干渉しない
const PIECE_MEMBERSHIPS: Group = Group::GROUP_2;

const WALL_MEMBERSHIPS: Group = Group::GROUP_3;

/// ニワトリなどの中立キャラクターのグループ
const NEUTRAL_MEMBERSHIPS: Group = Group::GROUP_4;

const FLYING_NEUTRAL_MEMBERSHIPS: Group = Group::GROUP_18;

/// プレイヤーキャラクターのグループ
const PLAYER_MEMBERSHIPS: Group = Group::GROUP_5;

const FLYING_PLAYER_MEMBERSHIPS: Group = Group::GROUP_16;

const PLAYER_BULLET_MEMBERSHIPS: Group = Group::GROUP_6;

const ENEMY_MEMBERSHIPS: Group = Group::GROUP_7;

const FLYING_ENEMY_MEMBERSHIPS: Group = Group::GROUP_17;

const ENEMY_BULLET_MEMBESHIPS: Group = Group::GROUP_8;

const GOLD_MEMBERSHIPS: Group = Group::GROUP_9;

/// 汎用のセンサーです
/// 魔法陣の出入りなどの判定に使われるほか、intersections_with_shape を通じて衝撃の範囲、延焼の範囲判定などに使われるため、
/// すべてのアクターとエンティティと衝突します
const SENSOR_MEMBERSHIPS: Group = Group::GROUP_10;

/// 商品がショップから押し出されないようにするための見えない壁です
const HIDDEN_WALL_MEMBERSHIPS: Group = Group::GROUP_11;

const RABBIT_MEMBERSHIPS: Group = Group::GROUP_12;

/// SHADOW_GROUP はシャドウが隠れているときのみのメンバーシップで、
/// 壁やアクターには衝突しますが、弾丸は当たりません
const SHADOW_MEMBERSHIPS: Group = Group::GROUP_13;

/// ドロップアイテムのグループ
/// ENTITY_GROUPと似ていますが、敵キャラクターと敵キャラクターの弾丸には衝突しません
/// 敵キャラクターがアイテムを押して盾にするのを避けるためです
const DROPPED_ITEM_MEMBERSHIPS: Group = Group::GROUP_14;

const WATER_MEMBERSHIPS: Group = Group::GROUP_15;

pub const ENTITY_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        ENTITY_MEMBERSHIPS,
        PIECE_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | FLYING_NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | FLYING_PLAYER_MEMBERSHIPS
            | PLAYER_BULLET_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | FLYING_ENEMY_MEMBERSHIPS
            | ENEMY_BULLET_MEMBESHIPS
            | WALL_MEMBERSHIPS
            | RABBIT_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | GOLD_MEMBERSHIPS
            | SENSOR_MEMBERSHIPS
            | WATER_MEMBERSHIPS,
    )
});

pub const PIECE_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        PIECE_MEMBERSHIPS,
        PIECE_MEMBERSHIPS | WALL_MEMBERSHIPS | WATER_MEMBERSHIPS | WATER_MEMBERSHIPS,
    )
});

pub const WALL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        WALL_MEMBERSHIPS,
        PIECE_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | FLYING_NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | FLYING_PLAYER_MEMBERSHIPS
            | PLAYER_BULLET_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | FLYING_ENEMY_MEMBERSHIPS
            | ENEMY_BULLET_MEMBESHIPS
            | RABBIT_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | GOLD_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS,
    )
});

const ACTOR_FILTER_BASE: LazyCell<Group> = LazyCell::new(|| {
    ENTITY_MEMBERSHIPS
        | NEUTRAL_MEMBERSHIPS
        | FLYING_NEUTRAL_MEMBERSHIPS
        | WALL_MEMBERSHIPS
        | PLAYER_MEMBERSHIPS
        | FLYING_PLAYER_MEMBERSHIPS
        | ENEMY_MEMBERSHIPS
        | FLYING_ENEMY_MEMBERSHIPS
        | RABBIT_MEMBERSHIPS
        | SENSOR_MEMBERSHIPS
        | SHADOW_MEMBERSHIPS
});

pub const NEUTRAL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        NEUTRAL_MEMBERSHIPS,
        PLAYER_BULLET_MEMBERSHIPS
            | ENEMY_BULLET_MEMBESHIPS
            | WATER_MEMBERSHIPS
            | *ACTOR_FILTER_BASE,
    )
});

pub const FLYING_NEUTRAL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        FLYING_NEUTRAL_MEMBERSHIPS,
        PLAYER_BULLET_MEMBERSHIPS | ENEMY_BULLET_MEMBESHIPS | *ACTOR_FILTER_BASE,
    )
});

pub const PLAYER_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        PLAYER_MEMBERSHIPS,
        ENEMY_BULLET_MEMBESHIPS | DROPPED_ITEM_MEMBERSHIPS | WATER_MEMBERSHIPS | *ACTOR_FILTER_BASE,
    )
});

pub const FLYING_PLAYER_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        FLYING_PLAYER_MEMBERSHIPS,
        ENEMY_BULLET_MEMBESHIPS | DROPPED_ITEM_MEMBERSHIPS | *ACTOR_FILTER_BASE,
    )
});

pub const ENEMY_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        ENEMY_MEMBERSHIPS,
        PLAYER_BULLET_MEMBERSHIPS | WATER_MEMBERSHIPS | *ACTOR_FILTER_BASE,
    )
});

pub const FLYING_ENEMY_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        FLYING_ENEMY_MEMBERSHIPS,
        PLAYER_BULLET_MEMBERSHIPS | *ACTOR_FILTER_BASE,
    )
});

const BULLET_FILTER_BASE: LazyCell<Group> =
    LazyCell::new(|| ENTITY_MEMBERSHIPS | WALL_MEMBERSHIPS | NEUTRAL_MEMBERSHIPS);

pub const PLAYER_BULLET_GROUP: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        PLAYER_BULLET_MEMBERSHIPS,
        // アイテムを押して動かせるように、プレイヤーの弾丸は DROPPED_ITEM_MEMBERSHIPS に衝突します
        ENEMY_MEMBERSHIPS
            | FLYING_ENEMY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | FLYING_NEUTRAL_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | *BULLET_FILTER_BASE,
    )
});

pub const ENEMY_BULLET_GROUP: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        ENEMY_BULLET_MEMBESHIPS,
        // 敵がアイテムを盾に接近するのを避けるために、敵の弾丸は DROPPED_ITEM_MEMBERSHIPS に衝突しません
        PLAYER_MEMBERSHIPS
            | FLYING_PLAYER_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | FLYING_NEUTRAL_MEMBERSHIPS
            | *BULLET_FILTER_BASE,
    )
});

pub const GOLD_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        GOLD_MEMBERSHIPS,
        ENTITY_MEMBERSHIPS | WALL_MEMBERSHIPS | WATER_MEMBERSHIPS,
    )
});

pub const SENSOR_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        SENSOR_MEMBERSHIPS,
        PLAYER_MEMBERSHIPS
            | FLYING_PLAYER_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | FLYING_ENEMY_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS
            | SENSOR_MEMBERSHIPS, // 爆弾で蜘蛛の巣を破壊するため、センサーはセンサーと衝突します
    )
});

pub const HIDDEN_WALL_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        HIDDEN_WALL_MEMBERSHIPS,
        ENTITY_MEMBERSHIPS | RABBIT_MEMBERSHIPS | DROPPED_ITEM_MEMBERSHIPS,
    )
});

pub const RABBIT_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        RABBIT_MEMBERSHIPS,
        ENTITY_MEMBERSHIPS
            | WALL_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | FLYING_PLAYER_MEMBERSHIPS
            | ENEMY_MEMBERSHIPS
            | FLYING_ENEMY_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS
            | HIDDEN_WALL_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | WATER_MEMBERSHIPS,
    )
});

pub const SHADOW_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(SHADOW_MEMBERSHIPS, WATER_MEMBERSHIPS | *ACTOR_FILTER_BASE)
});

pub const DROPPED_ITEM_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        DROPPED_ITEM_MEMBERSHIPS,
        DROPPED_ITEM_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            | FLYING_PLAYER_MEMBERSHIPS
            | PLAYER_BULLET_MEMBERSHIPS
            | WALL_MEMBERSHIPS
            | HIDDEN_WALL_MEMBERSHIPS
            | RABBIT_MEMBERSHIPS
            | WATER_MEMBERSHIPS,
    )
});

pub const WATER_GROUPS: LazyCell<CollisionGroups> = LazyCell::new(|| {
    CollisionGroups::new(
        WATER_MEMBERSHIPS,
        PIECE_MEMBERSHIPS
            | ENTITY_MEMBERSHIPS
            | NEUTRAL_MEMBERSHIPS
            | PLAYER_MEMBERSHIPS
            // | FLYING_PLAYER_MEMBERSHIPS // 水は飛行中のプレイヤーと衝突しないことに注意
            | ENEMY_MEMBERSHIPS
            // | FLYING_ENEMY_MEMBERSHIPS // 水は飛行中の敵と衝突しないことに注意
            | RABBIT_MEMBERSHIPS
            | DROPPED_ITEM_MEMBERSHIPS
            | GOLD_MEMBERSHIPS
            | SHADOW_MEMBERSHIPS,
    )
});
