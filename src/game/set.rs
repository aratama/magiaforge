// https://bevy-cheatbook.github.io/programming/system-sets.html
use bevy::prelude::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSet;
