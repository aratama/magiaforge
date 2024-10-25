// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
#![windows_subsystem = "windows"]

mod game;

use game::game::run_game;

fn main() {
    run_game();
}
