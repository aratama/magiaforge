use bevy::reflect::Reflect;

#[derive(Reflect, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
    SummonFriendSlime,
    SummonEnemySlime,
}

pub const SPELL_TYPES: [SpellType; 12] = [
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
    SpellType::SummonFriendSlime,
    SpellType::SummonEnemySlime,
];
