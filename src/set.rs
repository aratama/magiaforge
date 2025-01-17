// https://bevy-cheatbook.github.io/programming/system-sets.html
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedUpdateInGameSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedUpdateGameActiveSet;

/// ゲームメニューが閉じていて、プレイヤーが移動や詠唱などをできる状態のときに実行されるシステムのセットです
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedUpdatePlayerActiveSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedUpdateAfterAll;

pub struct GameSetPlugin;

impl Plugin for GameSetPlugin {
    fn build(&self, app: &mut App) {
        // FixedUpdateでスケジュールされたシステムには、before(PhysicsSet::SyncBackend) でスケジュールをする必要があります
        // これがない場合、変更が正しく rapier に通知されず、数回に一度のような再現性の低いバグが起きることがあるようです
        // https://taintedcoders.com/bevy/physics/rapier

        app.configure_sets(
            FixedUpdate,
            FixedUpdateInGameSet
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );

        app.configure_sets(
            FixedUpdate,
            FixedUpdateGameActiveSet
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend),
        );

        app.configure_sets(
            FixedUpdate,
            FixedUpdatePlayerActiveSet
                .run_if(
                    in_state(GameState::InGame)
                        .and(in_state(TimeState::Active))
                        .and(in_state(GameMenuState::Closed)),
                )
                .before(PhysicsSet::SyncBackend),
        );

        app.configure_sets(
            FixedUpdate,
            FixedUpdateAfterAll
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend)
                .after(FixedUpdateInGameSet)
                .after(FixedUpdateGameActiveSet)
                .after(FixedUpdatePlayerActiveSet),
        );
    }
}
