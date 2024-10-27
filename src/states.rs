use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    // #[default]をつけたstateが最初のステートになります
    // どの画面から起動したとしても、カメラの設定は最初に行わなければなりませんが、
    // StartUpスケジュールは OnEnter より後に実行される仕様のため、
    // カメラの初期化を StartUp スケジュールで実行することはできません
    // このため、初期化専用の Setupステートを設けています
    // https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#onenter-state-schedules-now-run-before-startup-schedules
    //
    // また、bevy_asset_loader の読み込み中を表すステートとしても Setup は必要です
    #[default]
    Setup,

    MainMenu,

    Config,

    InGame,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MainMenuPhase {
    #[default]
    Active,

    /// スタートボタンを押してフェードアウトしている最中
    Paused,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::InGame)]
pub enum GameMenuState {
    #[default]
    Close,

    Open,
}
