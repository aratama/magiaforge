use bevy::reflect::Reflect;

#[derive(Reflect, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, strum::EnumIter)]
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
    Dash,
}
