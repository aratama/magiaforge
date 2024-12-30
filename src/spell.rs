use crate::cast::SpellCast;
use crate::constant::TILE_SIZE;
use crate::entity::servant_seed::ServantType;
use crate::language::Dict;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Reflect, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, strum::EnumIter, Hash, Serialize, Deserialize)]
pub enum SpellType {
    MagicBolt,
    LightBall,
    PurpleBolt,
    SlimeCharge,
    WaterBall,
    BulletSpeedDoown,
    DualCast,
    SummonEnemySlime,
    SummonEnemyEyeball,
    HeavyShot,
    Bomb,
    TripleCast,
    Homing,
    SummonFriendSlime,
    PrecisionUp,
    SummonFriendEyeball,
    Dash,
    Impact,
    BulletSpeedUp,
    Heal,
    QuickCast,
}

/// 呪文の基礎情報
pub struct SpellProps {
    pub rank: u32,
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
                rank: 0,
                name: Dict {
                    ja: "マジックボルト",
                    en: "Magic Bolt",
                    zh_cn: "魔法箭"
                },
                description: Dict {
                    ja: "魔力の塊を発射する、最も基本的な攻撃魔法です。",
                    en: "A basic attack spell that fires a bolt of magic.",
                    zh_cn: "发射魔法箭的基本攻击魔法。"
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
            SpellType::LightBall =>  SpellProps {
                rank: 0,
                name: Dict {
                    ja: "光球",
                    en: "Light Ball",
                    zh_cn: "光球"
                },
                description: Dict {
                    ja: "周囲をしばらく明るく照らす光の玉を出現させます。威力はありません。",
                    en: "Creates a ball of light that illuminates the area for a while. It has no attack power.",
                    zh_cn: "产生一个照亮区域一段时间的光球。它没有攻击力。"
                },
                cast_delay: 120,
                icon: "light_ball_icon",
                price: 10,
                cast: SpellCast::Bullet {
                    slice: "light_ball",
                    collier_radius: 5.0,
                    speed: 4.0,
                    lifetime: 60 * 60,
                    damage: 0,
                    impulse: 0.0,
                    scattering: 0.4,
                    light_intensity: 4.0,
                    light_radius: TILE_SIZE * 10.0,
                    light_color_hlsa: [0.0, 0.0, 1.0, 1.0],
                },
            },
            SpellType::PurpleBolt =>  SpellProps {
                rank: 1,
                name: Dict {
                    ja: "悪意の視線",
                    en: "Evil Eye",
                    zh_cn: "邪恶之眼"
                },
                description: Dict {
                    ja:
                        "邪悪な魔力を帯びた視線です。浴びせられると少し悪寒が走ります。",
                    en: "Fires a slow-moving purple energy bolt. It is weak but consumes little mana.",
                    zh_cn: "发射一个移动缓慢的紫色能量弹。它很弱，但消耗的魔法很少。"
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
                rank: 1,
                name: Dict {
                    ja: "スライムの塊",
                    en: "Slime Limp",
                    zh_cn: "史莱姆块"
                },
                description: Dict {
                    ja: "ぷにぷにとした塊で殴りつけます。痛くはありませんが、相手を大きく吹き飛ばします。",
                    en: "Slap with a soft, squishy lump. It doesn't hurt much, but it knocks the opponent backward.",
                    zh_cn: "用柔软、软乎乎的块状物拍打。虽然不会很疼，但会将对手击退。"
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
            SpellType::WaterBall =>  SpellProps {
                rank: 2,
                name: Dict {
                    ja: "水の塊",
                    en: "Water Ball",
                    zh_cn: "水球"
                },
                description: Dict {
                    ja: "水の塊を発射します。威力は低いですが、相手を押し返すことができます。",
                    en: "Fires a ball of water. It is weak but can push the opponent back.",
                    zh_cn: "发射一个水球。它很弱，但可以将对手击退。"
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
            SpellType::BulletSpeedDoown =>  SpellProps {
                rank: 2,
                name: Dict {
                    ja: "減速",
                    en: "Speed Down",
                    zh_cn: "减速"
                },
                description: Dict { ja: "次に発射する魔法の弾速を50%低下させます。",
                en: "Reduces the speed of the next magic bullet by 50%.",
                zh_cn: "将下一个魔法弹的速度降低50%。"
             },
                cast_delay: 0,
                icon: "bullet_speed_down",
                price: 20,
                cast: SpellCast::BulletSpeedUpDown { delta: -0.5 },
            },
            SpellType::DualCast => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "並列詠唱",
                    en: "Dual Cast",
                    zh_cn: "双重施法"
                },
                description: Dict { ja: "ふたつの投射物呪文を同時に詠唱します。詠唱遅延は大きいほうに揃えられます。", 
                en: "Casts two projectile spells at the same time.",
                zh_cn: "同时施放两个投射法术。施法延迟将与较大的值对齐。"
             },
                cast_delay: 0,
                icon: "spell_dual_cast",
                price: 20,
                cast: SpellCast::MultipleCast { amount: 2 },
            },
            SpellType::SummonEnemySlime => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "敵スライム召喚",
                    en: "Summon Enemy Slime",
                    zh_cn: "召唤敌人史莱姆"
                },
                description: Dict { ja: "敵のスライムを召喚します。",
                en: "Summons a enemy slime",
                zh_cn: "召唤一个敌人史莱姆"
             },
                cast_delay: 30,
                icon: "slime",
                price: 200,
                cast: SpellCast::Summon { friend: false, servant_type: ServantType::Slime },
            },
            SpellType::SummonEnemyEyeball => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "敵アイボール召喚",
                    en: "Summon Enemy Eyeball",
                    zh_cn: "召唤敌人眼球"
                },
                description: Dict { ja: "敵のアイボールを召喚します。",
                en: "Summons a enemy Eyeball",
                zh_cn: "召唤一个敌人眼球"
             },
                cast_delay: 30,
                icon: "eyeball",
                price: 200,
                cast: SpellCast::Summon { friend: false, servant_type: ServantType::Eyeball },
            },
            SpellType::HeavyShot => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "ヘヴィーショット",
                    en: "Heavy Shot",
                    zh_cn: "重型射击"
                },
                description: Dict { ja: "次に発射する魔法弾の威力が上昇しますが、飛翔速度が低下します。",
                en: "The next magic bullet you fire will be more powerful and slower.",
                zh_cn: "下一个魔法弹的威力将增加，但飞行速度将减慢。"
             },
                cast_delay: 5,
                icon: "spell_heavy_shot",
                price: 30,
                cast: SpellCast::HeavyShot,
            },
            SpellType::Bomb => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "爆弾",
                    en: "Bomb",
                    zh_cn: "炸弹"
                },
                description: Dict { ja: "黒色火薬が詰まった爆弾です。時間が経つと爆発します。",
                en: "A bomb filled with black powder. It explodes after a while.",
                zh_cn: "TODO"
             },
                cast_delay: 120,
                icon: "bomb_icon",
                price: 300,
                cast: SpellCast::Bomb,
            },
            SpellType::TripleCast =>  SpellProps {
                rank: 3,
                name: Dict {
                    ja: "三並列詠唱",
                    en: "Triple Cast",
                    zh_cn: "三重施法"
                },
                description: Dict { ja: "みっつの投射物呪文を同時に詠唱します。", 
                en: "Casts three projectile spells at the same time.",
                zh_cn: "同时施放三个投射法术。"
             },
                cast_delay: 0,
                icon: "spell_triple_cast",
                price:30,
                cast: SpellCast::MultipleCast { amount: 3 },
            },
            SpellType::Homing => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "自律型追尾",
                    en: "Self-directed Homing",
                    zh_cn: "自主追踪"
                },
                description: Dict { ja: "次に発射する魔法弾が近くの敵に向かって追尾します。", 
                en: "The next magic bullet you fire will home in on the enemy.",
                zh_cn: "下一个魔法弹将追踪敌人。"
             },
                cast_delay: 5,
                icon: "spell_homing",
                price: 40,
                cast: SpellCast::Homing,
            },
            SpellType::SummonFriendSlime => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "味方スライム召喚",
                    en: "Summon Friend Slime",
                    zh_cn: "召唤友军史莱姆"
                },
                description: Dict { ja: "味方のスライムを召喚します。",
                en: "Summons a friend slime",
                zh_cn: "召唤一个友军史莱姆"
             },
                cast_delay: 30,
                icon: "slime",
                price: 200,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Slime },
            },
            SpellType::PrecisionUp => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "精度向上",
                    en: "Precision Up",
                    zh_cn: "精度提高"
                },
                description: Dict { ja: "弾丸の精度を向上させます。",
                en: "Increases the accuracy of bullets.",
                zh_cn: "提高子弹的精度。"
             },
                cast_delay: 1,
                icon: "precision_icon",
                price: 500,
                cast: SpellCast::PrecisionUp,
            },
            SpellType::SummonFriendEyeball => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "味方アイボール召喚",
                    en: "Summon Friend Eyeball",
                    zh_cn: "召唤友军眼球"
                },
                description: Dict { ja: "味方のアイボールを召喚します。",
                en: "Summons a friend Eyeball", 
                zh_cn: "召唤一个友军眼球"
             },
                cast_delay: 30,
                icon: "eyeball",
                price: 200,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Eyeball },
            },
            SpellType::Dash => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "ダッシュ",
                    en: "Dash",
                    zh_cn: "冲刺"
                },
                description: Dict { ja: "短距離を素早く走ります。",
                en: "Dashes a short distance.",
                zh_cn: "短距离快速奔跑。"
             },
                cast_delay: 50,
                icon: "dash",
                price: 500,
                cast: SpellCast::Dash,
            },
            SpellType::Impact => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "衝撃",
                    en: "Impact",
                    zh_cn: "冲击"
                },
                description: Dict { ja: "周囲に衝撃波を起こします。敵も味方もまとめて吹き飛ばします。",
                en: "Creates a shockwave around you that knocks back enemies and allies.",
                zh_cn: "在你周围产生冲击波，将敌人和盟友一起击退。"
             },
                cast_delay: 60,
                icon: "impact_icon",
                price: 500,
                cast: SpellCast::Impact,
            },
            SpellType::BulletSpeedUp => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "加速",
                    en: "Speed Up",
                    zh_cn: "加速"
                },
                description: Dict { ja: "次に発射する魔法の弾速を50%上昇させます。",
                en: "Increases the speed of the next magic bullet by 50%.",
                zh_cn: "将下一个魔法弹的速度提高50%。"
             },
                cast_delay: 0,
                icon: "bullet_speed_up",
                price: 20,
                cast: SpellCast::BulletSpeedUpDown { delta: 0.5 },
            },
            SpellType::Heal =>  SpellProps {
                rank: 5,
                name: Dict {
                    ja: "回復",
                    en: "Heal",
                    zh_cn: "治疗"
                },
                description: Dict { ja: "自分自身の体力を少しだけ回復します。", 
                en: "Heals a small amount of your own health.",
                zh_cn: "治疗自己一点点的生命值。"
             },
                cast_delay: 120,
                icon: "spell_heal",
                price: 40,
                cast: SpellCast::Heal,
            },
            SpellType::QuickCast => SpellProps {
                rank: 5,
                name: Dict {
                    ja: "高速詠唱",
                    en: "Quick Cast",
                    zh_cn: "快速施法"
                },
                description: Dict { ja: "次に詠唱する呪文の詠唱時間を減らします。",
                en: "Reduces the casting time of the next spell.",
                zh_cn: "减少下一个法术的施法时间。"
             },
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
    zh_cn: "伤害"
};

const KNOCKBACK: Dict<&'static str> = Dict {
    ja: "ノックバック",
    en: "Knockback",
    zh_cn: "击退"
};

const SPEED: Dict<&'static str> = Dict {
    ja: "射出速度",
    en: "Speed",
    zh_cn: "速度"
};

const LIFETIME: Dict<&'static str> = Dict {
    ja: "持続時間",
    en: "Lifetime",
    zh_cn: "持续时间"
};

const SCATTERING: Dict<&'static str> = Dict {
    ja: "拡散",
    en: "Scattering",
    zh_cn: "扩散"
};

const SIZE: Dict<&'static str> = Dict {
    ja: "大きさ",
    en: "Size",
    zh_cn: "大小"
};

const HEAL_TEXT: Dict<&'static str> = Dict {
    ja: "回復",
    en: "Heal",
    zh_cn: "治疗"
};

pub fn get_spell_appendix(cast: SpellCast) -> Dict<String> {
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
            let mut empty = Dict::empty();
            empty += DAMAGE.to_string();
            empty += Dict::literal(damage);
            empty += KNOCKBACK.to_string();
            empty += Dict::literal(impulse * 0.001);
            empty += SPEED.to_string();
            empty += Dict::literal(speed);
            empty += LIFETIME.to_string();
            empty += Dict::literal(lifetime);
            empty += SCATTERING.to_string();
            empty += Dict::literal(scattering);
            empty += SIZE.to_string();
            empty += Dict::literal(collier_radius);
            empty
        }
        SpellCast::Heal => {
            let mut empty = Dict::empty();
            empty += HEAL_TEXT.to_string();
            empty += Dict::literal("10");
            empty
        }
        SpellCast::HeavyShot => {
            // TODO
            // format!("威力: +5"),
            Dict::empty()
        }
        _ => Dict::empty()
    }
}
