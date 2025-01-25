// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
// https://qiita.com/LNSEAB/items/6f60da458460274e768d
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actor;
mod asset;
mod audio;
mod camera;
mod cast;
mod collision;
mod component;
mod config;
mod constant;
mod controller;
mod curve;
mod debug;
mod debug_command;
mod enemy;
mod entity;
mod finder;
mod footsteps;
mod game;
mod hud;
mod input;
mod interpreter;
mod inventory;
mod inventory_item;
mod language;
mod level;
mod message;
mod page;
mod physics;
mod player_state;
mod random;
mod registry;
mod save;
mod se;
mod set;
mod spell;
mod states;
mod ui;
mod wand;

use game::run_game;

fn main() {
    run_game();
}
