use crate::spell::SpellType;

pub enum SpellCategory {
    Bullet {
        collier_radius: f32,

        /// 魔法弾の速度
        /// pixels_per_meter が 100.0 に設定されているので、
        /// 200は1フレームに2ピクセル移動する速度です
        speed: f32,

        lifetime: u32,
        damage: i32,
        impulse: f32,

        light_intensity: f32,
        light_radius: f32,
        light_color_hlsa: [f32; 4],
    },
    Heal,
    Buff,
}

pub struct SpellProps {
    pub name: &'static str,
    pub description: &'static str,

    pub mana_drain: i32,
    pub cast_delay: u32,
    pub icon: &'static str,

    pub category: SpellCategory,
}

const MAGIC_BOLT: SpellProps = SpellProps {
    name: "マジックボルト",
    description: "魔力の塊を発射する、最も基本的な魔法です。",
    mana_drain: 50,
    cast_delay: 10,
    icon: "bullet_magic_bolt",
    category: SpellCategory::Bullet {
        collier_radius: 5.0,
        speed: 200.0,
        lifetime: 240,
        damage: 5,
        impulse: 20000.0,
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
    category: SpellCategory::Bullet {
        collier_radius: 5.0,
        speed: 50.0,
        lifetime: 500,
        damage: 3,
        impulse: 10000.0,
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
    category: SpellCategory::Bullet {
        collier_radius: 5.0,
        speed: 2.0,
        lifetime: 5,
        damage: 10,
        impulse: 40000.0,
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
    category: SpellCategory::Heal,
};

const BULLET_SPEED_UP: SpellProps = SpellProps {
    name: "加速",
    description: "次に発射する魔法の弾速を25%上昇させます。",
    mana_drain: 20,
    cast_delay: 0,
    icon: "empty",
    category: SpellCategory::Buff,
};

pub fn spell_to_props(spell: SpellType) -> SpellProps {
    match spell {
        SpellType::MagicBolt => MAGIC_BOLT,
        SpellType::PurpleBolt => PURPLE_BOLT,
        SpellType::SlimeCharge => SLIME_CHARGE,
        SpellType::Heal => HEAL,
        SpellType::BulletSpeedUp => BULLET_SPEED_UP,
    }
}
