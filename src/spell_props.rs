use crate::spell::SpellType;

/// 呪文を詠唱したときの動作を表します
/// 弾丸系魔法は Bullet にまとめられており、
/// そのほかの魔法も動作の種別によって分類されています
pub enum SpellCast {
    Bullet {
        slice: &'static str,

        collier_radius: f32,

        /// 魔法弾の速度
        /// pixels_per_meter が 100.0 に設定されているので、
        /// 200は1フレームに2ピクセル移動する速度です
        speed: f32,

        lifetime: u32,
        damage: i32,
        impulse: f32,

        scattering: f32,

        light_intensity: f32,
        light_radius: f32,
        light_color_hlsa: [f32; 4],
    },
    Heal,
    BulletSpeedUpDown {
        delta: f32,
    },
}

/// 呪文の基礎情報
pub struct SpellProps {
    pub name: &'static str,
    pub description: &'static str,
    pub mana_drain: i32,
    pub cast_delay: u32,
    pub icon: &'static str,
    pub category: SpellCast,
}

const MAGIC_BOLT: SpellProps = SpellProps {
    name: "マジックボルト",
    description: "魔力の塊を発射する、最も基本的な魔法です。",
    mana_drain: 50,
    cast_delay: 10,
    icon: "bullet_magic_bolt",
    category: SpellCast::Bullet {
        slice: "bullet_magic_bolt",
        collier_radius: 5.0,
        speed: 100.0,
        lifetime: 240,
        damage: 5,
        impulse: 20000.0,
        scattering: 0.4,
        light_intensity: 1.0,
        light_radius: 50.0,
        light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
    },
};

const PURPLE_BOLT: SpellProps = SpellProps {
    name: "スライム弾",
    description:
        "紫色のエネルギー弾を発射します。動きは遅く威力も低いですが、少ない魔力でも発射できます。",
    mana_drain: 10,
    cast_delay: 120,
    icon: "bullet_purple",
    category: SpellCast::Bullet {
        slice: "bullet_purple",
        collier_radius: 5.0,
        speed: 50.0,
        lifetime: 500,
        damage: 3,
        impulse: 10000.0,
        scattering: 0.6,
        light_intensity: 0.0,
        light_radius: 0.0,
        light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
    },
};

const SLIME_CHARGE: SpellProps = SpellProps {
    name: "スライムチャージ",
    description: "ぷにぷにとした塊で殴りつけます。やわらかいのであまり痛くありませんが、相手を大きく吹き飛ばします。",
    mana_drain: 200,
    cast_delay: 30,
    icon: "bullet_slime_charge",
    category: SpellCast::Bullet {
        slice: "bullet_slime_charge",
        collier_radius: 5.0,
        speed: 2.0,
        lifetime: 5,
        damage: 10,
        impulse: 40000.0,
        scattering: 0.0,
        light_intensity: 0.0,
        light_radius: 0.0,
        light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
    },
};

const HEAL: SpellProps = SpellProps {
    name: "ヒール",
    description: "自分自身の体力を少しだけ回復します。",
    mana_drain: 20,
    cast_delay: 120,
    icon: "spell_heal",
    category: SpellCast::Heal,
};

const BULLET_SPEED_UP: SpellProps = SpellProps {
    name: "加速",
    description: "次に発射する魔法の弾速を50%上昇させます。",
    mana_drain: 20,
    cast_delay: 0,
    icon: "bullet_speed_up",
    category: SpellCast::BulletSpeedUpDown { delta: 0.5 },
};

const BULLET_SPEED_DOWN: SpellProps = SpellProps {
    name: "減速",
    description: "次に発射する魔法の弾速を50%低下させます。",
    mana_drain: 20,
    cast_delay: 0,
    icon: "bullet_speed_down",
    category: SpellCast::BulletSpeedUpDown { delta: -0.5 },
};

pub fn spell_to_props(spell: SpellType) -> SpellProps {
    match spell {
        SpellType::MagicBolt => MAGIC_BOLT,
        SpellType::PurpleBolt => PURPLE_BOLT,
        SpellType::SlimeCharge => SLIME_CHARGE,
        SpellType::Heal => HEAL,
        SpellType::BulletSpeedUp => BULLET_SPEED_UP,
        SpellType::BulletSpeedDoown => BULLET_SPEED_DOWN,
    }
}
