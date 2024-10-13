use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    // LoadingScreen,
    MainMenu,

    // #[default]をつけてデフォルトに設定したstateが起動時の画面になります
    #[default]
    InGame,
}
