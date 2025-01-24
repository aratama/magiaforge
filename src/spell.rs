use crate::cast::SpellCast;
use crate::cast::SpellCastBullet;
use crate::language::Dict;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Reflect, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Spell(pub String);

impl Spell {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
/// 呪文の基礎情報
pub struct SpellProps {
    pub rank: u8,
    pub name: Dict<String>,
    pub description: Dict<String>,
    pub cast_delay: u32,
    pub icon: String,
    pub price: u32,
    pub cast: SpellCast,
}

const DAMAGE: Dict<&'static str> = Dict {
    ja: " / ダメージ",
    en: " / Damage",
    zh_cn: " / 伤害",
    zh_tw: " / 傷害",
    es: " / Daño",
    fr: " / Dégâts",
    pt: " / Dano",
    de: " / Schaden",
    ko: " / 피해",
    ru: " / Урон",
};

const KNOCKBACK: Dict<&'static str> = Dict {
    ja: " / ノックバック",
    en: " / Knockback",
    zh_cn: " / 击退",
    zh_tw: " / 擊退",
    es: " / Retroceso",
    fr: " / Recul",
    pt: " / Recuo",
    de: " / Rückstoß",
    ko: " / 넉백",
    ru: " / Отбрасывание",
};

const SPEED: Dict<&'static str> = Dict {
    ja: " / 射出速度",
    en: " / Speed",
    zh_cn: " / 速度",
    zh_tw: " / 速度",
    es: " / Velocidad",
    fr: " / Vitesse",
    pt: " / Velocidade",
    de: " / Geschwindigkeit",
    ko: " / 속도",
    ru: " / Скорость",
};

const LIFETIME: Dict<&'static str> = Dict {
    ja: " / 持続時間",
    en: " / Lifetime",
    zh_cn: " / 持续时间",
    zh_tw: " / 持續時間",
    es: " / Duración",
    fr: " / Durée",
    pt: " / Duração",
    de: " / Lebensdauer",
    ko: " / 지속 시간",
    ru: " / Время жизни",
};

const SCATTERING: Dict<&'static str> = Dict {
    ja: " / 拡散",
    en: " / Scattering",
    zh_cn: " / 扩散",
    zh_tw: " / 擴散",
    es: " / Dispersión",
    fr: " / Dispersion",
    pt: " / Dispersão",
    de: " / Streuung",
    ko: " / 산란",
    ru: " / Разброс",
};

const SIZE: Dict<&'static str> = Dict {
    ja: " / 大きさ",
    en: " / Size",
    zh_cn: " / 大小",
    zh_tw: " / 大小",
    es: " / Tamaño",
    fr: " / Taille",
    pt: " / Tamanho",
    de: " / Größe",
    ko: " / 크기",
    ru: " / Размер",
};

const HEAL_TEXT: Dict<&'static str> = Dict {
    ja: " / 回復",
    en: " / Heal",
    zh_cn: " / 治疗",
    zh_tw: " / 治療",
    es: " / Curar",
    fr: " / Soigner",
    pt: " / Cura",
    de: " / Heilen",
    ko: " / 치유",
    ru: " / Исцеление",
};

pub fn get_spell_appendix(cast: &SpellCast) -> Dict<String> {
    match cast {
        SpellCast::Bullet(SpellCastBullet {
            slices: _,
            collier_radius,
            speed,
            lifetime,
            damage,
            impulse,
            scattering,
            light_intensity: _,
            light_radius: _,
            light_color_hlsa: _,
            freeze: _,
            levitation: _,
            stagger: _,
        }) => {
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
        _ => Dict::empty(),
    }
}
