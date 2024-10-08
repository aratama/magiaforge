use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingScreen,
    MainMenu,
    InGame,
}
