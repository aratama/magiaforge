// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
// https://qiita.com/LNSEAB/items/6f60da458460274e768d
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod asset;
mod audio;
mod camera;
mod cast;
mod se;
mod config;
mod constant;
mod controller;
mod curve;
mod debug;
mod enemy;
mod entity;
mod equipment;
mod footsteps;
mod game;
mod hud;
mod input;
mod interaction_sensor;
mod inventory;
mod inventory_item;
mod language;
mod level;
mod page;
mod player_state;
mod random;
mod set;
mod spell;
mod spell_props;
mod states;
mod ui;
mod wand;
mod wand_props;

use game::run_game;

fn main() {
    run_game();
}
