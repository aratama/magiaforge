use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// なぜか aarch64-apple-darwin でのみビルドに失敗するので macos を除外
// error[E0432]: unresolved import `bevy_remote_inspector`
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "macos")))]
use bevy_remote_inspector::RemoteInspectorPlugins;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_plugins(RapierDebugRenderPlugin {
                enabled: true,
                mode: DebugRenderMode::COLLIDER_SHAPES,
                ..default()
            });

        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "macos")))]
        app.add_plugins(RemoteInspectorPlugins);
    }
}
