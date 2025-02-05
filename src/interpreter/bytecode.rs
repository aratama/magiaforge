use crate::spell::Spell;

#[allow(dead_code)]
enum ByteCode {
    HasSpell(Spell),
    If,
    End,
}
