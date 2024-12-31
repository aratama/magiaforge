use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
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

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(RemoteInspectorPlugins);
    }
}
