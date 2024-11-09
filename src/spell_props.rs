use crate::spell::Spell;

pub struct SpellProps {
    pub cost: i32,
    pub cooldown: u32,
    pub slice: &'static str,
}

const MAGIC_BOLT: SpellProps = SpellProps {
    cost: 50,
    cooldown: 10,
    slice: "bullet_magic_bolt",
};

const PURPLE_BOLT: SpellProps = SpellProps {
    cost: 100,
    cooldown: 20,
    slice: "bullet_purple",
};

const SLIME_CHARGE: SpellProps = SpellProps {
    cost: 200,
    cooldown: 30,
    slice: "bullet_slime_charge",
};

const HEAL: SpellProps = SpellProps {
    cost: 20,
    cooldown: 30,
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
