use crate::spell::Spell;

pub struct SpellProps {
    pub mana_drain: i32,
    pub cast_delay: u32,
    pub slice: &'static str,
}

const MAGIC_BOLT: SpellProps = SpellProps {
    mana_drain: 50,
    cast_delay: 10,
    slice: "bullet_magic_bolt",
};

const PURPLE_BOLT: SpellProps = SpellProps {
    mana_drain: 100,
    cast_delay: 20,
    slice: "bullet_purple",
};

const SLIME_CHARGE: SpellProps = SpellProps {
    mana_drain: 200,
    cast_delay: 30,
    slice: "bullet_slime_charge",
};

const HEAL: SpellProps = SpellProps {
    mana_drain: 20,
    cast_delay: 120,
    slice: "spell_heal",
};

pub fn spell_to_props(spell: Spell) -> SpellProps {
    match spell {
        Spell::MagicBolt => MAGIC_BOLT,
        Spell::PurpleBolt => PURPLE_BOLT,
        Spell::SlimeCharge => SLIME_CHARGE,
        Spell::Heal => HEAL,
    }
}
