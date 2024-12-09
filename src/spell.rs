#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum SpellType {
    MagicBolt,
    PurpleBolt,
    SlimeCharge,
    Heal,
    BulletSpeedUp,
    BulletSpeedDoown,
    DualCast,
    TripleCast,
    Homing,
    HeavyShot,
}

pub const SPELL_TYPES: [SpellType; 10] = [
    SpellType::MagicBolt,
    SpellType::PurpleBolt,
    SpellType::SlimeCharge,
    SpellType::Heal,
    SpellType::BulletSpeedUp,
    SpellType::BulletSpeedDoown,
    SpellType::DualCast,
    SpellType::TripleCast,
    SpellType::Homing,
    SpellType::HeavyShot,
];
