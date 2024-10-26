// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
// https://qiita.com/LNSEAB/items/6f60da458460274e768d
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actor;
mod asset;
mod audio;
mod bgm;
mod camera;
mod config;
mod constant;
mod entity;
mod game;
mod gamepad;
mod hud;
mod page;
mod serialize;
mod set;
mod states;
mod ui;
mod world;

use game::run_game;

fn main() {
    run_game();
}
