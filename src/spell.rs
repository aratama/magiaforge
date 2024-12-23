use crate::entity::servant_seed::ServantType;
use crate::language::Dict;
use crate::language::Languages;
use bevy::reflect::Reflect;

#[derive(Reflect, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, strum::EnumIter)]
pub enum SpellType {
    MagicBolt,
    WaterBall,
    PurpleBolt,
    SlimeCharge,
    Heal,
    BulletSpeedUp,
    BulletSpeedDoown,
    DualCast,
    TripleCast,
    Homing,
    HeavyShot,
    SummonFriendSlime,
    SummonEnemySlime,
    SummonFriendEyeball,
    SummonEnemyEyeball,
    Dash,
    QuickCast,
}

/// 呪文を詠唱したときの動作を表します
/// 弾丸系魔法は Bullet にまとめられており、
/// そのほかの魔法も動作の種別によって分類されています
#[derive(Debug)]
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
    MultipleCast {
        amount: u32,
    },
    Homing,
    HeavyShot,
    Summon {
        friend: bool,
        servant_type: ServantType,
    },
    Dash,
    QuickCast,
}

/// 呪文の基礎情報
pub struct SpellProps {
    pub name: Dict<&'static str>,
    pub description: Dict<&'static str>,
    pub cast_delay: u32,
    pub icon: &'static str,
    pub price: u32,
    pub cast: SpellCast,
}

impl SpellType {
    pub fn to_props(&self) -> SpellProps {
        match self {
            SpellType::MagicBolt =>  SpellProps {
                name: Dict {
                    ja: "マジックボルト",
                    en: "Magic Bolt",
                },
                description: Dict {
                    ja: "魔力の塊を発射する、最も基本的な攻撃魔法です。",
                    en: "A basic attack spell that fires a bolt of magic.",
                },
                cast_delay: 20,
                icon: "bullet_magic_bolt",
                price: 10,
                cast: SpellCast::Bullet {
                    slice: "bullet_magic_bolt",
                    collier_radius: 5.0,
                    speed: 100.0,
                    lifetime: 240,
                    damage: 8,
                    impulse: 0.0,
                    scattering: 0.4,
                    light_intensity: 1.0,
                    light_radius: 50.0,
                    light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
                },
            },
            SpellType::WaterBall =>  SpellProps {
                name: Dict {
                    ja: "水の塊",
                    en: "Water Ball",
                },
                description: Dict {
                    ja: "水の塊を発射します。威力は低いですが、相手を押し返すことができます。",
                    en: "Fires a ball of water. It is weak but can push the opponent back.",
                },
                cast_delay: 8,
                icon: "spell_water_ball",
                price: 15,
                cast: SpellCast::Bullet {
                    slice: "water_ball",
                    collier_radius: 5.0,
                    speed: 80.0,
                    lifetime: 240,
                    damage: 1,
                    impulse: 80000.0,
                    scattering: 0.4,
                    light_intensity: 1.0,
                    light_radius: 50.0,
                    light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
                },
            },
            SpellType::PurpleBolt =>  SpellProps {
                name: Dict {
                    ja: "悪意の視線",
                    en: "Evil Eye",
                },
                description: Dict {
                    ja:
                        "邪悪な魔力を帯びた視線です。浴びせられると少し悪寒が走ります。",
                    en: "Fires a slow-moving purple energy bolt. It is weak but consumes little mana.",
                },
                cast_delay: 120,
                icon: "bullet_purple",
                price: 5,
                cast: SpellCast::Bullet {
                    slice: "bullet_purple",
                    collier_radius: 5.0,
                    speed: 50.0,
                    lifetime: 500,
                    damage: 3,
                    impulse: 0.0,
                    scattering: 0.6,
                    light_intensity: 0.0,
                    light_radius: 0.0,
                    light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
                },
            },
            SpellType::SlimeCharge => SpellProps {
                name: Dict {
                    ja: "スライムの塊",
                    en: "Slime Limp",
                },
                description: Dict {
                    ja: "ぷにぷにとした塊で殴りつけます。痛くはありませんが、相手を大きく吹き飛ばします。",
                    en: "Slap with a soft, squishy lump. It doesn't hurt much, but it knocks the opponent backward."
                },
                cast_delay: 30,
                icon: "bullet_slime_charge",
                price: 15,
                cast: SpellCast::Bullet {
                    slice: "bullet_slime_charge",
                    collier_radius: 5.0,
                    speed: 2.0,
                    lifetime: 5,
                    damage: 1,
                    impulse: 40000.0,
                    scattering: 0.0,
                    light_intensity: 0.0,
                    light_radius: 0.0,
                    light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
                },
            },
            SpellType::Heal =>  SpellProps {
                name: Dict {
                    ja: "回復",
                    en: "Heal",
                },
                description: Dict { ja: "自分自身の体力を少しだけ回復します。", 
                en: "Heals a small amount of your own health." },
                cast_delay: 120,
                icon: "spell_heal",
                price: 40,
                cast: SpellCast::Heal,
            },
            SpellType::BulletSpeedUp => SpellProps {
                name: Dict {
                    ja: "加速",
                    en: "Speed Up",
                },
                description: Dict { ja: "次に発射する魔法の弾速を50%上昇させます。",
                en: "Increases the speed of the next magic bullet by 50%." },
                cast_delay: 0,
                icon: "bullet_speed_up",
                price: 20,
                cast: SpellCast::BulletSpeedUpDown { delta: 0.5 },
            },
            SpellType::BulletSpeedDoown =>  SpellProps {
                name: Dict {
                    ja: "減速",
                    en: "Speed Down",
                },
                description: Dict { ja: "次に発射する魔法の弾速を50%低下させます。",
                en: "Reduces the speed of the next magic bullet by 50%." },
                cast_delay: 0,
                icon: "bullet_speed_down",
                price: 20,
                cast: SpellCast::BulletSpeedUpDown { delta: -0.5 },
            },
            SpellType::DualCast => SpellProps {
                name: Dict {
                    ja: "並列詠唱",
                    en: "Dual Cast",
                },
                description: Dict { ja: "ふたつの投射物呪文を同時に詠唱します。詠唱遅延は大きいほうに揃えられます。", 
                en: "Casts two projectile spells at the same time." },
                cast_delay: 0,
                icon: "spell_dual_cast",
                price: 20,
                cast: SpellCast::MultipleCast { amount: 2 },
            },
            SpellType::TripleCast =>  SpellProps {
                name: Dict {
                    ja: "三並列詠唱",
                    en: "Triple Cast",
                },
                description: Dict { ja: "みっつの投射物呪文を同時に詠唱します。", 
                en: "Casts three projectile spells at the same time." },
                cast_delay: 0,
                icon: "spell_triple_cast",
                price:30,
                cast: SpellCast::MultipleCast { amount: 3 },
            },
            SpellType::Homing => SpellProps {
                name: Dict {
                    ja: "自律型追尾",
                    en: "Self-directed Homing",
                },
                description: Dict { ja: "次に発射する魔法弾が近くの敵に向かって追尾します。", 
                en: "The next magic bullet you fire will home in on the enemy." },
                cast_delay: 5,
                icon: "spell_homing",
                price: 40,
                cast: SpellCast::Homing,
            },
            SpellType::HeavyShot => SpellProps {
                name: Dict {
                    ja: "ヘヴィーショット",
                    en: "Heavy Shot",
                },
                description: Dict { ja: "次に発射する魔法弾の威力が上昇しますが、飛翔速度が低下します。",
                en: "The next magic bullet you fire will be more powerful and slower." },
                cast_delay: 5,
                icon: "spell_heavy_shot",
                price: 30,
                cast: SpellCast::HeavyShot,
            },
            SpellType::SummonFriendSlime => SpellProps {
                name: Dict {
                    ja: "味方スライム召喚",
                    en: "Summon Friend Slime",
                },
                description: Dict { ja: "味方のスライムを召喚します。",
                en: "Summons a friend slime" },
                cast_delay: 30,
                icon: "slime",
                price: 200,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Slime },
            },
            SpellType::SummonEnemySlime => SpellProps {
                name: Dict {
                    ja: "敵スライム召喚",
                    en: "Summon Enemy Slime",
                },
                description: Dict { ja: "敵のスライムを召喚します。",
                en: "Summons a enemy slime" },
                cast_delay: 30,
                icon: "slime",
                price: 200,
                cast: SpellCast::Summon { friend: false, servant_type: ServantType::Slime },
            },
            SpellType::SummonFriendEyeball => SpellProps {
                name: Dict {
                    ja: "味方アイボール召喚",
                    en: "Summon Friend Eyeball",
                },
                description: Dict { ja: "味方のアイボールを召喚します。",
                en: "Summons a friend Eyeball" },
                cast_delay: 30,
                icon: "eyeball",
                price: 200,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Eyeball },
            },
            SpellType::SummonEnemyEyeball => SpellProps {
                name: Dict {
                    ja: "敵スライム召喚",
                    en: "Summon Enemy Slime",
                },
                description: Dict { ja: "敵のスライムを召喚します。",
                en: "Summons a enemy slime" },
                cast_delay: 30,
                icon: "eyeball",
                price: 200,
                cast: SpellCast::Summon { friend: false, servant_type: ServantType::Eyeball },
            },
            SpellType::Dash => SpellProps {
                name: Dict {
                    ja: "ダッシュ",
                    en: "Dash",
                },
                description: Dict { ja: "短距離を素早く走ります。",
                en: "Dashes a short distance." },
                cast_delay: 50,
                icon: "dash",
                price: 500,
                cast: SpellCast::Dash,
            },
            SpellType::QuickCast => SpellProps {
                name: Dict {
                    ja: "高速詠唱",
                    en: "Quick Cast",
                },
                description: Dict { ja: "次に詠唱する呪文の詠唱時間を減らします。",
                en: "Reduces the casting time of the next spell." },
                cast_delay: 1,
                icon: "quick_cast",
                price: 500,
                cast: SpellCast::QuickCast,
            },
        }
    }
}

const DAMAGE: Dict<&'static str> = Dict {
    ja: "ダメージ",
    en: "Damage",
};

const KNOCKBACK: Dict<&'static str> = Dict {
    ja: "ノックバック",
    en: "Knockback",
};

const SPEED: Dict<&'static str> = Dict {
    ja: "射出速度",
    en: "Speed",
};

const LIFETIME: Dict<&'static str> = Dict {
    ja: "持続時間",
    en: "Lifetime",
};

const SCATTERING: Dict<&'static str> = Dict {
    ja: "拡散",
    en: "Scattering",
};

const SIZE: Dict<&'static str> = Dict {
    ja: "大きさ",
    en: "Size",
};

const HEAL_TEXT: Dict<&'static str> = Dict {
    ja: "回復",
    en: "Heal",
};

pub fn get_spell_appendix(cast: SpellCast, language: Languages) -> String {
    match cast {
        SpellCast::Bullet {
            slice: _,
            collier_radius,
            speed,
            lifetime,
            damage,
            impulse,
            scattering,
            light_intensity: _,
            light_radius: _,
            light_color_hlsa: _,
        } => {
            format!(
                "{}:{}  {}:{}\n{}:{}  {}:{}\n{}:{}  {}:{}",
                DAMAGE.get(language),
                damage,
                KNOCKBACK.get(language),
                impulse * 0.001,
                SPEED.get(language),
                speed,
                LIFETIME.get(language),
                lifetime,
                SCATTERING.get(language),
                scattering,
                SIZE.get(language),
                collier_radius,
            )
        }
        SpellCast::Heal => {
            format!("{}:{}", HEAL_TEXT.get(language), 10)
        }
        SpellCast::HeavyShot => format!("威力: +5"),
        _ => format!(""),
    }
}
