use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum BulletType {
    BlueBullet,
    PurpleBullet,
    SlimeAttackBullet,
}

pub struct BulletProps {
    pub slice: &'static str,
    pub collier_radius: f32,

    /// 魔法弾の速度
    /// pixels_per_meter が 100.0 に設定されているので、
    /// 200は1フレームに2ピクセル移動する速度です
    pub speed: f32,
    pub lifetime: u32,
    pub damage: i32,
    pub impulse: f32,

    pub light_intensity: f32,
    pub light_radius: f32,
    pub light_color_hlsa: [f32; 4],
}

pub const MAGIC_BOLT_PROPS: BulletProps = BulletProps {
    slice: "bullet_magic_bolt",
    collier_radius: 5.0,
    speed: 200.0,
    lifetime: 240,
    damage: 5,
    impulse: 20000.0,

    light_intensity: 1.0,
    light_radius: 50.0,
    light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
};

pub const PURPLE_BULLET_PROPS: BulletProps = BulletProps {
    slice: "bullet_purple",
    collier_radius: 5.0,
    speed: 50.0,
    lifetime: 500,
    damage: 5,
    impulse: 10000.0,

    light_intensity: 0.0,
    light_radius: 0.0,
    light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
};

pub const SLIME_CHARGE_PROPS: BulletProps = BulletProps {
    slice: "bullet_slime_charge",
    collier_radius: 5.0,
    speed: 2.0,
    lifetime: 5,
    damage: 10,
    impulse: 40000.0,

    light_intensity: 0.0,
    light_radius: 0.0,
    light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
};

pub fn bullet_type_to_props(bullet_type: BulletType) -> &'static BulletProps {
    match bullet_type {
        BulletType::BlueBullet => &MAGIC_BOLT_PROPS,
        BulletType::PurpleBullet => &PURPLE_BULLET_PROPS,
        BulletType::SlimeAttackBullet => &SLIME_CHARGE_PROPS,
    }
}
