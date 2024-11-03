use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
pub struct Breakable {
    pub life: i32,
}
