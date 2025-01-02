use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

    /// タイトル画面
    MainMenu,

    /// 通常のプレイ画面
    InGame,

    /// 魔法陣でのワープ中を表すステート
    /// いったん画面上のエンティティをすべて削除するため、別ステートに切り替えています
    /// また、オンラインの場合はすぐ次のレベルに行くのではなく名前入力画面に遷移しますが、
    /// その場合の条件分岐もこのステートで行っています
    Warp,

    // 名前入力画面
    // 名前入力が完了したらオンラインアリーナのレベルへ移動します
    NameInput,

    // エンディング画面
    Ending,
    //
    // 画面を追加したら OverlayPlugin や update_pointer_image_by_angle にも変更が必要
}

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MainMenuPhase {
    #[default]
    Active,

    /// スタートボタンを押してフェードアウトしている最中
    Paused,
}

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::InGame)]
pub enum GameMenuState {
    #[default]
    Closed,

    PauseMenuOpen,

    PauseMenuClosing,

    WandEditOpen,
}

#[derive(SubStates, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::InGame)]
pub enum TimeState {
    #[default]
    Active,

    Inactive,
}
